use crate::i18n::Lang;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Completion {
    pub label: String,
    pub insert: String,
    pub detail: String,
    pub doc: String,
}

pub struct Prefix {
    pub start: usize,
    pub text: String,
    pub after_dot: bool,
    pub recv_type: Option<String>,
}

pub fn matching(
    prefix: &Prefix,
    code: &str,
    lang: Lang,
    code_lang: crate::codelang::CodeLang,
) -> Vec<Completion> {
    suggest(prefix, code, false, lang, code_lang)
}

pub fn matching_forced(
    prefix: &Prefix,
    code: &str,
    lang: Lang,
    code_lang: crate::codelang::CodeLang,
) -> Vec<Completion> {
    suggest(prefix, code, true, lang, code_lang)
}

fn suggest(
    prefix: &Prefix,
    code: &str,
    forced: bool,
    lang: Lang,
    code_lang: crate::codelang::CodeLang,
) -> Vec<Completion> {
    let p = prefix.text.to_ascii_lowercase();
    if !forced && p.is_empty() && !prefix.after_dot {
        return Vec::new();
    }

    let mut hits: Vec<(i32, Completion)> = Vec::new();
    let mut seen = HashSet::new();

    if !prefix.after_dot {
        for (name, kind) in declared_names_lang(code, code_lang) {
            if name == prefix.text {
                continue;
            }
            let l = name.to_ascii_lowercase();
            let ok = forced && p.is_empty()
                || l.starts_with(&p)
                || (!p.is_empty() && l.contains(&p));
            if ok && seen.insert(name.clone()) {
                let rank = if l.starts_with(&p) { 0 } else { 1 };
                hits.push((
                    rank,
                    item(&name, &name, &kind, var_doc(lang, &name, &kind)),
                ));
            }
        }
    }

    let recv = prefix.recv_type.as_deref();
    for api in catalog_for(code_lang) {
        if prefix.after_dot {
            match (recv, api.owner) {
                (Some(t), Some(owner)) if type_matches(t, owner) => {}
                (None, Some(owner)) if matches!(owner, "String" | "string" | "str") => {}
                _ => continue,
            }
        } else if api.owner.is_some() && !api.global {
            continue;
        }
        let insert = if prefix.after_dot {
            api.insert.trim_start_matches('.').to_string()
        } else {
            api.insert.to_string()
        };
        let l = api.label.to_ascii_lowercase();
        let ok = (forced && p.is_empty())
            || prefix.after_dot && p.is_empty()
            || l.starts_with(&p)
            || (!p.is_empty() && l.contains(&p));
        if ok && seen.insert(api.label.to_string()) {
            let rank = if prefix.after_dot {
                0
            } else if l.starts_with(&p) {
                2
            } else {
                3
            };
            hits.push((
                rank,
                item(
                    api.label,
                    &insert,
                    api.detail,
                    if lang == Lang::Es {
                        api.doc_es
                    } else {
                        api.doc_en
                    }
                    .into(),
                ),
            ));
        }
    }

    hits.sort_by_key(|(rank, item)| (*rank, item.label.len()));
    hits.into_iter().map(|(_, c)| c).take(14).collect()
}

pub fn declared_names(code: &str) -> Vec<(String, String)> {
    declared_names_lang(code, crate::codelang::CodeLang::Java)
}

pub fn declared_names_lang(
    code: &str,
    lang: crate::codelang::CodeLang,
) -> Vec<(String, String)> {
    match lang {
        crate::codelang::CodeLang::Python => declared_python(code),
        crate::codelang::CodeLang::CSharp => declared_typed(
            code,
            r"\b(string|int|bool|double|char|long|float|short|var|Console|Math)(\s*\[\s*\])?\s+([A-Za-z_][A-Za-z0-9_]*)",
        ),
        crate::codelang::CodeLang::Java => declared_typed(
            code,
            r"\b(String|int|boolean|double|char|long|float|short|Integer|Double|Boolean|Math|System)(\s*\[\s*\])?\s+([A-Za-z_][A-Za-z0-9_]*)",
        ),
    }
}

fn declared_python(code: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let re = Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=").expect("py decl");
    for cap in re.captures_iter(code) {
        let name = cap.get(1).unwrap().as_str();
        if matches!(
            name,
            "if" | "for" | "while" | "def" | "class" | "True" | "False" | "None" | "print"
        ) {
            continue;
        }
        if seen.insert(name.to_string()) {
            out.push((name.to_string(), "var".into()));
        }
    }
    out
}

fn declared_typed(code: &str, pat: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let decl = Regex::new(pat).expect("decl regex");
    for line in code.lines() {
        let vis = line
            .find("//")
            .or_else(|| line.find('#'))
            .map(|i| &line[..i])
            .unwrap_or(line);
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

pub fn current_prefix(
    code: &str,
    cursor_bytes: usize,
    code_lang: crate::codelang::CodeLang,
) -> Prefix {
    let cursor = cursor_bytes.min(code.len());
    let before = &code[..cursor];
    let start = before
        .char_indices()
        .rev()
        .find(|(_, ch)| !(ch.is_alphanumeric() || *ch == '_'))
        .map(|(i, ch)| i + ch.len_utf8())
        .unwrap_or(0);
    let text = before[start..].to_string();
    let after_dot = before[..start].ends_with('.');
    let recv_type = if after_dot {
        recv_type_at(code, start.saturating_sub(1), code_lang)
    } else {
        None
    };
    Prefix {
        start,
        text,
        after_dot,
        recv_type,
    }
}

fn recv_type_at(
    code: &str,
    dot_at: usize,
    code_lang: crate::codelang::CodeLang,
) -> Option<String> {
    let before = &code[..dot_at];
    let ident_start = before
        .char_indices()
        .rev()
        .find(|(_, ch)| !(ch.is_alphanumeric() || *ch == '_'))
        .map(|(i, ch)| i + ch.len_utf8())
        .unwrap_or(0);
    let ident = &before[ident_start..];
    if ident.is_empty() {
        return None;
    }
    match ident {
        "out" if before[..ident_start].ends_with("System.") => return Some("PrintStream".into()),
        "System" => return Some("System".into()),
        "Console" => return Some("Console".into()),
        "Math" => return Some("Math".into()),
        "String" => return Some("String".into()),
        "string" => return Some("string".into()),
        "Integer" => return Some("Integer".into()),
        "int" if matches!(code_lang, crate::codelang::CodeLang::CSharp) => {
            return Some("int".into())
        }
        "Double" => return Some("Double".into()),
        "str" => return Some("str".into()),
        _ => {}
    }
    declared_names_lang(code, code_lang)
        .into_iter()
        .find(|(n, _)| n == ident)
        .map(|(_, t)| t)
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

struct Api {
    label: &'static str,
    insert: &'static str,
    detail: &'static str,
    doc_es: &'static str,
    doc_en: &'static str,
    owner: Option<&'static str>,
    global: bool,
}

fn catalog_for(lang: crate::codelang::CodeLang) -> Vec<Api> {
    match lang {
        crate::codelang::CodeLang::Java => catalog_java(),
        crate::codelang::CodeLang::CSharp => catalog_cs(),
        crate::codelang::CodeLang::Python => catalog_py(),
    }
}

fn catalog_java() -> Vec<Api> {
    vec![
        Api {
            label: "sout",
            insert: "System.out.println(|);",
            detail: "imprimir",
            doc_es: "System.out.println(x) — escribe x y un salto de línea en la consola (PrintStream).",
            doc_en: "System.out.println(x) — prints x and a newline to the console (PrintStream).",
            owner: None,
            global: true,
        },
        Api {
            label: "for",
            insert: "for (|) {\n            \n        }",
            detail: "ciclo for",
            doc_es: "for (init; cond; step) — el cursor queda dentro del paréntesis.",
            doc_en: "for (init; cond; step) — cursor lands inside the parentheses.",
            owner: None,
            global: true,
        },
        Api {
            label: "fori",
            insert: "for (int i = 0; i < |.length; i++) {\n            \n        }",
            detail: "ciclo for",
            doc_es: "for (init; cond; step) — repite mientras cond sea true. i++ avanza el índice.",
            doc_en: "for (init; cond; step) — repeats while cond is true. i++ advances the index.",
            owner: None,
            global: true,
        },
        Api {
            label: "while",
            insert: "while (|) {\n            \n        }",
            detail: "ciclo while",
            doc_es: "while (cond) — repite el bloque mientras cond sea true. Debes cambiar cond o no termina.",
            doc_en: "while (cond) — repeats the block while cond is true. You must change cond or it never ends.",
            owner: None,
            global: true,
        },
        Api {
            label: "if",
            insert: "if (|) {\n            \n        }",
            detail: "condicional",
            doc_es: "if (cond) — ejecuta el bloque solo si cond es true.",
            doc_en: "if (cond) — runs the block only if cond is true.",
            owner: None,
            global: true,
        },
        Api {
            label: "ife",
            insert: "if (|) {\n            \n        } else {\n            \n        }",
            detail: "if / else",
            doc_es: "if / else — un camino si cond es true, el otro si es false.",
            doc_en: "if / else — one path if cond is true, the other if false.",
            owner: None,
            global: true,
        },
        m("length()", ".length()", "int", "String",
            "int length() — número de caracteres del String. \"Hola\".length() → 4.",
            "int length() — character count of the String. \"Hi\".length() → 2."),
        m("charAt", ".charAt(|)", "char", "String",
            "char charAt(int i) — carácter en el índice i (0 = primero). \"Hola\".charAt(1) → 'o'. IndexOutOfBounds si i no vale.",
            "char charAt(int i) — character at index i (0 = first). Throws IndexOutOfBounds if i is invalid."),
        m("substring", ".substring(|)", "String", "String",
            "String substring(int from) / substring(from, to) — recorte. to es exclusivo. \"Hola\".substring(1, 3) → \"ol\".",
            "String substring(int from) / substring(from, to) — slice. to is exclusive."),
        m("equals", ".equals(|)", "boolean", "String",
            "boolean equals(Object o) — compara contenido. Usa esto, no ==, para textos.",
            "boolean equals(Object o) — content equality. Use this, not ==, for strings."),
        m("equalsIgnoreCase", ".equalsIgnoreCase(|)", "boolean", "String",
            "boolean equalsIgnoreCase(String s) — igual que equals, ignora mayúsculas.",
            "boolean equalsIgnoreCase(String s) — like equals, case-insensitive."),
        m("toUpperCase", ".toUpperCase()", "String", "String",
            "String toUpperCase() — copia en MAYÚSCULAS. El original no cambia (String es inmutable).",
            "String toUpperCase() — uppercase copy. Original unchanged (String is immutable)."),
        m("toLowerCase", ".toLowerCase()", "String", "String",
            "String toLowerCase() — copia en minúsculas. Inmutable: devuelve un String nuevo.",
            "String toLowerCase() — lowercase copy. Returns a new String."),
        m("contains", ".contains(|)", "boolean", "String",
            "boolean contains(CharSequence s) — true si el texto contiene s.",
            "boolean contains(CharSequence s) — true if this string contains s."),
        m("indexOf", ".indexOf(|)", "int", "String",
            "int indexOf(String s) — primer índice de s, o -1 si no está.",
            "int indexOf(String s) — first index of s, or -1 if missing."),
        m("lastIndexOf", ".lastIndexOf(|)", "int", "String",
            "int lastIndexOf(String s) — último índice de s, o -1.",
            "int lastIndexOf(String s) — last index of s, or -1."),
        m("startsWith", ".startsWith(|)", "boolean", "String",
            "boolean startsWith(String p) — true si empieza por p.",
            "boolean startsWith(String p) — true if it starts with p."),
        m("endsWith", ".endsWith(|)", "boolean", "String",
            "boolean endsWith(String s) — true si termina en s.",
            "boolean endsWith(String s) — true if it ends with s."),
        m("isEmpty", ".isEmpty()", "boolean", "String",
            "boolean isEmpty() — true si length() == 0.",
            "boolean isEmpty() — true if length() == 0."),
        m("isBlank", ".isBlank()", "boolean", "String",
            "boolean isBlank() — true si está vacío o solo espacios (Java 11+).",
            "boolean isBlank() — true if empty or whitespace only (Java 11+)."),
        m("trim", ".trim()", "String", "String",
            "String trim() — quita espacios al inicio y al final (ASCII).",
            "String trim() — strips leading/trailing ASCII whitespace."),
        m("replace", ".replace(|, )", "String", "String",
            "String replace(CharSequence a, CharSequence b) — sustituye todas las apariciones de a por b.",
            "String replace(CharSequence a, CharSequence b) — replace all a with b."),
        m("split", ".split(|)", "String[]", "String",
            "String[] split(String regex) — parte el texto. \"a,b\".split(\",\") → [\"a\",\"b\"].",
            "String[] split(String regex) — split by regex. \"a,b\".split(\",\") → [\"a\",\"b\"]."),
        m("concat", ".concat(|)", "String", "String",
            "String concat(String s) — concatena. Equivale a + con Strings.",
            "String concat(String s) — concatenate. Same idea as + for Strings."),
        m("compareTo", ".compareTo(|)", "int", "String",
            "int compareTo(String s) — <0 si this va antes, 0 si igual, >0 si después (orden lexicográfico).",
            "int compareTo(String s) — <0 if this is before, 0 if equal, >0 if after."),
        m("valueOf", "String.valueOf(|)", "String", "String",
            "static String valueOf(x) — convierte número/boolean/char a texto.",
            "static String valueOf(x) — converts number/boolean/char to text."),
        Api {
            label: "length",
            insert: ".length",
            detail: "int",
            doc_es: "arr.length — cantidad de elementos del arreglo (campo, no método). No lleva ().",
            doc_en: "arr.length — array element count (field, not a method). No parentheses.",
            owner: Some("array"),
            global: true,
        },
        m("println", ".println(|);", "void", "PrintStream",
            "void println(x) — imprime x y salto de línea. System.out es un PrintStream.",
            "void println(x) — prints x then a newline. System.out is a PrintStream."),
        m("print", ".print(|);", "void", "PrintStream",
            "void print(x) — imprime x sin salto de línea.",
            "void print(x) — prints x with no newline."),
        m("printf", ".printf(|);", "PrintStream", "PrintStream",
            "PrintStream printf(String fmt, Object... args) — formato estilo C: %s %d %n.",
            "PrintStream printf(String fmt, Object... args) — C-style format: %s %d %n."),
        m("out", ".out", "PrintStream", "System",
            "System.out — PrintStream estándar de salida (consola).",
            "System.out — standard output PrintStream (console)."),
        m("abs", ".abs(|)", "num", "Math",
            "static Math.abs(x) — valor absoluto.",
            "static Math.abs(x) — absolute value."),
        m("max", ".max(|, )", "num", "Math",
            "static Math.max(a, b) — el mayor de los dos.",
            "static Math.max(a, b) — the larger of the two."),
        m("min", ".min(|, )", "num", "Math",
            "static Math.min(a, b) — el menor de los dos.",
            "static Math.min(a, b) — the smaller of the two."),
        m("pow", ".pow(|, )", "double", "Math",
            "static double Math.pow(a, b) — a elevado a b.",
            "static double Math.pow(a, b) — a raised to b."),
        m("sqrt", ".sqrt(|)", "double", "Math",
            "static double Math.sqrt(x) — raíz cuadrada.",
            "static double Math.sqrt(x) — square root."),
        m("random", ".random()", "double", "Math",
            "static double Math.random() — [0.0, 1.0). Para 0..n: (int)(Math.random()*n).",
            "static double Math.random() — [0.0, 1.0). For 0..n: (int)(Math.random()*n)."),
        m("round", ".round(|)", "long", "Math",
            "static long Math.round(double) — redondeo al entero más cercano.",
            "static long Math.round(double) — nearest whole number."),
        m("parseInt", "Integer.parseInt(|)", "int", "Integer",
            "static int Integer.parseInt(String s) — texto a int. NumberFormatException si no es número.",
            "static int Integer.parseInt(String s) — text to int. NumberFormatException if invalid."),
        m("parseDouble", "Double.parseDouble(|)", "double", "Double",
            "static double Double.parseDouble(String s) — texto a double.",
            "static double Double.parseDouble(String s) — text to double."),
        kw("String", "tipo",
            "String — texto inmutable. Compara con equals, no ==. Índices desde 0.",
            "String — immutable text. Compare with equals, not ==. Indexes from 0."),
        kw("int", "tipo",
            "int — entero 32 bits (−2³¹ … 2³¹−1). Literales: 0, 18, -3.",
            "int — 32-bit integer (−2³¹ … 2³¹−1). Literals: 0, 18, -3."),
        kw("boolean", "tipo",
            "boolean — solo true o false. Condiciones de if/while.",
            "boolean — only true or false. Used in if/while conditions."),
        kw("double", "tipo",
            "double — decimal 64 bits. 3.14, 0.5. Cuidado con == entre decimales.",
            "double — 64-bit floating point. 3.14, 0.5. Be careful with == on floats."),
        kw("char", "tipo",
            "char — un carácter Unicode entre comillas simples: 'A', 'ñ'.",
            "char — one Unicode character in single quotes: 'A'."),
        kw("true", "literal", "true — valor boolean verdadero.", "true — boolean true."),
        kw("false", "literal", "false — valor boolean falso.", "false — boolean false."),
        kw("null", "literal",
            "null — referencia vacía. Llamar un método sobre null → NullPointerException.",
            "null — empty reference. Calling a method on null → NullPointerException."),
        kw("new", "operador",
            "new Tipo() — crea un objeto en memoria (p. ej. new String[3]).",
            "new Type() — allocates an object (e.g. new String[3])."),
        Api {
            label: "else",
            insert: "else {\n            |\n        }",
            detail: "else",
            doc_es: "else — rama alternativa del if. Cursor dentro del bloque.",
            doc_en: "else — alternative branch of if. Cursor inside the block.",
            owner: None,
            global: true,
        },
        Api {
            label: "switch",
            insert: "switch (|) {\n            case :\n                break;\n            default:\n        }",
            detail: "switch",
            doc_es: "switch (x) — elige un case. Cursor dentro del paréntesis.",
            doc_en: "switch (x) — pick a case. Cursor inside the parentheses.",
            owner: None,
            global: true,
        },
        kw("return", "keyword", "return x; — sale del método y entrega x.", "return x; — leave the method with value x."),
        kw("System", "clase",
            "java.lang.System — System.out (consola), System.in, currentTimeMillis().",
            "java.lang.System — System.out (console), System.in, currentTimeMillis()."),
        kw("Math", "clase",
            "java.lang.Math — abs, max, min, pow, sqrt, random. Métodos static: Math.x(...).",
            "java.lang.Math — abs, max, min, pow, sqrt, random. Static methods: Math.x(...)."),
        kw("Integer", "clase",
            "Integer — caja de int. Integer.parseInt(s) convierte texto a int.",
            "Integer — int wrapper. Integer.parseInt(s) parses text to int."),
    ]
}

fn m(
    label: &'static str,
    insert: &'static str,
    detail: &'static str,
    owner: &'static str,
    doc_es: &'static str,
    doc_en: &'static str,
) -> Api {
    Api {
        label,
        insert,
        detail,
        doc_es,
        doc_en,
        owner: Some(owner),
        global: matches!(
            label,
            "equals" | "charAt" | "length()" | "length" | "parseInt" | "parseDouble"
                | "Length" | "Substring" | "Contains" | "ToUpper" | "Parse"
                | "upper" | "lower" | "find" | "replace" | "split"
        ),
    }
}

fn kw(label: &'static str, detail: &'static str, doc_es: &'static str, doc_en: &'static str) -> Api {
    Api {
        label,
        insert: label,
        detail,
        doc_es,
        doc_en,
        owner: None,
        global: true,
    }
}

fn type_matches(have: &str, owner: &str) -> bool {
    if owner == "array" {
        return have.ends_with("[]") || have == "list";
    }
    have == owner
        || (owner == "String" && matches!(have, "String" | "string"))
        || (owner == "string" && matches!(have, "string" | "String"))
        || (owner == "str" && matches!(have, "str" | "var" | "String" | "string"))
        || (owner == "PrintStream" && have == "PrintStream")
        || (owner == "Console" && have == "Console")
}

fn var_doc(lang: Lang, name: &str, kind: &str) -> String {
    let base = match kind.trim_end_matches("[]") {
        "String" | "string" | "str" => {
            if lang == Lang::Es {
                "Texto. Métodos según el lenguaje: length/len, substring, upper, equals/==."
            } else {
                "Text. Language methods: length/len, substring, upper, equals/==."
            }
        }
        "int" | "long" | "short" => {
            if lang == Lang::Es {
                "Entero. Operadores: + - * / %  comparar con == != < >."
            } else {
                "Integer. Operators: + - * / %  compare with == != < >."
            }
        }
        "boolean" | "bool" => {
            if lang == Lang::Es {
                "boolean. Combina con && || !  — va en if/while."
            } else {
                "boolean. Combine with && || !  — used in if/while."
            }
        }
        "double" | "float" => {
            if lang == Lang::Es {
                "Decimal. Math.round / (int) para truncar."
            } else {
                "Floating point. Math.round / (int) to truncate."
            }
        }
        "char" => {
            if lang == Lang::Es {
                "Un carácter. Puedes sumar: c + 1. Compara con ==."
            } else {
                "One character. You can add: c + 1. Compare with ==."
            }
        }
        _ => {
            if lang == Lang::Es {
                "Identificador declarado en tu código."
            } else {
                "Name declared in your code."
            }
        }
    };
    if kind.ends_with("[]") {
        format!("{name}: {kind}. length = cantidad de casillas. Índices 0..length-1.")
    } else {
        format!("{name}: {kind}. {base}")
    }
}

fn catalog_cs() -> Vec<Api> {
    vec![
        Api {
            label: "cw",
            insert: "Console.WriteLine(|);",
            detail: "imprimir",
            doc_es: "Console.WriteLine(x) — escribe x y un salto de línea en la consola.",
            doc_en: "Console.WriteLine(x) — prints x and a newline.",
            owner: None,
            global: true,
        },
        Api {
            label: "if",
            insert: "if (|) {\n            \n        }",
            detail: "condicional",
            doc_es: "if (cond) — ejecuta el bloque solo si cond es true.",
            doc_en: "if (cond) — runs the block only if cond is true.",
            owner: None,
            global: true,
        },
        Api {
            label: "ife",
            insert: "if (|) {\n            \n        } else {\n            \n        }",
            detail: "if / else",
            doc_es: "if / else — un camino si cond es true, el otro si es false.",
            doc_en: "if / else — one path if cond is true, the other if false.",
            owner: None,
            global: true,
        },
        Api {
            label: "else",
            insert: "else {\n            |\n        }",
            detail: "else",
            doc_es: "else — rama alternativa del if.",
            doc_en: "else — alternative branch of if.",
            owner: None,
            global: true,
        },
        Api {
            label: "for",
            insert: "for (|) {\n            \n        }",
            detail: "ciclo for",
            doc_es: "for (init; cond; step) — el cursor queda dentro del paréntesis.",
            doc_en: "for (init; cond; step) — cursor lands inside the parentheses.",
            owner: None,
            global: true,
        },
        Api {
            label: "foreach",
            insert: "foreach (var x in |) {\n            \n        }",
            detail: "foreach",
            doc_es: "foreach (var x in seq) — recorre cada elemento.",
            doc_en: "foreach (var x in seq) — iterates each element.",
            owner: None,
            global: true,
        },
        Api {
            label: "while",
            insert: "while (|) {\n            \n        }",
            detail: "ciclo while",
            doc_es: "while (cond) — repite mientras cond sea true.",
            doc_en: "while (cond) — repeats while cond is true.",
            owner: None,
            global: true,
        },
        Api {
            label: "switch",
            insert: "switch (|) {\n            case :\n                break;\n            default:\n        }",
            detail: "switch",
            doc_es: "switch (x) — elige un case.",
            doc_en: "switch (x) — pick a case.",
            owner: None,
            global: true,
        },
        m("Length", ".Length", "int", "string",
            "int Length — número de caracteres. \"Hola\".Length → 4. Es propiedad, no lleva ().",
            "int Length — character count. Property, no parentheses."),
        m("Substring", ".Substring(|)", "string", "string",
            "string Substring(int from) / Substring(from, len) — recorte. El 2.º arg es longitud.",
            "string Substring(int from) / Substring(from, len) — slice. 2nd arg is length."),
        m("Equals", ".Equals(|)", "bool", "string",
            "bool Equals(string s) — compara contenido. Para texto usa Equals, no == (salvo interpolación habitual).",
            "bool Equals(string s) — content equality."),
        m("Contains", ".Contains(|)", "bool", "string",
            "bool Contains(string s) — true si el texto contiene s.",
            "bool Contains(string s) — true if this string contains s."),
        m("IndexOf", ".IndexOf(|)", "int", "string",
            "int IndexOf(string s) — primer índice de s, o -1 si no está.",
            "int IndexOf(string s) — first index of s, or -1."),
        m("ToUpper", ".ToUpper()", "string", "string",
            "string ToUpper() — copia en MAYÚSCULAS. El original no cambia.",
            "string ToUpper() — uppercase copy. Original unchanged."),
        m("ToLower", ".ToLower()", "string", "string",
            "string ToLower() — copia en minúsculas.",
            "string ToLower() — lowercase copy."),
        m("StartsWith", ".StartsWith(|)", "bool", "string",
            "bool StartsWith(string p) — true si empieza por p.",
            "bool StartsWith(string p) — true if it starts with p."),
        m("EndsWith", ".EndsWith(|)", "bool", "string",
            "bool EndsWith(string s) — true si termina en s.",
            "bool EndsWith(string s) — true if it ends with s."),
        m("Trim", ".Trim()", "string", "string",
            "string Trim() — quita espacios al inicio y al final.",
            "string Trim() — strips leading/trailing whitespace."),
        m("Replace", ".Replace(|, )", "string", "string",
            "string Replace(a, b) — sustituye todas las apariciones de a por b.",
            "string Replace(a, b) — replace all a with b."),
        m("Split", ".Split(|)", "string[]", "string",
            "string[] Split(char) — parte el texto. \"a,b\".Split(',') → [\"a\",\"b\"].",
            "string[] Split(char) — split. \"a,b\".Split(',') → [\"a\",\"b\"]."),
        m("WriteLine", ".WriteLine(|);", "void", "Console",
            "Console.WriteLine(x) — imprime x y salto de línea.",
            "Console.WriteLine(x) — prints x then a newline."),
        m("Write", ".Write(|);", "void", "Console",
            "Console.Write(x) — imprime x sin salto de línea.",
            "Console.Write(x) — prints x with no newline."),
        m("Abs", ".Abs(|)", "num", "Math",
            "static Math.Abs(x) — valor absoluto.",
            "static Math.Abs(x) — absolute value."),
        m("Max", ".Max(|, )", "num", "Math",
            "static Math.Max(a, b) — el mayor de los dos.",
            "static Math.Max(a, b) — the larger of the two."),
        m("Min", ".Min(|, )", "num", "Math",
            "static Math.Min(a, b) — el menor de los dos.",
            "static Math.Min(a, b) — the smaller of the two."),
        m("Pow", ".Pow(|, )", "double", "Math",
            "static double Math.Pow(a, b) — a elevado a b.",
            "static double Math.Pow(a, b) — a raised to b."),
        m("Sqrt", ".Sqrt(|)", "double", "Math",
            "static double Math.Sqrt(x) — raíz cuadrada.",
            "static double Math.Sqrt(x) — square root."),
        m("Round", ".Round(|)", "double", "Math",
            "static Math.Round(x) — redondeo al entero más cercano.",
            "static Math.Round(x) — nearest whole number."),
        m("Parse", "int.Parse(|)", "int", "int",
            "static int int.Parse(string s) — texto a int. FormatException si no es número.",
            "static int int.Parse(string s) — text to int. FormatException if invalid."),
        kw("string", "tipo",
            "string — texto. Length, Substring, Contains, ToUpper. Índices desde 0.",
            "string — text. Length, Substring, Contains, ToUpper. Indexes from 0."),
        kw("int", "tipo", "int — entero 32 bits. Literales: 0, 18, -3.", "int — 32-bit integer."),
        kw("bool", "tipo", "bool — solo true o false.", "bool — only true or false."),
        kw("double", "tipo", "double — decimal. Cuidado con == entre decimales.", "double — floating point."),
        kw("char", "tipo", "char — un carácter entre comillas simples: 'A'.", "char — one character in single quotes."),
        kw("true", "literal", "true — valor bool verdadero.", "true — boolean true."),
        kw("false", "literal", "false — valor bool falso.", "false — boolean false."),
        kw("null", "literal", "null — referencia vacía. NullReferenceException si llamas un método.", "null — empty reference."),
        kw("var", "tipo", "var — el compilador infiere el tipo: var x = 3; es int.", "var — type inferred: var x = 3; is int."),
        kw("new", "operador", "new Tipo() — crea un objeto (p. ej. new int[3]).", "new Type() — allocates an object."),
        kw("return", "keyword", "return x; — sale del método y entrega x.", "return x; — leave the method with value x."),
        kw("Console", "clase", "System.Console — WriteLine, Write, ReadLine.", "System.Console — WriteLine, Write, ReadLine."),
        kw("Math", "clase", "System.Math — Abs, Max, Min, Pow, Sqrt. Métodos static: Math.X(...).", "System.Math — Abs, Max, Min, Pow, Sqrt."),
    ]
}

fn catalog_py() -> Vec<Api> {
    vec![
        Api {
            label: "print",
            insert: "print(|)",
            detail: "imprimir",
            doc_es: "print(x) — escribe x y un salto de línea. Varios valores: print(a, b).",
            doc_en: "print(x) — prints x and a newline. Several values: print(a, b).",
            owner: None,
            global: true,
        },
        Api {
            label: "if",
            insert: "if |:\n    ",
            detail: "condicional",
            doc_es: "if cond: — ejecuta el bloque indentado solo si cond es True. Asigna con =, no con :.",
            doc_en: "if cond: — runs the indented block only if cond is True.",
            owner: None,
            global: true,
        },
        Api {
            label: "ife",
            insert: "if |:\n    \nelse:\n    ",
            detail: "if / else",
            doc_es: "if / else — un camino si cond es True, el otro si es False.",
            doc_en: "if / else — one path if True, the other if False.",
            owner: None,
            global: true,
        },
        Api {
            label: "elif",
            insert: "elif |:\n    ",
            detail: "elif",
            doc_es: "elif cond: — otra condición después de if. No uses else if.",
            doc_en: "elif cond: — another condition after if. Not else if.",
            owner: None,
            global: true,
        },
        Api {
            label: "else",
            insert: "else:\n    |",
            detail: "else",
            doc_es: "else: — rama alternativa del if. Lleva dos puntos.",
            doc_en: "else: — alternative branch. Needs a colon.",
            owner: None,
            global: true,
        },
        Api {
            label: "for",
            insert: "for | in :\n    ",
            detail: "ciclo for",
            doc_es: "for x in seq: — recorre cada elemento. range(n) da 0..n-1.",
            doc_en: "for x in seq: — iterates each element. range(n) is 0..n-1.",
            owner: None,
            global: true,
        },
        Api {
            label: "while",
            insert: "while |:\n    ",
            detail: "ciclo while",
            doc_es: "while cond: — repite mientras cond sea True. Debes cambiar cond o no termina.",
            doc_en: "while cond: — repeats while True. You must change cond or it never ends.",
            owner: None,
            global: true,
        },
        Api {
            label: "def",
            insert: "def |():\n    ",
            detail: "función",
            doc_es: "def nombre(): — define una función. El cuerpo va indentado.",
            doc_en: "def name(): — defines a function. Body is indented.",
            owner: None,
            global: true,
        },
        kw("len", "builtin",
            "len(x) — cantidad de caracteres o elementos. len(\"Hola\") → 4.",
            "len(x) — length of a string or sequence. len(\"Hi\") → 2."),
        kw("range", "builtin",
            "range(n) — 0,1,…,n-1. range(a, b) empieza en a y no incluye b.",
            "range(n) — 0,1,…,n-1. range(a, b) starts at a, excludes b."),
        kw("int", "tipo",
            "int(x) — convierte a entero. int(\"18\") → 18. ValueError si no es número.",
            "int(x) — convert to integer. ValueError if invalid."),
        kw("str", "tipo",
            "str(x) — convierte a texto. Métodos: upper, lower, find, replace, split.",
            "str(x) — convert to text. Methods: upper, lower, find, replace, split."),
        kw("float", "tipo", "float(x) — decimal. float(\"3.14\") → 3.14.", "float(x) — floating point."),
        kw("bool", "tipo", "bool — True o False. Condiciones de if/while.", "bool — True or False."),
        kw("True", "literal", "True — valor verdadero (T mayúscula).", "True — boolean true."),
        kw("False", "literal", "False — valor falso (F mayúscula).", "False — boolean false."),
        kw("None", "literal", "None — sin valor. No uses null.", "None — no value. Not null."),
        kw("return", "keyword", "return x — sale de la función y entrega x.", "return x — leave the function with value x."),
        kw("and", "op", "and — las dos condiciones True.", "and — both conditions True."),
        kw("or", "op", "or — al menos una condición True.", "or — at least one True."),
        kw("not", "op", "not — invierte True/False.", "not — invert True/False."),
        m("upper", ".upper()", "str", "str",
            "s.upper() — copia en MAYÚSCULAS. El original no cambia.",
            "s.upper() — uppercase copy. Original unchanged."),
        m("lower", ".lower()", "str", "str",
            "s.lower() — copia en minúsculas.",
            "s.lower() — lowercase copy."),
        m("find", ".find(|)", "int", "str",
            "s.find(sub) — primer índice de sub, o -1 si no está.",
            "s.find(sub) — first index of sub, or -1."),
        m("index", ".index(|)", "int", "str",
            "s.index(sub) — como find, pero ValueError si no está.",
            "s.index(sub) — like find, but ValueError if missing."),
        m("replace", ".replace(|, )", "str", "str",
            "s.replace(a, b) — sustituye todas las apariciones de a por b.",
            "s.replace(a, b) — replace all a with b."),
        m("split", ".split(|)", "list", "str",
            "s.split(sep) — parte el texto. \"a,b\".split(\",\") → [\"a\",\"b\"].",
            "s.split(sep) — split. \"a,b\".split(\",\") → [\"a\",\"b\"]."),
        m("strip", ".strip()", "str", "str",
            "s.strip() — quita espacios al inicio y al final.",
            "s.strip() — strips leading/trailing whitespace."),
        m("startswith", ".startswith(|)", "bool", "str",
            "s.startswith(p) — True si empieza por p.",
            "s.startswith(p) — True if it starts with p."),
        m("endswith", ".endswith(|)", "bool", "str",
            "s.endswith(p) — True si termina en p.",
            "s.endswith(p) — True if it ends with p."),
        m("isdigit", ".isdigit()", "bool", "str",
            "s.isdigit() — True si todos los caracteres son dígitos.",
            "s.isdigit() — True if all characters are digits."),
        m("join", ".join(|)", "str", "str",
            "sep.join(lista) — une elementos. \",\".join([\"a\",\"b\"]) → \"a,b\".",
            "sep.join(seq) — join items with this separator."),
        kw("abs", "builtin", "abs(x) — valor absoluto.", "abs(x) — absolute value."),
        kw("max", "builtin", "max(a, b) — el mayor.", "max(a, b) — the larger."),
        kw("min", "builtin", "min(a, b) — el menor.", "min(a, b) — the smaller."),
        kw("round", "builtin", "round(x) — entero más cercano. round(x, n) con decimales.", "round(x) — nearest whole number."),
    ]
}

fn item(label: &str, insert: &str, detail: &str, doc: String) -> Completion {
    Completion {
        label: label.into(),
        insert: insert.into(),
        detail: detail.into(),
        doc,
    }
}
