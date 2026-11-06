#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Es,
    En,
}

impl Lang {
    pub fn from_saved(s: &str) -> Self {
        if s.eq_ignore_ascii_case("en") {
            Lang::En
        } else {
            Lang::Es
        }
    }

    pub fn as_saved(self) -> &'static str {
        match self {
            Lang::Es => "es",
            Lang::En => "en",
        }
    }

    pub fn other(self) -> Self {
        match self {
            Lang::Es => Lang::En,
            Lang::En => Lang::Es,
        }
    }

    pub fn switching_to(self) -> &'static str {
        match self {
            Lang::Es => "Cambiando a español…",
            Lang::En => "Switching to English…",
        }
    }
}

pub fn tr(lang: Lang, es: &'static str, en: &'static str) -> &'static str {
    match lang {
        Lang::Es => es,
        Lang::En => en,
    }
}

const COMMENT_PAIRS: &[(&str, &str)] = &[
    ("Create a String variable called", "Crea una variable String llamada"),
    ("Create a string variable called", "Crea una variable string llamada"),
    ("Create a String variable", "Crea una variable String"),
    ("Create a string variable", "Crea una variable string"),
    ("Create String[]", "Crea String[]"),
    ("Create int[]", "Crea int[]"),
    ("Create boolean", "Crea boolean"),
    ("Create String", "Crea String"),
    ("Create int", "Crea int"),
    ("Do not change the lines below", "No cambies las líneas de abajo"),
    ("Do not change", "No cambies"),
    ("Do not only copy the examples", "No copies solo los ejemplos"),
    ("READ the EXAMPLE blocks first. They already run.", "LEE primero los bloques EJEMPLO. Ya se ejecutan."),
    ("Then complete YOUR TURN (TODO).", "Luego completa TU TURNO (TODO)."),
    ("YOUR TURN — write YOUR loops", "TU TURNO — escribe TUS ciclos"),
    ("YOUR TURN", "TU TURNO"),
    ("Look at EXAMPLE", "Mira el EJEMPLO"),
    ("Look at", "Mira"),
    ("Write the for yourself", "Escribe el for tú mismo"),
    ("with a for loop", "con un ciclo for"),
    ("with a while loop", "con un ciclo while"),
    ("Print each", "Imprime cada"),
    ("Write your code ONLY in the TODO sections.", "Escribe tu código SOLO en las secciones TODO."),
    ("Correct syntax:", "Sintaxis correcta:"),
    ("Three parts: start ; condition ; step", "Tres partes: inicio ; condición ; paso"),
    ("Either loop can do any job", "Cualquier ciclo puede hacer el trabajo"),
    ("typical when you know how many times", "típico cuando sabes cuántas veces"),
    ("typical until a condition changes", "típico hasta que cambia una condición"),
    ("Example:", "Ejemplo:"),
    ("EXAMPLE", "EJEMPLO"),
    ("Use new variable names", "Usa nombres de variable nuevos"),
    ("keep going WHILE", "sigue MIENTRAS"),
    ("You MUST write", "DEBES escribir"),
    ("or the loop never ends", "o el ciclo no termina"),
    ("Not the usual choice", "No es la opción habitual"),
    ("still correct", "sigue siendo correcto"),
    ("Same countdown", "La misma cuenta atrás"),
    ("Print each city", "Imprime cada ciudad"),
    ("Print each number", "Imprime cada número"),
    ("then subtract", "luego resta"),
    ("Search id", "Busca el id"),
    ("Compute average", "Calcula el promedio"),
    ("Fill lineTotal", "Llena lineTotal"),
    ("Count scores", "Cuenta los scores"),
    ("Copy passing names", "Copia los nombres que aprueban"),
];

pub fn translate_comments(code: &str, to: Lang) -> String {
    let mut out = String::with_capacity(code.len());
    for (i, line) in code.lines().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&translate_comment_line(line, to));
    }
    if code.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn translate_comment_line(line: &str, to: Lang) -> String {
    let trimmed = line.trim_start();
    let marker = comment_marker(trimmed);
    let Some(slash) = marker else {
        return line.to_string();
    };
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];
    let mut body = trimmed[slash.len()..].to_string();
    for (en, es) in COMMENT_PAIRS {
        let (from, dest) = match to {
            Lang::Es => (*en, *es),
            Lang::En => (*es, *en),
        };
        if from != dest {
            body = body.replace(from, dest);
        }
    }
    format!("{indent}{slash}{body}")
}

fn comment_marker(trimmed: &str) -> Option<&'static str> {
    if trimmed.starts_with("#!") {
        return None;
    }
    if trimmed.starts_with("// ") {
        Some("// ")
    } else if trimmed.starts_with("//") {
        Some("//")
    } else if trimmed.starts_with("# ") {
        Some("# ")
    } else if trimmed.starts_with('#') {
        Some("#")
    } else {
        None
    }
}
