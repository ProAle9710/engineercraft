use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Diag {
    pub line: usize,
    pub message: String,
    pub extra: String,
}

pub enum Outcome {
    NoJdk,
    CompileError { errors: Vec<Diag> },
    RuntimeError { line: Option<usize>, message: String, stdout: String },
    Timeout,
    FailedChecks { stdout: String, missing: Vec<String> },
    Success { stdout: String },
}

pub fn jdk_version() -> Option<String> {
    let mut cmd = Command::new("javac");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let out = cmd.arg("-version").output().ok()?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    text.lines()
        .map(str::trim)
        .find(|l| l.starts_with("javac"))
        .map(|l| l.replacen("javac", "JDK", 1))
}

pub fn lint(source: &str) -> Vec<Diag> {
    let mut errors = student_mistakes(source);
    match compile_source(source, "javacraft-lint") {
        CompileStatus::Ok | CompileStatus::NoJdk => errors,
        CompileStatus::Errors(javac) => {
            for e in javac {
                if !errors.iter().any(|a| a.line == e.line) {
                    errors.push(e);
                }
            }
            errors.sort_by_key(|e| e.line);
            errors
        }
    }
}

#[allow(dead_code)]
pub fn format_errors(errors: &[Diag]) -> String {
    let n = errors.len();
    let mut s = format!(
        "SINTAXIS  ·  {n} {}\n",
        if n == 1 { "error" } else { "errores" }
    );
    for e in errors {
        s.push_str(&format!("\nLínea {}  ·  {}\n", e.line, e.message));
        if !e.extra.is_empty() {
            s.push_str(&e.extra);
            s.push('\n');
        }
        s.push_str(explain(&e.message));
        s.push('\n');
    }
    s
}

pub enum CompileStatus {
    Ok,
    NoJdk,
    Errors(Vec<Diag>),
}

pub fn compile_debug(source: &str) -> CompileStatus {
    compile_source(source, "javacraft-debug")
}

fn compile_source(source: &str, folder: &str) -> CompileStatus {
    if jdk_version().is_none() {
        return CompileStatus::NoJdk;
    }
    let dir = std::env::temp_dir().join(folder);
    if fs::create_dir_all(&dir).is_err() {
        return CompileStatus::Errors(vec![Diag {
            line: 1,
            message: "No pude crear el directorio temporal.".into(),
            extra: String::new(),
        }]);
    }
    if fs::write(dir.join("Starter.java"), source).is_err() {
        return CompileStatus::Errors(vec![Diag {
            line: 1,
            message: "No pude escribir Starter.java.".into(),
            extra: String::new(),
        }]);
    }
    let compile = command("javac")
        .current_dir(&dir)
        .args(["-g", "-encoding", "UTF-8", "Starter.java"])
        .output();
    let compile = match compile {
        Ok(o) => o,
        Err(_) => return CompileStatus::NoJdk,
    };
    if compile.status.success() {
        CompileStatus::Ok
    } else {
        CompileStatus::Errors(parse_javac(&String::from_utf8_lossy(&compile.stderr)))
    }
}

fn student_mistakes(source: &str) -> Vec<Diag> {
    let mut out = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let n = i + 1;
        let t = line.trim_start();
        if t.starts_with("let ") {
            out.push(Diag {
                line: n,
                message: "let no existe en Java".into(),
                extra: "Es JavaScript. Aquí: String name = \"Ana\";".into(),
            });
        } else if t.starts_with("var ") {
            out.push(Diag {
                line: n,
                message: "no uses var en esta clase".into(),
                extra: "Escribe el tipo: String name = \"Ana\";  o  int age = 18;".into(),
            });
        } else if t.contains("console.log") {
            out.push(Diag {
                line: n,
                message: "console.log no existe en Java".into(),
                extra: "Usa System.out.println(...);".into(),
            });
        }
    }
    out
}

pub fn evaluate(source: &str, needles: &[String]) -> Outcome {
    if jdk_version().is_none() {
        return Outcome::NoJdk;
    }

    let dir = work_dir();
    match compile_source(source, "javacraft-run") {
        CompileStatus::NoJdk => return Outcome::NoJdk,
        CompileStatus::Errors(errors) => return Outcome::CompileError { errors },
        CompileStatus::Ok => {}
    }

    match run_java(&dir, Duration::from_secs(8)) {
        RunJava::Timeout => Outcome::Timeout,
        RunJava::Failed { stdout, stderr } => {
            let (line, message) = parse_runtime(&stderr);
            Outcome::RuntimeError { line, message, stdout }
        }
        RunJava::Ok { stdout } => {
            let missing: Vec<String> = needles
                .iter()
                .filter(|n| !stdout.contains(n.as_str()))
                .cloned()
                .collect();
            if missing.is_empty() {
                Outcome::Success { stdout }
            } else {
                Outcome::FailedChecks { stdout, missing }
            }
        }
    }
}

enum RunJava {
    Ok { stdout: String },
    Failed { stdout: String, stderr: String },
    Timeout,
}

fn run_java(dir: &PathBuf, timeout: Duration) -> RunJava {
    let mut child = match command("java")
        .current_dir(dir)
        .args(["-Dfile.encoding=UTF-8", "Starter"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            return RunJava::Failed {
                stdout: String::new(),
                stderr: "No pude ejecutar java.".into(),
            };
        }
    };

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout = String::new();
                let mut stderr = String::new();
                if let Some(mut s) = child.stdout.take() {
                    let mut buf = Vec::new();
                    let _ = s.read_to_end(&mut buf);
                    stdout = String::from_utf8_lossy(&buf).into_owned();
                }
                if let Some(mut s) = child.stderr.take() {
                    let mut buf = Vec::new();
                    let _ = s.read_to_end(&mut buf);
                    stderr = String::from_utf8_lossy(&buf).into_owned();
                }
                if status.success() {
                    return RunJava::Ok { stdout };
                }
                return RunJava::Failed { stdout, stderr };
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return RunJava::Timeout;
                }
                std::thread::sleep(Duration::from_millis(40));
            }
            Err(e) => {
                return RunJava::Failed {
                    stdout: String::new(),
                    stderr: e.to_string(),
                };
            }
        }
    }
}

fn parse_javac(stderr: &str) -> Vec<Diag> {
    let re = Regex::new(r"(?m)^(.+\.java):(\d+): error: (.+)$").expect("regex javac");
    let lines: Vec<&str> = stderr.lines().collect();
    let mut errors = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        if let Some(c) = re.captures(line) {
            let line_no = c.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
            let message = c.get(3).map(|m| m.as_str().to_string()).unwrap_or_default();
            let mut extra = String::new();
            for follow in lines.iter().skip(idx + 1).take(6) {
                if follow.contains(".java:") && follow.contains("error:") {
                    break;
                }
                if follow.trim().is_empty() {
                    continue;
                }
                extra.push_str(follow.trim());
                extra.push('\n');
            }
            errors.push(Diag {
                line: line_no,
                message,
                extra: extra.trim().to_string(),
            });
        }
    }

    if errors.is_empty() && !stderr.trim().is_empty() {
        errors.push(Diag {
            line: 1,
            message: stderr.trim().to_string(),
            extra: String::new(),
        });
    }
    errors
}

fn parse_runtime(stderr: &str) -> (Option<usize>, String) {
    let exception = stderr
        .lines()
        .find(|l| l.contains("Exception") || l.contains("Error"))
        .unwrap_or(stderr.trim())
        .trim()
        .to_string();
    let line_re = Regex::new(r"Starter\.java:(\d+)").expect("regex runtime line");
    let line = line_re
        .captures(stderr)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());
    (line, exception)
}

pub fn explain(message: &str) -> &'static str {
    let m = message.to_ascii_lowercase();
    if m.contains("cannot find symbol") {
        "Esa variable o método no existe. Créala en el TODO, con tipo y nombre: String nombre = \"Ana\";"
    } else if m.contains("let no existe") {
        "let es de JavaScript. En Java: String nombre = \"Ana\";"
    } else if m.contains("no uses var") {
        "No uses var. Escribe el tipo: int age = 18;"
    } else if m.contains("console.log") {
        "En Java se imprime con System.out.println(...);"
    } else if m.contains("';' expected") || m.contains("';' esperado") {
        "Falta un punto y coma al final de la línea."
    } else if m.contains("not a statement") {
        "Java no reconoce esa línea como instrucción. Revisa la sintaxis (tipo nombre = valor;)."
    } else if m.contains("reached end of file") {
        "Falta un } de cierre. Cuenta las llaves { }."
    } else if m.contains("incompatible types") {
        "El tipo no coincide. int va con números, String con texto entre comillas, boolean con true/false."
    } else if m.contains("array required") {
        "Estás usando [índice] en algo que no es un arreglo."
    } else if m.contains("variable") && m.contains("might not have been initialized") {
        "La variable se declaró pero no tiene valor antes de usarla."
    } else if m.contains("class starter is public") {
        "La clase pública debe llamarse Starter y estar en Starter.java. No cambies el nombre de la clase."
    } else if m.contains("illegal start") {
        "Hay un símbolo de más o de menos. Revisa paréntesis, comillas y llaves."
    } else if m.contains("else without if") {
        "Hay un else sin su if. No borres el if de arriba."
    } else if m.contains("arrayindexoutofbounds") {
        "El índice está fuera del arreglo. El primero es [0] y el último es length - 1."
    } else if m.contains("nullpointer") {
        "Estás usando un objeto que vale null. Asigna el valor antes de llamarle un método."
    } else if m.contains("string index out of range") {
        "charAt() se salió del texto. El primer carácter es charAt(0)."
    } else {
        "Lee el mensaje de javac: te dice el archivo, la línea y qué no entiende."
    }
}

fn work_dir() -> PathBuf {
    std::env::temp_dir().join("javacraft-run")
}

fn command(name: &str) -> Command {
    let mut cmd = Command::new(name);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}
