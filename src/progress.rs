use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Progress {
    pub completed: HashSet<String>,
    pub drafts: HashMap<String, String>,
    #[serde(default)]
    pub lang: String,
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
            let _ = fs::write(path, raw);
        }
    }

    pub fn mark_complete(&mut self, id: &str) {
        self.completed.insert(id.to_string());
    }
}

fn file_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("javacraft")
        .join("progress.json")
}
