use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Progress {
    pub completed: HashSet<String>,
    pub drafts: HashMap<String, String>,
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub full_file: bool,
    #[serde(default = "default_dark")]
    pub dark: bool,
    #[serde(default)]
    pub last_seen_version: String,
    #[serde(default)]
    pub code_lang: String,
    /// 0 = automático según resolución.
    #[serde(default)]
    pub ui_zoom: f32,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
}

fn default_dark() -> bool {
    true
}

fn default_font_size() -> f32 {
    13.5
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            completed: HashSet::new(),
            drafts: HashMap::new(),
            lang: String::new(),
            full_file: false,
            dark: true,
            last_seen_version: String::new(),
            code_lang: String::new(),
            ui_zoom: 0.0,
            font_size: default_font_size(),
        }
    }
}

impl Progress {
    pub fn load() -> Self {
        let path = file_path();
        match fs::read_to_string(&path) {
            Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let path = file_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(raw) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&path, raw);
            lock_private(&path);
        }
    }

    pub fn mark_complete(&mut self, id: &str) {
        self.completed.insert(id.to_string());
    }

    pub fn is_complete(&self, lang: crate::codelang::CodeLang, id: &str) -> bool {
        self.completed_ids(lang).contains(id)
    }

    pub fn completed_ids(&self, lang: crate::codelang::CodeLang) -> HashSet<String> {
        let prefix = format!("{}:", lang.id());
        let mut ids = HashSet::new();
        for k in &self.completed {
            if let Some(id) = k.strip_prefix(&prefix) {
                if id != "free" {
                    ids.insert(id.to_string());
                }
            } else if lang == crate::codelang::CodeLang::Java
                && !k.contains(':')
                && k != "free"
            {
                ids.insert(k.clone());
            }
        }
        ids
    }

    pub fn complete_count(&self, lang: crate::codelang::CodeLang) -> usize {
        self.completed_ids(lang).len()
    }
}

fn file_path() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    let neu = base.join("engineercraft");
    let old = base.join("javacraft").join("progress.json");
    let dest = neu.join("progress.json");
    if !dest.exists() && old.exists() {
        let _ = fs::create_dir_all(&neu);
        let _ = fs::copy(&old, &dest);
    }
    let _ = fs::create_dir_all(&neu);
    lock_private(&neu);
    if dest.exists() {
        lock_private(&dest);
    }
    dest
}

fn lock_private(path: &Path) {
    #[cfg(windows)]
    {
        let Some(s) = path.to_str() else {
            return;
        };
        let Ok(user) = std::env::var("USERNAME") else {
            return;
        };
        if user.is_empty() {
            return;
        };
        let mut cmd = Command::new("icacls");
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x0800_0000);
        }
        let grant = format!("{user}:(F)");
        let _ = cmd
            .args([s, "/inheritance:r", "/grant:r", &grant])
            .output();
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
    }
}
