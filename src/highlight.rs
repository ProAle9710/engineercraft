use crate::codelang::CodeLang;
use crate::theme::{hair, pal, Palette};
use eframe::egui::text::{LayoutJob, TextFormat};
use eframe::egui::{Color32, FontId, Stroke};
use std::collections::HashSet;

pub fn java(
    code: &str,
    error_lines: &HashSet<usize>,
    debug_line: Option<usize>,
    font: FontId,
    lang: CodeLang,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = f32::INFINITY;
    let p = pal();

    let mut line_no = 1usize;
    let mut rest = code;
    while !rest.is_empty() {
        let (line, next) = split_line(rest);
        rest = next;
        let bad = error_lines.contains(&line_no);
        let dbg = debug_line == Some(line_no);
        let (syn, bg, underline) = if bad {
            let o = p.opposite();
            (o, o.bg, hair(o.err))
        } else if dbg {
            (p, p.debug_bg, Stroke::new(2.5_f32, p.warn))
        } else {
            (p, Color32::TRANSPARENT, Stroke::NONE)
        };
        tokenize_line(line, syn, bg, underline, font.clone(), lang, &mut job);
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
                color: pal().dim,
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

fn tokenize_line(
    line: &str,
    p: Palette,
    bg: Color32,
    underline: Stroke,
    font: FontId,
    lang: CodeLang,
    job: &mut LayoutJob,
) {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if lang == CodeLang::Python && ch == '#' {
            let comment: String = chars[i..].iter().collect();
            let color = if comment.contains("TODO") { p.todo } else { p.cm };
            append(job, &comment, color, bg, underline, &font);
            break;
        }
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            let comment: String = chars[i..].iter().collect();
            let color = if comment.contains("TODO") { p.todo } else { p.cm };
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
            append(job, &s, p.str_, bg, underline, &font);
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
            append(job, &s, p.str_, bg, underline, &font);
            continue;
        }
        if ch.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            append(job, &s, p.num, bg, underline, &font);
            continue;
        }
        if is_ident_start(ch) {
            let start = i;
            i += 1;
            while i < chars.len() && is_ident(chars[i]) {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            let color = if is_keyword(&s, lang) {
                p.kw
            } else if is_type(&s, lang) {
                p.ty
            } else if s == "TODO" {
                p.todo
            } else if s == "System" || s == "out" {
                p.accent
            } else {
                p.text
            };
            append(job, &s, color, bg, underline, &font);
            continue;
        }
        let color = if "{}()[]".contains(ch) {
            p.warn
        } else if ch == '\n' {
            p.text
        } else {
            p.muted
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

fn is_keyword(s: &str, lang: CodeLang) -> bool {
    match lang {
        CodeLang::Python => matches!(
            s,
            "if" | "else" | "elif" | "for" | "while" | "return" | "def" | "class" | "import"
                | "from" | "True" | "False" | "None" | "break" | "continue" | "in" | "not" | "and"
                | "or" | "pass" | "lambda" | "try" | "except"
        ),
        CodeLang::CSharp => matches!(
            s,
            "public" | "class" | "static" | "void" | "if" | "else" | "for" | "while" | "return"
                | "new" | "true" | "false" | "null" | "break" | "continue" | "using" | "namespace"
                | "this" | "var" | "try" | "catch"
        ),
        CodeLang::Java => matches!(
            s,
            "public" | "class" | "static" | "void" | "if" | "else" | "for" | "while" | "return"
                | "new" | "true" | "false" | "null" | "break" | "continue" | "package" | "import"
                | "this" | "final" | "try" | "catch"
        ),
    }
}

fn is_type(s: &str, lang: CodeLang) -> bool {
    match lang {
        CodeLang::Python => false,
        CodeLang::CSharp => matches!(
            s,
            "string" | "int" | "bool" | "double" | "char" | "byte" | "long" | "float" | "short"
                | "Console" | "Math"
        ),
        CodeLang::Java => matches!(
            s,
            "String" | "int" | "boolean" | "double" | "char" | "byte" | "long" | "float" | "short"
                | "Math"
        ),
    }
}
