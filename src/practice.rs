pub struct Guide {
    pub code: String,
    pub cursor: usize,
    pub hint: String,
}

pub fn first_write_cursor(code: &str) -> usize {
    if let Some(slot) = slots(code).into_iter().find(|s| !s.filled) {
        return slot.cursor;
    }
    let indent = indent_of(code, guess_lang(code));
    if let Some((start, line)) = first_empty_line(code) {
        return start + leading_ws(line).min(indent.len());
    }
    code.len()
}

pub fn line_index(code: &str, cursor: usize) -> usize {
    code[..cursor.min(code.len())]
        .bytes()
        .filter(|b| *b == b'\n')
        .count()
        + 1
}

pub fn hint_at(
    code: &str,
    cursor: usize,
    lang: crate::i18n::Lang,
    code_lang: crate::codelang::CodeLang,
) -> String {
    let todo = active_todo(code, cursor, code_lang);
    match (lang, code_lang) {
        (crate::i18n::Lang::Es, crate::codelang::CodeLang::Python) => {
            format!("Bloque activo: {todo}  ·  nombre = valor")
        }
        (crate::i18n::Lang::En, crate::codelang::CodeLang::Python) => {
            format!("Active block: {todo}  ·  name = value")
        }
        (crate::i18n::Lang::Es, crate::codelang::CodeLang::CSharp) => {
            format!("Bloque activo: {todo}  ·  tipo + nombre + valor;")
        }
        (crate::i18n::Lang::En, crate::codelang::CodeLang::CSharp) => {
            format!("Active block: {todo}  ·  type + name + value;")
        }
        (crate::i18n::Lang::Es, _) => {
            format!("Bloque activo: {todo}  ·  tipo + nombre + valor;")
        }
        (crate::i18n::Lang::En, _) => {
            format!("Active block: {todo}  ·  type + name + value;")
        }
    }
}

pub fn on_arrive(
    code: &str,
    cursor: usize,
    lang: crate::i18n::Lang,
    code_lang: crate::codelang::CodeLang,
) -> Guide {
    let indent = indent_of(code, code_lang);
    let cursor = cursor.min(code.len());
    let (line_start, line, _col) = line_at(code, cursor);
    let trimmed = line.trim_start();

    if is_guide_comment(trimmed, code_lang) {
        return Guide {
            hint: match lang {
                crate::i18n::Lang::Es => {
                    "Comentario: puedes copiarlo. Para escribir, baja al bloque vacío.".into()
                }
                crate::i18n::Lang::En => {
                    "Comment: you can copy it. To type, go to the empty block below.".into()
                }
            },
            code: code.to_string(),
            cursor,
        };
    }

    if line.trim().is_empty() {
        let (code, cursor) = ensure_indent_at(code, line_start, &indent, code_lang);
        let todo = active_todo(&code, cursor, code_lang);
        return Guide {
            hint: match lang {
                crate::i18n::Lang::Es => format!(
                    "Escribe aquí el código de {todo}. {}",
                    where_hint(code_lang, true)
                ),
                crate::i18n::Lang::En => format!(
                    "Write the {todo} code here. {}",
                    where_hint(code_lang, false)
                ),
            },
            code,
            cursor,
        };
    }

    let col = cursor.saturating_sub(line_start);
    let lead = leading_ws(line);
        if !is_comment(trimmed, code_lang) && col < lead {
        return Guide {
            hint: hint_at(code, cursor, lang, code_lang),
            code: code.to_string(),
            cursor: line_start + lead,
        };
    }

    Guide {
        hint: hint_at(code, cursor, lang, code_lang),
        code: code.to_string(),
        cursor,
    }
}

pub fn after_edit(
    code: &str,
    cursor: usize,
    lang: crate::i18n::Lang,
    code_lang: crate::codelang::CodeLang,
) -> Guide {
    let indent = indent_of(code, code_lang);
    let (line_start, line, _) = line_at(code, cursor.min(code.len()));
    let p = comment_prefix(code_lang);
    if let Some(cmt_at) = line.find(&format!("{p} TODO")).or_else(|| {
        line.find(&format!("{p} Example"))
            .or_else(|| line.find(&format!("{p} ---")))
            .or_else(|| line.find(&format!("{p} Do not")))
            .or_else(|| line.find(&format!("{p} Ejemplo")))
    }) {
        if cmt_at > leading_ws(line) {
            let typed = line[..cmt_at].trim().to_string();
            let comment = line[cmt_at..].to_string();
            let mut restored = String::new();
            restored.push_str(&code[..line_start]);
            restored.push_str(&indent);
            restored.push_str(&comment);
            restored.push_str(&code[line_start + line.len()..]);
            let target = write_cursor_after_comment_block(&restored, line_start);
            let (mut restored, mut cur) =
                ensure_indent_at(&restored, target, &indent, code_lang);
            if !typed.is_empty() {
                restored.insert_str(cur, &typed);
                cur += typed.len();
            }
            return Guide {
                hint: crate::i18n::tr(
                    lang,
                    "Los comentarios TODO no se editan. Tu código fue al bloque de escritura.",
                    "TODO comments are read-only. Your code was moved to the write block.",
                )
                .into(),
                code: restored,
                cursor: cur,
            };
        }
    }
    Guide {
        hint: hint_at(code, cursor.min(code.len()), lang, code_lang),
        code: code.to_string(),
        cursor,
    }
}

struct Slot {
    filled: bool,
    cursor: usize,
}

fn guess_lang(code: &str) -> crate::codelang::CodeLang {
    if code.lines().any(|l| l.trim_start().starts_with("# TODO") || l.trim_start() == "#")
        && !code.contains("public class")
        && !code.contains("static void Main")
    {
        crate::codelang::CodeLang::Python
    } else if code.contains("static void Main") || code.contains("using System") {
        crate::codelang::CodeLang::CSharp
    } else {
        crate::codelang::CodeLang::Java
    }
}

fn comment_prefix(lang: crate::codelang::CodeLang) -> &'static str {
    match lang {
        crate::codelang::CodeLang::Python => "#",
        _ => "//",
    }
}

fn is_comment(trimmed: &str, lang: crate::codelang::CodeLang) -> bool {
    match lang {
        crate::codelang::CodeLang::Python => trimmed.starts_with('#'),
        _ => trimmed.starts_with("//"),
    }
}

fn where_hint(lang: crate::codelang::CodeLang, es: bool) -> &'static str {
    match (lang, es) {
        (crate::codelang::CodeLang::Python, true) => "Va al nivel del archivo, sin clase.",
        (crate::codelang::CodeLang::Python, false) => "It belongs at file level, no class.",
        (crate::codelang::CodeLang::CSharp, true) => "Respeta la indentación: va dentro de Main.",
        (crate::codelang::CodeLang::CSharp, false) => "Keep the indent: it belongs inside Main.",
        (_, true) => "Respeta la indentación: va dentro de main.",
        (_, false) => "Keep the indent: it belongs inside main.",
    }
}

fn is_todo_line(trimmed: &str) -> bool {
    trimmed.starts_with("// TODO") || trimmed.starts_with("# TODO")
}

fn slots(code: &str) -> Vec<Slot> {
    let indent = indent_of(code, crate::codelang::CodeLang::Java);
    let mut out = Vec::new();
    let spans = line_spans(code);
    let mut i = 0;
    while i < spans.len() {
        let (_, _, line) = spans[i];
        if is_todo_line(line.trim_start()) {
            let mut j = i + 1;
            while j < spans.len() && is_any_comment(spans[j].2.trim_start()) {
                j += 1;
            }
            let mut filled = false;
            let mut cursor = spans.get(j).map(|(s, _, _)| *s).unwrap_or(code.len());
            let mut k = j;
            while k < spans.len() && !is_todo_line(spans[k].2.trim_start()) {
                let t = spans[k].2.trim();
                if !t.is_empty() && !is_any_comment(t) {
                    filled = true;
                }
                if spans[k].2.trim().is_empty() && !filled {
                    cursor = spans[k].0 + indent.len().min(spans[k].2.len());
                }
                k += 1;
            }
            out.push(Slot { filled, cursor });
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

fn is_any_comment(trimmed: &str) -> bool {
    trimmed.starts_with("//") || trimmed.starts_with('#')
}

fn indent_of(code: &str, lang: crate::codelang::CodeLang) -> String {
    for line in code.lines() {
        if is_todo_line(line.trim_start()) {
            return " ".repeat(leading_ws(line));
        }
    }
    match lang {
        crate::codelang::CodeLang::Python => "    ".into(),
        _ => "        ".into(),
    }
}

fn is_guide_comment(trimmed: &str, lang: crate::codelang::CodeLang) -> bool {
    let p = comment_prefix(lang);
    let t = trimmed.trim_start_matches(p).trim_start();
    t.starts_with("TODO")
        || t.starts_with("Example")
        || t.starts_with("---")
        || t.starts_with("Do not")
        || t.starts_with("YOUR TURN")
        || t.starts_with("TU TURNO")
        || t.starts_with("Look at")
        || t.starts_with("Mira")
        || t.starts_with("Use ")
        || t.starts_with("Ejemplo")
        || t.starts_with("Correct syntax")
        || t.starts_with("Sintaxis")
}

fn active_todo(code: &str, cursor: usize, lang: crate::codelang::CodeLang) -> String {
    let p = comment_prefix(lang);
    let mut last = "TODO".to_string();
    let mut offset = 0;
    for line in code.lines() {
        let t = line.trim_start();
        if t.starts_with(&format!("{p} TODO")) {
            last = t
                .trim_start_matches(p)
                .trim_start()
                .split(':')
                .next()
                .unwrap_or("TODO")
                .to_string();
        }
        offset += line.len() + 1;
        if offset > cursor {
            break;
        }
    }
    last
}

fn write_cursor_after_comment_block(code: &str, comment_line_start: usize) -> usize {
    let spans = line_spans(code);
    let Some(idx) = spans.iter().position(|(s, _, _)| *s == comment_line_start) else {
        return comment_line_start;
    };
    let mut j = idx;
    while j + 1 < spans.len() && is_any_comment(spans[j + 1].2.trim_start()) {
        j += 1;
    }
    if j + 1 < spans.len() {
        spans[j + 1].0
    } else {
        code.len()
    }
}

fn ensure_indent_at(
    code: &str,
    at: usize,
    indent: &str,
    lang: crate::codelang::CodeLang,
) -> (String, usize) {
    let at = at.min(code.len());
    let (line_start, line, _) = line_at(code, at);
    if !line.trim().is_empty() && !is_comment(line.trim_start(), lang) {
        return (code.to_string(), at.max(line_start + leading_ws(line)));
    }
    let rest = &code[line_start + line.len()..];
    let mut next = String::new();
    next.push_str(&code[..line_start]);
    next.push_str(indent);
    if rest.is_empty() {
        next.push('\n');
    } else {
        next.push_str(rest);
    }
    let cursor = line_start + indent.len();
    (next, cursor)
}

fn first_empty_line(code: &str) -> Option<(usize, &str)> {
    line_spans(code)
        .into_iter()
        .find(|(_, _, l)| l.trim().is_empty())
        .map(|(s, _, l)| (s, l))
}

fn line_at(code: &str, cursor: usize) -> (usize, &str, usize) {
    let cursor = cursor.min(code.len());
    let start = code[..cursor].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end = code[cursor..]
        .find('\n')
        .map(|i| cursor + i)
        .unwrap_or(code.len());
    let line = &code[start..end];
    let col = cursor.saturating_sub(start);
    (start, line, col)
}

fn line_spans(code: &str) -> Vec<(usize, usize, &str)> {
    let mut out = Vec::new();
    let mut start = 0;
    for (i, ch) in code.char_indices() {
        if ch == '\n' {
            out.push((start, i, &code[start..i]));
            start = i + 1;
        }
    }
    out.push((start, code.len(), &code[start..]));
    out
}

fn leading_ws(line: &str) -> usize {
    line.chars().take_while(|c| *c == ' ' || *c == '\t').count()
}
