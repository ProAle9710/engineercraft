use serde::Deserialize;
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
    let msi_url = pick_msi(&rel.assets);
    Ok(Some(Pending {
        version,
        msi_url,
        page_url: rel.html_url,
    }))
}

fn pick_msi(assets: &[GhAsset]) -> Option<String> {
    let url = |pred: fn(&str) -> bool| {
        assets
            .iter()
            .find(|a| pred(&a.name))
            .map(|a| a.browser_download_url.clone())
    };
    url(|n| n.starts_with("EngineerCraft") && n.ends_with(".msi"))
        .or_else(|| url(|n| n.starts_with("JavaCraft") && n.ends_with(".msi")))
        .or_else(|| url(|n| n.ends_with(".msi")))
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
        Some(url) => match download_and_install(url, &pending.version) {
            Ok(()) => Event::Installing,
            Err(e) => Event::Failed(e),
        },
        None => {
            let _ = webbrowser::open(&pending.page_url);
            Event::Done
        }
    }
}

fn download_and_install(url: &str, version: &str) -> Result<(), String> {
    let resp = ureq::get(url)
        .set("User-Agent", &format!("JavaCraft/{version}"))
        .call()
        .map_err(|e| e.to_string())?;
    let path = msi_path(version)?;
    let mut file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    std::io::copy(&mut resp.into_reader(), &mut file).map_err(|e| e.to_string())?;
    drop(file);
    launch_msi(&path)
}

fn msi_path(version: &str) -> Result<PathBuf, String> {
    let dir = std::env::temp_dir();
    Ok(dir.join(format!("EngineerCraft-{version}.msi")))
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
