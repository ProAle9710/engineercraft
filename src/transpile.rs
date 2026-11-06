use crate::catalog::{self, Exercise};
use crate::codelang::CodeLang;

pub fn adapt(ex: &Exercise, lang: CodeLang) -> Exercise {
    if matches!(lang, CodeLang::Java) {
        return ex.clone();
    }
    if ex.is_free() {
        return free_for(lang);
    }
    let mut out = ex.clone();
    match lang {
        CodeLang::CSharp => {
            out.prefix = cs_prefix(&ex.prefix);
            out.body = to_cs(&ex.body);
            out.suffix = to_cs(&ex.suffix);
        }
        CodeLang::Python => {
            out.prefix = py_prefix(&ex.prefix);
            out.body = to_py(&dedent_main(&ex.body));
            out.suffix = to_py(&dedent_main(&ex.suffix));
        }
        CodeLang::Java => {}
    }
    out.prefix_newlines = out.prefix.matches('\n').count();
    out
}

pub fn adapt_all(src: &[Exercise], lang: CodeLang) -> Vec<Exercise> {
    src.iter().map(|e| adapt(e, lang)).collect()
}

fn free_for(lang: CodeLang) -> Exercise {
    let mut ex = catalog::free_exercise();
    match lang {
        CodeLang::Java => {}
        CodeLang::CSharp => {
            ex.body = "using System;\n\nclass Starter {\n    static void Main(string[] args) {\n        \n    }\n}\n".into();
            ex.brief_es = "# Práctica libre\n\nEscribe C# completo (`Main`). CORRER muestra la consola real.".into();
            ex.brief_en = "# Free practice\n\nWrite full C# (`Main`). RUN shows the real console.".into();
        }
        CodeLang::Python => {
            ex.body = "# práctica libre\n\n".into();
            ex.brief_es = "# Práctica libre\n\nEscribe Python. CORRER muestra la consola real.".into();
            ex.brief_en = "# Free practice\n\nWrite Python. RUN shows the real console.".into();
        }
    }
    ex
}

fn cs_prefix(java_prefix: &str) -> String {
    let mut s = java_prefix.replace("public class Starter", "class Starter");
    s = s.replace(
        "public static void main(String[] args)",
        "static void Main(string[] args)",
    );
    s = to_cs(&s);
    if !s.contains("using System") {
        format!("using System;\n\n{s}")
    } else {
        s
    }
}

fn py_prefix(java_prefix: &str) -> String {
    let mut out = String::new();
    for line in java_prefix.lines() {
        let t = line.trim();
        if t.starts_with("public class") || t.starts_with("public static") || t == "{" {
            break;
        }
        if t.starts_with("//") {
            out.push_str(&line.replace("//", "#"));
            out.push('\n');
        } else if t.is_empty() {
            out.push('\n');
        }
    }
    out
}

fn to_cs(src: &str) -> String {
    src.replace("System.out.println", "Console.WriteLine")
        .replace("System.out.print", "Console.Write")
        .replace("boolean ", "bool ")
        .replace("String ", "string ")
        .replace("String[]", "string[]")
        .replace(".length()", ".Length")
        .replace(".length", ".Length")
        .replace(".equals(", ".Equals(")
        .replace(".charAt(", "[")
}

fn to_py(src: &str) -> String {
    let mut out = String::new();
    for line in src.split_inclusive('\n') {
        let (nl, raw) = if let Some(s) = line.strip_suffix('\n') {
            (true, s)
        } else {
            (false, line)
        };
        let mut l = raw.to_string();
        l = l.replace("System.out.println", "print");
        l = l.replace("System.out.print", "print");
        l = l.replace("//", "#");
        if let Some(rest) = print_args(&l) {
            l = format!(
                "{}print({})",
                indent_of(&l),
                rest.replace(" + ", ", ")
            );
        }
        l = regex_decl(&l);
        l = l.replace("int[] ", "");
        l = l.replace("string[] ", "");
        l = l.replace("String[] ", "");
        l = l.replace("boolean[] ", "");
        l = l.replace("bool[] ", "");
        l = l.replace(" = {", " = [");
        if l.trim_end().ends_with("};") {
            l = l.replacen("};", "]", 1);
        }
        l = l.replace(".length()", "");
        l = l.replace(".Equals(", " == ");
        l = l.replace(".equals(", " == ");
        let t = l.trim_end();
        if t.ends_with(';') && !t.contains("for (") {
            l = format!("{}{}", &t[..t.len() - 1], trailing_ws(&l));
        }
        out.push_str(&l);
        if nl {
            out.push('\n');
        }
    }
    out
}

fn print_args(line: &str) -> Option<String> {
    let t = line.trim_start();
    let rest = t.strip_prefix("print(")?;
    let rest = rest.strip_suffix(';').unwrap_or(rest);
    let rest = rest.strip_suffix(')').unwrap_or(rest);
    Some(rest.to_string())
}

fn indent_of(line: &str) -> String {
    line.chars().take_while(|c| *c == ' ' || *c == '\t').collect()
}

fn trailing_ws(line: &str) -> String {
    line[line.trim_end().len()..].to_string()
}

fn regex_decl(line: &str) -> String {
    let kinds = [
        "String ", "string ", "boolean ", "bool ", "int ", "double ", "char ", "long ", "float ",
    ];
    let trim = line.trim_start();
    for k in kinds {
        if let Some(rest) = trim.strip_prefix(k) {
            if rest.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
                return format!("{}{rest}", indent_of(line));
            }
        }
    }
    line.to_string()
}

fn dedent_main(s: &str) -> String {
    s.lines()
        .map(|l| {
            if let Some(r) = l.strip_prefix("        ") {
                r.to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if s.ends_with('\n') { "\n" } else { "" }
}
