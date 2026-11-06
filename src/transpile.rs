use crate::catalog::{self, Exercise};
use crate::codelang::CodeLang;
use regex::Regex;
use std::collections::HashMap;

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
    out.brief_es = rewrite_guide(&ex.brief_es, lang);
    out.brief_en = rewrite_guide(&ex.brief_en, lang);
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
    let lang = CodeLang::Python;
    let mut out = String::new();
    for line in java_prefix.lines() {
        let t = line.trim();
        if t.starts_with("public class") || t.starts_with("public static") || t == "{" {
            break;
        }
        if t.starts_with("//") {
            let c = rewrite_comment(&line.replace("//", "#"), lang);
            out.push_str(&c);
            out.push('\n');
        } else if t.is_empty() {
            out.push('\n');
        }
    }
    out
}

fn to_cs(src: &str) -> String {
    map_code_lines(src, |l| {
        let s = l.replace("System.out.println", "Console.WriteLine")
            .replace("System.out.print", "Console.Write")
            .replace("boolean ", "bool ")
            .replace("String ", "string ")
            .replace("String[]", "string[]")
            .replace(".length()", ".Length")
            .replace(".length", ".Length")
            .replace(".equals(", ".Equals(");
        Regex::new(r"\.charAt\(\s*([^()]*)\s*\)")
            .expect("charAt")
            .replace_all(&s, "[$1]")
            .into_owned()
    })
}

fn map_code_lines(src: &str, mut f: impl FnMut(&str) -> String) -> String {
    let mut out = String::new();
    for line in src.split_inclusive('\n') {
        let (nl, raw) = if let Some(s) = line.strip_suffix('\n') {
            (true, s)
        } else {
            (false, line)
        };
        let t = raw.trim_start();
        let mapped = if t.starts_with("//") || t.starts_with('#') {
            rewrite_comment(raw, CodeLang::CSharp)
        } else {
            f(raw)
        };
        out.push_str(&mapped);
        if nl {
            out.push('\n');
        }
    }
    out
}

fn to_py(src: &str) -> String {
    let mut out = String::new();
    for line in src.split_inclusive('\n') {
        let (nl, raw) = if let Some(s) = line.strip_suffix('\n') {
            (true, s)
        } else {
            (false, line)
        };
        let trim = raw.trim();
        if trim == "}" || trim == "} ;" {
            if nl {
                // skip Java's closing braces
            }
            continue;
        }
        if raw.trim_start().starts_with("//") || raw.trim_start().starts_with('#') {
            let l = rewrite_comment(&raw.replace("//", "#"), CodeLang::Python);
            out.push_str(&l);
            if nl {
                out.push('\n');
            }
            continue;
        }
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

fn rewrite_comment(line: &str, lang: CodeLang) -> String {
    rewrite_guide(line, lang)
}

fn rewrite_guide(text: &str, lang: CodeLang) -> String {
    match lang {
        CodeLang::Java => text.to_string(),
        CodeLang::CSharp => rewrite_cs(text),
        CodeLang::Python => rewrite_py(text),
    }
}

fn strip_java_type(text: &str) -> String {
    let re = regex::Regex::new(
        r"\b(String|string|int|boolean|bool|double|char|long|float)\s+(\w+)\s*=",
    )
    .expect("decl");
    re.replace_all(text, "$2 =").into_owned()
}

fn rewrite_py(text: &str) -> String {
    let mut s = text.to_string();
    s = s.replace("```java", "```python");
    s = s.replace("Starter.java", "Starter.py");
    s = s.replace("System.out.println", "print");
    s = s.replace("System.out.print", "print");
    s = s.replace("javac Starter.py", "python Starter.py");
    s = s.replace("javac Starter.java", "python Starter.py");
    s = s.replace("java Starter", "python Starter.py");
    s = s.replace("método `main`", "archivo (sin clase)");
    s = s.replace("método main", "archivo");
    s = s.replace("method `main`", "the file (no class)");
    s = s.replace("inside of main", "in the file");
    s = s.replace("dentro del método `main`", "en el archivo, sin clase");
    s = s.replace("dentro de main", "en el archivo");
    s = s.replace("inside main", "in the file");
    s = s.replace("sintaxis correcta de Java", "sintaxis correcta de Python");
    s = s.replace("correct Java syntax", "correct Python syntax");
    s = s.replace("Correct syntax:", "Correct syntax:");
    s = s.replace("Sintaxis correcta:", "Sintaxis correcta:");
    s = s.replace("En Java debes escribir el tipo `String` antes del nombre:", "En Python no se declara el tipo:");
    s = s.replace("In Java you must write the type `String` before the name:", "In Python you do not write a type:");
    s = s.replace("Un String es texto. En Java", "Un string es texto. En Python");
    s = s.replace("A String is text. In Java", "A string is text. In Python");
    s = s.replace("de Java", "de Python");
    s = s.replace(" of Java", " of Python");
    s = s.replace("String firstName", "firstName");
    s = s.replace("String lastName", "lastName");
    s = s.replace("String favoriteColor", "favoriteColor");
    s = strip_java_type(&s);
    s = s.replace("String name = \"value\";", "name = \"value\"");
    s = s.replace("`String name = \"...\"`", "`name = \"...\"`");
    s = s.replace("String[]", "list");
    s = s.replace("int[]", "list");
    s = s.replace("variable String llamada", "variable llamada");
    s = s.replace("variable String", "variable");
    s = s.replace("variables String", "variables de texto");
    s = s.replace("3 variables String", "3 variables de texto");
    s = s.replace("una String", "un texto");
    s = s.replace("un String", "un texto");
    s = s.replace("\";", "\"");
    s
}

fn rewrite_cs(text: &str) -> String {
    let mut s = text.to_string();
    s = s.replace("```java", "```csharp");
    s = s.replace("Starter.java", "Starter.cs");
    s = s.replace("System.out.println", "Console.WriteLine");
    s = s.replace("System.out.print", "Console.Write");
    s = s.replace("javac Starter.java", "dotnet run");
    s = s.replace("java Starter", "dotnet run");
    s = s.replace("public static void main", "static void Main");
    s = s.replace("void main", "void Main");
    s = s.replace("método `main`", "método `Main`");
    s = s.replace("method `main`", "method `Main`");
    s = s.replace("dentro de main", "dentro de Main");
    s = s.replace("inside main", "inside Main");
    s = s.replace("sintaxis correcta de Java", "sintaxis correcta de C#");
    s = s.replace("correct Java syntax", "correct C# syntax");
    s = s.replace("En Java debes escribir el tipo `String`", "En C# escribes el tipo `string`");
    s = s.replace("In Java you must write the type `String`", "In C# you write the type `string`");
    s = s.replace("Un String es texto. En Java", "Un string es texto. En C#");
    s = s.replace("de Java", "de C#");
    s = s.replace(" of Java", " of C#");
    s = s.replace("boolean ", "bool ");
    s = s.replace("`String`", "`string`");
    s = s.replace("String ", "string ");
    s = s.replace("String[]", "string[]");
    s
}

/// C# -> Java para el paso a paso (F10). Conserva el número de línea
/// 1:1 (toda sustitución es dentro de la línea; los helpers van al final).
pub fn cs_to_java(src: &str) -> String {
    let decls: HashMap<String, String> =
        crate::complete::declared_names_lang(src, CodeLang::CSharp)
            .into_iter()
            .collect();
    let mut out = String::new();
    for raw in src.split_inclusive('\n') {
        let (nl, line) = match raw.strip_suffix('\n') {
            Some(s) => (true, s),
            None => (false, raw),
        };
        out.push_str(&cs_line_to_java(line, &decls));
        if nl {
            out.push('\n');
        }
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(CS_JAVA_HELPERS);
    out
}

fn re(pat: &str) -> Regex {
    Regex::new(pat).expect("cs->java regex")
}

fn cs_line_to_java(line: &str, decls: &HashMap<String, String>) -> String {
    let t = line.trim_start();
    if t.starts_with("using ") && t.trim_end().ends_with(';') {
        return format!("{}// {}", &line[..line.len() - t.len()], t);
    }
    let mut l = line.to_string();
    l = l.replace("System.Console.", "Console.");
    l = l.replace("void Main(", "void main(");
    l = l.replace("public static void main(", "static void main(");
    l = l.replace("static void main(", "public static void main(");
    l = l.replace("Console.Error.WriteLine", "System.err.println");
    l = l.replace("Console.Error.Write", "System.err.print");
    l = l.replace("Console.WriteLine", "System.out.println");
    l = l.replace("Console.Write", "System.out.print");
    l = l.replace("Console.ReadLine()", "Dbg.readLine()");
    l = re(r"foreach\s*\(\s*([A-Za-z_]\w*(?:\s*\[\s*\])?)\s+([A-Za-z_]\w*)\s+in\s+([^)]+)\)")
        .replace_all(&l, "for ($1 $2 : $3)")
        .into_owned();
    l = l.replace("string.IsNullOrWhiteSpace(", "Dbg.blank(");
    l = l.replace("string.IsNullOrEmpty(", "Dbg.empty(");
    l = l.replace("string.Concat(", "Dbg.concat(");
    l = l.replace("string.Join(", "String.join(");
    l = l.replace("string.Format(", "String.format(");
    l = l.replace("string.Empty", "\"\"");
    l = l.replace("string.Copy(", "new String(");
    l = l.replace("Convert.ToInt32(", "Integer.parseInt(");
    l = l.replace("Convert.ToInt64(", "Long.parseLong(");
    l = l.replace("Convert.ToDouble(", "Double.parseDouble(");
    l = l.replace("Convert.ToSingle(", "Float.parseFloat(");
    l = l.replace("Convert.ToBoolean(", "Boolean.parseBoolean(");
    l = l.replace("Convert.ToChar(", "Dbg.firstChar(");
    l = l.replace("Convert.ToString(", "String.valueOf(");
    l = l.replace("int.Parse(", "Integer.parseInt(");
    l = l.replace("long.Parse(", "Long.parseLong(");
    l = l.replace("double.Parse(", "Double.parseDouble(");
    l = l.replace("float.Parse(", "Float.parseFloat(");
    l = l.replace("bool.Parse(", "Boolean.parseBoolean(");
    l = l.replace("char.Parse(", "Dbg.firstChar(");
    for (cs, j) in [
        ("Pow", "pow"),
        ("Sqrt", "sqrt"),
        ("Abs", "abs"),
        ("Max", "max"),
        ("Min", "min"),
        ("Round", "round"),
        ("Floor", "floor"),
        ("Ceiling", "ceil"),
        ("Exp", "exp"),
        ("Log", "log"),
    ] {
        l = l.replace(&format!("Math.{cs}("), &format!("Math.{j}("));
    }
    for (cs, j) in [
        ("ToUpper", "toUpperCase"),
        ("ToLower", "toLowerCase"),
        ("Trim", "trim"),
        ("Contains", "contains"),
        ("StartsWith", "startsWith"),
        ("EndsWith", "endsWith"),
        ("IndexOf", "indexOf"),
        ("LastIndexOf", "lastIndexOf"),
        ("Substring", "substring"),
        ("Split", "split"),
        ("Equals", "equals"),
        ("CompareTo", "compareTo"),
        ("Replace", "replace"),
        ("ToCharArray", "toCharArray"),
    ] {
        l = re(&format!(r"\.{}\s*\(", cs))
            .replace_all(&l, &format!(".{j}("))
            .into_owned();
    }
    l = re(r"\.\s*ToString\s*\(\s*\)")
        .replace_all(&l, ".toString()")
        .into_owned();
    l = re(r"(\w+)\.PadLeft\s*\(")
        .replace_all(&l, "Dbg.padLeft($1, ")
        .into_owned();
    l = re(r"(\w+)\.PadRight\s*\(")
        .replace_all(&l, "Dbg.padRight($1, ")
        .into_owned();
    l = re(r"(\w+)\.Length\b")
        .replace_all(&l, "Dbg.len($1)")
        .into_owned();
    l = re(r"(\w+(?:\.\w+)*)\s*\?\?\s*(\w+(?:\.\w+)*)")
        .replace_all(&l, "($1 != null ? $1 : $2)")
        .into_owned();
    l = l.replace("?.", ".");
    l = re(r"\{\d+\}").replace_all(&l, "%s").into_owned();
    l = l.replace("$\"", "\"").replace("@\"", "\"");
    l = re(r"\b(internal|override|virtual|partial)\s+")
        .replace_all(&l, "")
        .into_owned();
    l = re(r"\bsealed\b").replace_all(&l, "final").into_owned();
    l = re(r"\b(readonly|const)\b").replace_all(&l, "final").into_owned();
    l = re(r"\b(\d+(?:\.\d+)?)[mM]\b")
        .replace_all(&l, "$1")
        .into_owned();
    for (cs, j) in [
        ("string", "String"),
        ("bool", "boolean"),
        ("decimal", "double"),
        ("uint", "int"),
        ("ulong", "long"),
        ("ushort", "short"),
        ("sbyte", "byte"),
        ("object", "Object"),
    ] {
        l = re(&format!(r"\b{}\b", cs)).replace_all(&l, j).into_owned();
    }
    cs_index_to_java(&l, decls)
}

/// text[i] sobre variables `string` -> Dbg.charAt(text, i). Los arreglos
/// (string[], int[], ...) conservan corchetes.
fn cs_index_to_java(line: &str, decls: &HashMap<String, String>) -> String {
    let idx = re(r"(\w+)\s*\[([^\[\]]+)\]");
    idx.replace_all(line, |c: &regex::Captures| {
        let name = c.get(1).unwrap().as_str();
        if decls.get(name).map(|t| t.as_str()) == Some("string") {
            format!("Dbg.charAt({}, {})", name, c.get(2).unwrap().as_str())
        } else {
            c.get(0).unwrap().as_str().to_string()
        }
    })
    .into_owned()
}

const CS_JAVA_HELPERS: &str = r#"
class Dbg {
    static int len(String s) { return s.length(); }
    static int len(String[] a) { return a.length; }
    static int len(int[] a) { return a.length; }
    static int len(double[] a) { return a.length; }
    static int len(long[] a) { return a.length; }
    static int len(float[] a) { return a.length; }
    static int len(boolean[] a) { return a.length; }
    static int len(char[] a) { return a.length; }
    static int len(Object[] a) { return a.length; }
    static char charAt(String s, int i) { return s.charAt(i); }
    static char firstChar(String s) { return s.isEmpty() ? ' ' : s.charAt(0); }
    static boolean empty(String s) { return s == null || s.isEmpty(); }
    static boolean blank(String s) { return s == null || s.trim().isEmpty(); }
    static String concat(Object... xs) {
        StringBuilder b = new StringBuilder();
        for (Object x : xs) { b.append(x); }
        return b.toString();
    }
    static String readLine() {
        try {
            return new java.io.BufferedReader(new java.io.InputStreamReader(System.in)).readLine();
        } catch (java.io.IOException e) {
            return "";
        }
    }
    static String padLeft(String s, int n) {
        StringBuilder b = new StringBuilder(s);
        while (b.length() < n) { b.insert(0, ' '); }
        return b.toString();
    }
    static String padRight(String s, int n) {
        StringBuilder b = new StringBuilder(s);
        while (b.length() < n) { b.append(' '); }
        return b.toString();
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cs_to_java_print() {
        let src = r#"using System;

class Starter {
    static void Main(string[] args) {
        string name = "Ana";
        int age = 20;
        bool ok = true;
        int[] nums = {3, 7, 1};
        string[] names = {"Ana", "Luis"};
        Console.WriteLine("Hola " + name);
        for (int i = 0; i < names.Length; i++) {
            Console.WriteLine("  " + names[i]);
        }
        char c = name[0];
        int n = nums.Length;
        Console.WriteLine(name.ToUpper() + " " + c + " " + n + ok);
    }
}
"#;
        let java = cs_to_java(src);
        let before_helpers = java.split("\nclass Dbg").next().unwrap();
        assert_eq!(
            before_helpers.trim_end_matches('\n').lines().count(),
            src.lines().count()
        );
        assert!(java.contains("static void main(String[] args)"));
        assert!(java.contains("System.out.println"));
        assert!(java.contains("Dbg.len(names)"));
        assert!(java.contains("Dbg.charAt(name, 0)"));
        assert!(java.contains("String name = \"Ana\";"));
        assert!(java.contains("boolean ok = true;"));
        assert!(java.contains("name.toUpperCase()"));
    }
}
