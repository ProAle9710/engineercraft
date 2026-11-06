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

pub fn matching(prefix: &Prefix, code: &str, lang: Lang) -> Vec<Completion> {
    suggest(prefix, code, false, lang)
}

pub fn matching_forced(prefix: &Prefix, code: &str, lang: Lang) -> Vec<Completion> {
    suggest(prefix, code, true, lang)
}

fn suggest(prefix: &Prefix, code: &str, forced: bool, lang: Lang) -> Vec<Completion> {
    let p = prefix.text.to_ascii_lowercase();
    if !forced && p.is_empty() && !prefix.after_dot {
        return Vec::new();
    }

    let mut hits: Vec<(i32, Completion)> = Vec::new();
    let mut seen = HashSet::new();

    if !prefix.after_dot {
        for (name, kind) in declared_names(code) {
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
    for api in catalog() {
        if prefix.after_dot {
            match (recv, api.owner) {
                (Some(t), Some(owner)) if type_matches(t, owner) => {}
                (None, Some("String")) => {}
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
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let decl = Regex::new(
        r"\b(String|int|boolean|double|char|long|float|short|Integer|Double|Boolean|Math|System)(\s*\[\s*\])?\s+([A-Za-z_][A-Za-z0-9_]*)",
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

pub fn current_prefix(code: &str, cursor_bytes: usize) -> Prefix {
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
        recv_type_at(code, start.saturating_sub(1))
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

fn recv_type_at(code: &str, dot_at: usize) -> Option<String> {
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
    if ident == "out" && before[..ident_start].ends_with("System.") {
        return Some("PrintStream".into());
    }
    if ident == "System" {
        return Some("System".into());
    }
    if ident == "Math" {
        return Some("Math".into());
    }
    if ident == "String" {
        return Some("String".into());
    }
    if ident == "Integer" {
        return Some("Integer".into());
    }
    if ident == "Double" {
        return Some("Double".into());
    }
    declared_names(code)
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

fn catalog() -> Vec<Api> {
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
        return have.ends_with("[]");
    }
    have == owner || (owner == "String" && have == "String") || (owner == "PrintStream" && have == "PrintStream")
}

fn var_doc(lang: Lang, name: &str, kind: &str) -> String {
    let base = match kind.trim_end_matches("[]") {
        "String" => {
            if lang == Lang::Es {
                "Variable String. Métodos: length(), charAt(i), equals(s), substring(a,b), toUpperCase()."
            } else {
                "String variable. Methods: length(), charAt(i), equals(s), substring(a,b), toUpperCase()."
            }
        }
        "int" | "long" | "short" => {
            if lang == Lang::Es {
                "Entero. Operadores: + - * / %  comparar con == != < >."
            } else {
                "Integer. Operators: + - * / %  compare with == != < >."
            }
        }
        "boolean" => {
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

fn item(label: &str, insert: &str, detail: &str, doc: String) -> Completion {
    Completion {
        label: label.into(),
        insert: insert.into(),
        detail: detail.into(),
        doc,
    }
}
