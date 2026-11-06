use crate::i18n::Lang;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Exercise {
    pub id: String,
    pub number: u32,
    pub title_es: String,
    pub title_en: String,
    pub level: u8,
    pub brief_es: String,
    pub brief_en: String,
    pub body: String,
    pub prefix: String,
    pub suffix: String,
    pub prefix_newlines: usize,
    pub stdout_needles: Vec<String>,
}

pub fn load() -> Result<Vec<Exercise>, String> {
    let root = exercises_dir().ok_or_else(|| {
        "No encontré la carpeta exercises/. Ejecuta desde la raíz del proyecto.".to_string()
    })?;

    let mut out = Vec::new();
    let mut dirs: Vec<PathBuf> = fs::read_dir(&root)
        .map_err(|e| format!("No pude leer {}: {e}", root.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    let println_re =
        Regex::new(r#"System\.out\.println\(\s*"([^"]+)"#).expect("regex println");

    for dir in dirs {
        let name = dir.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !name.starts_with("task-") {
            continue;
        }
        let number: u32 = name
            .trim_start_matches("task-")
            .chars()
            .take(2)
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        if number == 0 {
            continue;
        }

        let starter_path = dir.join("Starter.java");
        let starter = fs::read_to_string(&starter_path).map_err(|e| {
            format!("No pude leer {}: {e}", starter_path.display())
        })?;

        let brief_en_raw = read_first_existing(&dir, &["TASK.md"])
            .unwrap_or_else(|| "_No instructions._".into());
        let brief_es_raw = read_first_existing(&dir, &["TASK-es.md", "TASK.es.md"])
            .unwrap_or_else(|| brief_en_raw.clone());
        let title_en = first_heading(&brief_en_raw)
            .map(clean_title)
            .unwrap_or_else(|| name.to_string());
        let title_es = first_heading(&brief_es_raw)
            .map(clean_title)
            .unwrap_or_else(|| title_en.clone());
        let brief_en = cut_how_to_run(&brief_en_raw);
        let brief_es = cut_how_to_run(&brief_es_raw);

        let stdout_needles: Vec<String> = println_re
            .captures_iter(&starter)
            .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
            .filter(|s| !s.contains("TODO"))
            .collect();

        let level = match number {
            1..=15 => 1,
            16..=31 => 2,
            _ => 3,
        };

        let (prefix, body, suffix) = split_template(&starter);
        let prefix_newlines = prefix.matches('\n').count();

        out.push(Exercise {
            id: format!("{number:02}"),
            number,
            title_es,
            title_en,
            level,
            brief_es,
            brief_en,
            body,
            prefix,
            suffix,
            prefix_newlines,
            stdout_needles,
        });
    }

    if out.is_empty() {
        return Err(format!("No hay ejercicios en {}", root.display()));
    }
    out.sort_by_key(|e| e.number);
    out.insert(0, free_exercise());
    Ok(out)
}

pub const FREE_ID: &str = "free";

pub fn free_exercise() -> Exercise {
    let body = "public class Starter {\n    public static void main(String[] args) {\n        \n    }\n}\n".to_string();
    Exercise {
        id: FREE_ID.into(),
        number: 0,
        title_es: "Práctica libre".into(),
        title_en: "Free practice".into(),
        level: 0,
        brief_es: "# Práctica libre\n\nEscribe Java completo (clase + `main`). CORRER muestra la consola real: `stdout` / errores, sin comparar con una salida esperada.".into(),
        brief_en: "# Free practice\n\nWrite full Java (class + `main`). RUN shows the real console: `stdout` / errors, no expected-output checks.".into(),
        prefix: String::new(),
        suffix: String::new(),
        prefix_newlines: 0,
        stdout_needles: Vec::new(),
        body,
    }
}

pub fn exercises_dir() -> Option<PathBuf> {
    let mut candidates = vec![
        PathBuf::from("exercises"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("exercises"),
    ];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("exercises"));
            candidates.push(parent.join("../exercises"));
        }
    }
    candidates.into_iter().find(|p| p.is_dir())
}

fn read_first_existing(dir: &Path, names: &[&str]) -> Option<String> {
    names
        .iter()
        .map(|n| dir.join(n))
        .find_map(|p| fs::read_to_string(p).ok())
}

fn first_heading(md: &str) -> Option<String> {
    md.lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").trim().to_string())
}

fn clean_title(title: String) -> String {
    let stripped = title
        .split_once("—")
        .or_else(|| title.split_once(" - "))
        .map(|(_, rest)| rest.trim().to_string())
        .unwrap_or(title);
    stripped
        .trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ')
        .to_string()
}

pub fn split_template(src: &str) -> (String, String, String) {
    let Some(main_at) = src.find("void main") else {
        return (String::new(), src.to_string(), String::new());
    };
    let Some(rel) = src[main_at..].find('{') else {
        return (String::new(), src.to_string(), String::new());
    };
    let mut end = main_at + rel + 1;
    if src.as_bytes().get(end) == Some(&b'\n') {
        end += 1;
    }
    let prefix = src[..end].to_string();
    let rest = &src[end..];

    if let Some(marker) = rest.find("Do not change") {
        let cut = rest[..marker]
            .rfind("\n        // ---")
            .or_else(|| rest[..marker].rfind("\n// ---"))
            .map(|i| i + 1)
            .unwrap_or(marker);
        return (prefix, rest[..cut].to_string(), rest[cut..].to_string());
    }

    let mut lines: Vec<&str> = rest.lines().collect();
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    let mut suffix_lines = Vec::new();
    while suffix_lines.len() < 2 && lines.last().is_some_and(|l| l.trim() == "}") {
        suffix_lines.push(lines.pop().unwrap());
    }
    suffix_lines.reverse();
    let mut body = lines.join("\n");
    if !body.ends_with('\n') {
        body.push('\n');
    }
    let mut suffix = suffix_lines.join("\n");
    if !suffix.is_empty() && !suffix.ends_with('\n') {
        suffix.push('\n');
    }
    (prefix, body, suffix)
}

pub fn assemble(ex: &Exercise, body: &str) -> String {
    if ex.is_free() {
        return body.to_string();
    }
    format!("{}{}{}", ex.prefix, glue_body(body), ex.suffix)
}

fn glue_body(body: &str) -> String {
    let mut b = body.to_string();
    while b.ends_with(' ') || b.ends_with('\t') {
        b.pop();
    }
    if !b.ends_with('\n') {
        b.push('\n');
    }
    b
}

/// Extrae el cuerpo sin tocar prefix/suffix originales (toggle Simple/Avanzado).
pub fn body_of(ex: &Exercise, src: &str) -> String {
    if ex.is_free() {
        return src.to_string();
    }
    let src = src.replace("\r\n", "\n").replace('\r', "\n");
    let prefix = ex.prefix.replace("\r\n", "\n");
    let suffix = ex.suffix.replace("\r\n", "\n");
    if let Some(rest) = src.strip_prefix(prefix.as_str()) {
        if let Some(body) = rest.strip_suffix(suffix.as_str()) {
            return body.to_string();
        }
        if let Some(at) = rest.find(suffix.as_str()) {
            return rest[..at].to_string();
        }
        return rest.to_string();
    }
    if src.contains("public class") {
        split_template(&src).1
    } else {
        src.to_string()
    }
}

pub fn body_from_draft(ex: &Exercise, saved: Option<&String>) -> String {
    let saved = saved.filter(|s| !s.trim().is_empty());
    if ex.is_free() {
        return saved.cloned().unwrap_or_else(|| ex.body.clone());
    }
    match saved {
        Some(s) if s.contains("public class") => body_of(ex, s),
        Some(s) => s.clone(),
        None => ex.body.clone(),
    }
}

impl Exercise {
    pub fn is_free(&self) -> bool {
        self.id == FREE_ID
    }

    pub fn title(&self, lang: Lang) -> &str {
        match lang {
            Lang::Es => &self.title_es,
            Lang::En => &self.title_en,
        }
    }

    pub fn brief(&self, lang: Lang) -> &str {
        match lang {
            Lang::Es => &self.brief_es,
            Lang::En => &self.brief_en,
        }
    }

    pub fn level_name(&self, lang: Lang) -> &'static str {
        match (self.level, lang) {
            (0, Lang::Es) => "Libre",
            (0, Lang::En) => "Free",
            (1, Lang::Es) => "Variables y lógica",
            (1, Lang::En) => "Variables and logic",
            (2, Lang::Es) => "Métodos y arreglos",
            (2, Lang::En) => "Methods and arrays",
            (_, Lang::Es) => "Combinar",
            (_, Lang::En) => "Combine",
        }
    }
}

fn cut_how_to_run(md: &str) -> String {
    let cut_at = md
        .find("## Cómo ejecutar")
        .or_else(|| md.find("## How to run"))
        .unwrap_or(md.len());
    let mut text = md[..cut_at].trim().to_string();
    for noise in [
        "Si prefieres inglés, abre `TASK.md`.",
        "If you prefer Spanish, open `TASK-es.md`.",
    ] {
        text = text.replace(noise, "");
    }
    text
}
