use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::PathBuf;

pub const REPO_URL: &str = "https://github.com/ProAle9710/engineercraft";
const API_LATEST: &[&str] = &[
    "https://api.github.com/repos/ProAle9710/engineercraft/releases/latest",
    "https://api.github.com/repos/ProAle9710/javacraft/releases/latest",
];

#[derive(Clone)]
pub struct Pending {
    pub version: String,
    pub msi_url: Option<String>,
    pub msi_sha256: Option<String>,
    pub page_url: String,
}

#[derive(Deserialize)]
struct GhRelease {
    tag_name: String,
    html_url: String,
    assets: Vec<GhAsset>,
}

#[derive(Deserialize)]
struct GhAsset {
    name: String,
    browser_download_url: String,
    #[serde(default)]
    digest: Option<String>,
}

pub enum Event {
    Available(Pending),
    UpToDate,
    Failed(String),
    Installing,
    Done,
}

pub fn check_latest(current: &str) -> Event {
    match fetch(current) {
        Ok(Some(p)) => Event::Available(p),
        Ok(None) => Event::UpToDate,
        Err(e) => Event::Failed(e),
    }
}

fn fetch(current: &str) -> Result<Option<Pending>, String> {
    let mut best: Option<Pending> = None;
    let mut last_err: Option<String> = None;
    let mut ok = false;
    for url in API_LATEST {
        match fetch_one(url, current) {
            Ok(Some(p)) => {
                ok = true;
                if best
                    .as_ref()
                    .map(|b| newer(&p.version, &b.version))
                    .unwrap_or(true)
                {
                    best = Some(p);
                }
            }
            Ok(None) => ok = true,
            Err(e) => last_err = Some(e),
        }
    }
    if best.is_some() {
        return Ok(best);
    }
    if ok {
        return Ok(None);
    }
    Err(last_err.unwrap_or_else(|| "sin releases".into()))
}

fn fetch_one(url: &str, current: &str) -> Result<Option<Pending>, String> {
    let rel: GhRelease = ureq::get(url)
        .set("User-Agent", &format!("JavaCraft/{current}"))
        .set("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| e.to_string())?
        .into_json()
        .map_err(|e| e.to_string())?;

    let version = rel.tag_name.trim_start_matches('v').to_string();
    if !newer(&version, current) {
        return Ok(None);
    }
    if !rel.html_url.starts_with("https://github.com/ProAle9710/") {
        return Err("origen de release no permitido".into());
    }
    let msi = pick_msi(&rel.assets);
    Ok(Some(Pending {
        version,
        msi_url: msi.as_ref().map(|a| a.browser_download_url.clone()),
        msi_sha256: msi.and_then(|a| sha_from_digest(a.digest.as_deref())),
        page_url: rel.html_url,
    }))
}

fn allowed_msi_url(url: &str) -> bool {
    let https = url.starts_with("https://");
    let host = url.starts_with("https://github.com/ProAle9710/")
        || url.starts_with("https://objects.githubusercontent.com/")
        || url.starts_with("https://release-assets.githubusercontent.com/");
    https && host && !url.contains("..")
}

fn allowed_msi_name(name: &str) -> bool {
    let n = name.rsplit('/').next().unwrap_or(name);
    (n.starts_with("EngineerCraft-") || n.starts_with("JavaCraft-"))
        && n.ends_with(".msi")
        && !n.contains("..")
        && !n.contains('\\')
}

fn sha_from_digest(d: Option<&str>) -> Option<String> {
    let d = d?.trim();
    let hex = d.strip_prefix("sha256:")?;
    if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(hex.to_ascii_lowercase())
    } else {
        None
    }
}

fn pick_msi(assets: &[GhAsset]) -> Option<&GhAsset> {
    assets.iter().find(|a| {
        allowed_msi_name(&a.name) && allowed_msi_url(&a.browser_download_url)
    })
}

fn newer(remote: &str, local: &str) -> bool {
    parse(remote) > parse(local)
}

fn parse(v: &str) -> (u32, u32, u32) {
    let mut it = v.split('.');
    let n = |s: Option<&str>| s.and_then(|x| x.parse().ok()).unwrap_or(0);
    (n(it.next()), n(it.next()), n(it.next()))
}

pub fn apply(pending: &Pending) -> Event {
    match &pending.msi_url {
        Some(url) => match download_and_install(url, &pending.version, pending.msi_sha256.as_deref()) {
            Ok(()) => Event::Installing,
            Err(e) => Event::Failed(e),
        },
        None => {
            if pending.page_url.starts_with("https://github.com/ProAle9710/") {
                let _ = webbrowser::open(&pending.page_url);
            }
            Event::Done
        }
    }
}

fn download_and_install(url: &str, version: &str, expect_sha: Option<&str>) -> Result<(), String> {
    if !allowed_msi_url(url) {
        return Err("URL de MSI no permitida".into());
    }
    let expect = expect_sha.ok_or_else(|| "el release no trae sha256".to_string())?;
    let resp = ureq::get(url)
        .set("User-Agent", &format!("EngineerCraft/{version}"))
        .call()
        .map_err(|e| e.to_string())?;
    let mut body = Vec::new();
    resp.into_reader()
        .read_to_end(&mut body)
        .map_err(|e| e.to_string())?;
    const OLE: &[u8] = b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1";
    if body.len() < 8 || &body[..8] != OLE {
        return Err("el archivo no es un MSI".into());
    }
    let got = hex_sha256(&body);
    if got != expect {
        return Err("hash SHA-256 del MSI no coincide".into());
    }
    let path = msi_path(version)?;
    std::fs::write(&path, &body).map_err(|e| e.to_string())?;
    reject_bad_authenticode(&path)?;
    launch_msi(&path)
}

fn hex_sha256(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn msi_path(version: &str) -> Result<PathBuf, String> {
    let ver: String = version
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-')
        .collect();
    if ver.is_empty() {
        return Err("versión inválida".into());
    }
    Ok(std::env::temp_dir().join(format!("EngineerCraft-{ver}-{}.msi", std::process::id())))
}

#[cfg(windows)]
fn reject_bad_authenticode(path: &std::path::Path) -> Result<(), String> {
    let Some(s) = path.to_str() else {
        return Ok(());
    };
    let mut cmd = std::process::Command::new("powershell");
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    let out = cmd
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("(Get-AuthenticodeSignature -FilePath '{}').Status", s.replace('\'', "''")),
        ])
        .output()
        .map_err(|e| e.to_string())?;
    let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if status == "HashMismatch" || status == "UnknownError" {
        return Err(format!("firma Authenticode inválida: {status}"));
    }
    Ok(())
}

#[cfg(not(windows))]
fn reject_bad_authenticode(_path: &std::path::Path) -> Result<(), String> {
    Ok(())
}

#[cfg(windows)]
fn launch_msi(path: &std::path::Path) -> Result<(), String> {
    std::process::Command::new("msiexec")
        .arg("/i")
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(not(windows))]
fn launch_msi(_path: &std::path::Path) -> Result<(), String> {
    Err("MSI solo en Windows".into())
}
