use regex::Regex;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Completion {
    pub label: String,
    pub insert: String,
    pub detail: String,
}

pub fn matching(prefix: &str, code: &str) -> Vec<Completion> {
    suggest(prefix, code, false)
}

pub fn matching_forced(prefix: &str, code: &str) -> Vec<Completion> {
    suggest(prefix, code, true)
}

fn suggest(prefix: &str, code: &str, forced: bool) -> Vec<Completion> {
    let p = prefix.to_ascii_lowercase();
    if !forced && p.len() < 1 {
        return Vec::new();
    }

    let mut hits: Vec<(i32, Completion)> = Vec::new();
    let mut seen = HashSet::new();

    for (name, kind) in declared_names(code) {
        if name == prefix {
            continue;
        }
        let l = name.to_ascii_lowercase();
        let ok = forced && p.is_empty()
            || l.starts_with(&p)
            || (!p.is_empty() && l.contains(&p));
        if ok && seen.insert(name.clone()) {
            let rank = if l.starts_with(&p) { 0 } else { 1 };
            hits.push((rank, c(&name, &name, &kind)));
        }
    }

    if forced || p.len() >= 2 {
        for item in snippets() {
            let l = item.label.to_ascii_lowercase();
            let ok = forced && p.is_empty()
                || l.starts_with(&p)
                || l.contains(&p);
            if ok && seen.insert(item.label.clone()) {
                let rank = if l.starts_with(&p) { 2 } else { 3 };
                hits.push((rank, item));
            }
        }
    }

    hits.sort_by_key(|(rank, item)| (*rank, item.label.len()));
    hits.into_iter().map(|(_, c)| c).take(10).collect()
}

pub fn declared_names(code: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let decl = Regex::new(
        r"\b(String|int|boolean|double|char|long|float|short)(\s*\[\s*\])?\s+([A-Za-z_][A-Za-z0-9_]*)",
    )
    .expect("decl regex");
    for line in code.lines() {
        let vis = line.find("//").map(|i| &line[..i]).unwrap_or(line);
        for cap in decl.captures_iter(vis) {
            let ty = cap.get(1).map(|m| m.as_str()).unwrap_or("var");
            let arr = cap.get(2).map(|_| "[]").unwrap_or("");
            let name = cap.get(3).unwrap().as_str();
            if name == "variable" || name == "args" {
                continue;
            }
            if !seen.insert(name.to_string()) {
                continue;
            }
            out.push((name.to_string(), format!("{ty}{arr}")));
        }
    }
    out
}

pub fn declared_before(code: &str, before_line: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let decl = Regex::new(
        r"\b(String|int|boolean|double|char|long|float|short)(\s*\[\s*\])?\s+([A-Za-z_][A-Za-z0-9_]*)",
    )
    .expect("decl regex");
    for (i, line) in code.lines().enumerate() {
        if i + 1 >= before_line {
            continue;
        }
        let vis = line.find("//").map(|i| &line[..i]).unwrap_or(line);
        for cap in decl.captures_iter(vis) {
            let name = cap.get(3).unwrap().as_str();
            if name == "variable" || name == "args" {
                continue;
            }
            if seen.insert(name.to_string()) {
                out.push(name.to_string());
            }
        }
    }
    out
}

pub fn current_prefix(code: &str, cursor_bytes: usize) -> (usize, String) {
    let cursor = cursor_bytes.min(code.len());
    let before = &code[..cursor];
    let start = before
        .char_indices()
        .rev()
        .find(|(_, ch)| !(ch.is_alphanumeric() || *ch == '_'))
        .map(|(i, ch)| i + ch.len_utf8())
        .unwrap_or(0);
    (start, before[start..].to_string())
}

pub fn apply(code: &str, start: usize, cursor: usize, item: &Completion) -> (String, usize) {
    let cursor = cursor.min(code.len());
    let start = start.min(cursor);
    let mut insert = item.insert.clone();
    let caret = insert.find('|').unwrap_or(insert.len());
    insert = insert.replace('|', "");
    let mut out = String::new();
    out.push_str(&code[..start]);
    out.push_str(&insert);
    out.push_str(&code[cursor..]);
    let new_cursor = start + caret;
    (out, new_cursor)
}

fn snippets() -> Vec<Completion> {
    let mut items = vec![
        c("sout", "System.out.println(|);", "imprimir"),
        c("fori", "for (int i = 0; i < |.length; i++) {\n            \n        }", "ciclo for"),
        c("while", "while (|) {\n            \n        }", "ciclo while"),
        c("if", "if (|) {\n            \n        }", "condicional"),
        c("ife", "if (|) {\n            \n        } else {\n            \n        }", "if / else"),
        c("eq", ".equals(|)", "comparar texto"),
        c("charat", ".charAt(|)", "carácter"),
        c("len", ".length", "largo arreglo"),
        c("lent", ".length()", "largo String"),
    ];
    for kw in [
        "String", "int", "boolean", "double", "char", "true", "false",
        "null", "new", "else", "for", "while", "if", "return", "System",
    ] {
        items.push(c(kw, kw, "Java"));
    }
    items
}

fn c(label: &str, insert: &str, detail: &str) -> Completion {
    Completion {
        label: label.into(),
        insert: insert.into(),
        detail: detail.into(),
    }
}
