use crate::codelang::CodeLang;

pub fn autoclose(code: &str, cursor: usize, lang: CodeLang) -> Option<(String, usize)> {
    if cursor == 0 {
        return None;
    }
    let cursor = cursor.min(code.len());
    if in_line_comment(code, cursor, lang) {
        return None;
    }
    let ch = code[..cursor].chars().last()?;
    let after = code[cursor..].chars().next();

    if is_closer(ch, lang) && after == Some(ch) {
        let mut out = String::new();
        out.push_str(&code[..cursor]);
        let rest = &code[cursor..];
        let skip = ch.len_utf8();
        if rest.len() >= skip {
            out.push_str(&rest[skip..]);
        }
        return Some((out, cursor));
    }

    if ch == '"' || (ch == '\'' && lang != CodeLang::Python) {
        if !is_opening_quote(code, cursor, ch) {
            return None;
        }
        if after == Some(ch) {
            return None;
        }
        let mut out = String::new();
        out.push_str(&code[..cursor]);
        out.push(ch);
        if uses_semicolon(lang)
            && needs_semicolon(&code[..cursor.saturating_sub(ch.len_utf8())], &code[cursor..])
        {
            out.push(';');
        }
        out.push_str(&code[cursor..]);
        return Some((out, cursor));
    }

    if lang == CodeLang::Python && ch == '\'' {
        if !is_opening_quote(code, cursor, ch) || after == Some(ch) {
            return None;
        }
        let mut out = String::new();
        out.push_str(&code[..cursor]);
        out.push(ch);
        out.push_str(&code[cursor..]);
        return Some((out, cursor));
    }

    if ch == '{' && uses_braces(lang) && is_block_header(&code[..cursor.saturating_sub(1)], lang)
    {
        let indent = current_indent(code, cursor.saturating_sub(1));
        let inner = format!("{indent}        ");
        let mut out = String::new();
        out.push_str(&code[..cursor]);
        out.push('\n');
        out.push_str(&inner);
        let caret = out.len();
        out.push('\n');
        out.push_str(&indent);
        out.push('}');
        out.push_str(&code[cursor..]);
        return Some((out, caret));
    }

    if lang == CodeLang::Python && ch == ':' && is_py_header(&code[..cursor.saturating_sub(1)]) {
        let indent = current_indent(code, cursor.saturating_sub(1));
        let inner = format!("{indent}    ");
        let rest = &code[cursor..];
        if rest.starts_with('\n') {
            return None;
        }
        let mut out = String::new();
        out.push_str(&code[..cursor]);
        out.push('\n');
        out.push_str(&inner);
        let caret = out.len();
        out.push_str(rest);
        return Some((out, caret));
    }

    let closer = match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => return None,
    };
    if after == Some(closer) {
        return None;
    }
    let mut out = String::new();
    out.push_str(&code[..cursor]);
    out.push(closer);
    out.push_str(&code[cursor..]);
    Some((out, cursor))
}

fn uses_semicolon(lang: CodeLang) -> bool {
    matches!(lang, CodeLang::Java | CodeLang::CSharp)
}

fn uses_braces(lang: CodeLang) -> bool {
    matches!(lang, CodeLang::Java | CodeLang::CSharp)
}

fn is_closer(ch: char, lang: CodeLang) -> bool {
    match lang {
        CodeLang::Python => matches!(ch, ')' | ']' | '}' | '"' | '\''),
        _ => matches!(ch, ')' | ']' | '}' | '"' | '\''),
    }
}

fn in_line_comment(code: &str, cursor: usize, lang: CodeLang) -> bool {
    let line = code[..cursor].rsplit('\n').next().unwrap_or("");
    let mark = match lang {
        CodeLang::Python => "#",
        _ => "//",
    };
    match line.find(mark) {
        Some(i) => i + mark.len() <= line.len(),
        None => false,
    }
}

fn is_opening_quote(code: &str, cursor: usize, q: char) -> bool {
    let before = &code[..cursor.saturating_sub(q.len_utf8())];
    if before.ends_with('\\') {
        return false;
    }
    let line = before.rsplit('\n').next().unwrap_or(before);
    let n = line.chars().filter(|c| *c == q).count();
    n % 2 == 0
}

fn needs_semicolon(before: &str, after: &str) -> bool {
    let line_before = before.rsplit('\n').next().unwrap_or(before).trim_end();
    if !line_before.ends_with('=') {
        return false;
    }
    let tail = after.split('\n').next().unwrap_or(after);
    !line_before.contains(';') && !tail.contains(';')
}

fn is_block_header(before: &str, lang: CodeLang) -> bool {
    let line = before.rsplit('\n').next().unwrap_or(before).trim_end();
    let core = line.trim_end_matches('{').trim_end();
    let kw = match lang {
        CodeLang::CSharp => core.starts_with("if")
            || core.starts_with("for")
            || core.starts_with("while")
            || core.starts_with("else")
            || core.starts_with("do")
            || core.starts_with("switch")
            || core.starts_with("foreach")
            || core.starts_with("using")
            || core.starts_with("try")
            || core.starts_with("catch"),
        _ => {
            core.starts_with("if")
                || core.starts_with("for")
                || core.starts_with("while")
                || core.starts_with("else")
                || core.starts_with("do")
                || core.starts_with("switch")
                || core.starts_with("try")
                || core.starts_with("catch")
        }
    };
    kw || core.ends_with(')')
}

fn is_py_header(before: &str) -> bool {
    let line = before.rsplit('\n').next().unwrap_or(before).trim_end();
    let core = line.trim_end_matches(':').trim_end();
    core.starts_with("if ")
        || core.starts_with("elif ")
        || core == "else"
        || core.starts_with("for ")
        || core.starts_with("while ")
        || core.starts_with("def ")
        || core.starts_with("try")
        || core.starts_with("except")
}

fn current_indent(code: &str, at: usize) -> String {
    let line = code[..at.min(code.len())].rsplit('\n').next().unwrap_or("");
    line.chars()
        .take_while(|c| *c == ' ' || *c == '\t')
        .collect()
}
