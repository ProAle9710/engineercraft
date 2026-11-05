use crate::theme::{hair, ACCENT, DEBUG_BG, DIM, ERR, ERR_BG, MUTED, TEXT, WARN};
use eframe::egui::text::{LayoutJob, TextFormat};
use eframe::egui::{Color32, FontId, Stroke};
use std::collections::HashSet;

const KW: Color32 = Color32::from_rgb(200, 184, 154);
const TY: Color32 = Color32::from_rgb(168, 191, 168);
const STR: Color32 = Color32::from_rgb(214, 186, 140);
const NUM: Color32 = Color32::from_rgb(184, 160, 144);
const CM: Color32 = Color32::from_rgb(107, 103, 94);
const TODO: Color32 = Color32::from_rgb(224, 122, 95);

pub fn java(
    code: &str,
    error_lines: &HashSet<usize>,
    debug_line: Option<usize>,
    font: FontId,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = f32::INFINITY;

    let mut line_no = 1usize;
    let mut rest = code;
    while !rest.is_empty() {
        let (line, next) = split_line(rest);
        rest = next;
        let bad = error_lines.contains(&line_no);
        let dbg = debug_line == Some(line_no);
        let bg = if bad {
            ERR_BG
        } else if dbg {
            DEBUG_BG
        } else {
            Color32::TRANSPARENT
        };
        let underline = if bad {
            hair(ERR)
        } else if dbg {
            Stroke::new(2.5_f32, WARN)
        } else {
            Stroke::NONE
        };
        tokenize_line(line, bg, underline, font.clone(), &mut job);
        if !rest.is_empty() || code.ends_with('\n') {
            // keep going
        }
        line_no += 1;
        if rest.is_empty() {
            break;
        }
    }
    if code.is_empty() {
        job.append(
            " ",
            0.0,
            TextFormat {
                font_id: font,
                color: DIM,
                ..Default::default()
            },
        );
    }
    job
}

fn split_line(s: &str) -> (&str, &str) {
    match s.find('\n') {
        Some(i) => (&s[..=i], &s[i + 1..]),
        None => (s, ""),
    }
}

fn tokenize_line(line: &str, bg: Color32, underline: Stroke, font: FontId, job: &mut LayoutJob) {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            let comment: String = chars[i..].iter().collect();
            let color = if comment.contains("TODO") { TODO } else { CM };
            append(job, &comment, color, bg, underline, &font);
            break;
        }
        if ch == '"' {
            let start = i;
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 2;
                    continue;
                }
                if chars[i] == '"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            append(job, &s, STR, bg, underline, &font);
            continue;
        }
        if ch == '\'' {
            let start = i;
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 2;
                    continue;
                }
                if chars[i] == '\'' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            append(job, &s, STR, bg, underline, &font);
            continue;
        }
        if ch.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            append(job, &s, NUM, bg, underline, &font);
            continue;
        }
        if is_ident_start(ch) {
            let start = i;
            i += 1;
            while i < chars.len() && is_ident(chars[i]) {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            let color = if is_keyword(&s) {
                KW
            } else if is_type(&s) {
                TY
            } else if s == "TODO" {
                TODO
            } else if s == "System" || s == "out" {
                ACCENT
            } else {
                TEXT
            };
            append(job, &s, color, bg, underline, &font);
            continue;
        }
        let color = if "{}()[]".contains(ch) {
            WARN
        } else if ch == '\n' {
            TEXT
        } else {
            MUTED
        };
        append(job, &ch.to_string(), color, bg, underline, &font);
        i += 1;
    }
}

fn append(job: &mut LayoutJob, text: &str, color: Color32, bg: Color32, underline: Stroke, font: &FontId) {
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: font.clone(),
            color,
            background: bg,
            underline,
            ..Default::default()
        },
    );
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "public"
            | "class"
            | "static"
            | "void"
            | "if"
            | "else"
            | "for"
            | "while"
            | "return"
            | "new"
            | "true"
            | "false"
            | "null"
            | "break"
            | "continue"
            | "package"
            | "import"
            | "this"
            | "final"
            | "try"
            | "catch"
    )
}

fn is_type(s: &str) -> bool {
    matches!(
        s,
        "String" | "int" | "boolean" | "double" | "char" | "byte" | "long" | "float" | "short" | "Math"
    )
}
