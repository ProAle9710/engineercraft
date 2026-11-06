use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static SCRATCH: AtomicU64 = AtomicU64::new(1);

pub struct TempWork {
    pub path: PathBuf,
}

impl TempWork {
    pub fn create() -> Result<Self, String> {
        let n = SCRATCH.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("ec-{}-{n}", std::process::id()));
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        Ok(Self { path })
    }
}

impl Drop for TempWork {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub(crate) fn harden(cmd: &mut Command, cwd: &Path) {
    cmd.current_dir(cwd);
    cmd.env_clear();
    if let Ok(p) = std::env::var("PATH") {
        cmd.env("PATH", p);
    }
    if let Ok(r) = std::env::var("SystemRoot") {
        cmd.env("SystemRoot", &r);
        cmd.env("WINDIR", r);
    }
    if let Ok(w) = std::env::var("WINDIR") {
        cmd.env("WINDIR", w);
    }
    cmd.env("TEMP", cwd);
    cmd.env("TMP", cwd);
    cmd.env("TMPDIR", cwd);
    cmd.env("HOME", cwd);
    cmd.env("USERPROFILE", cwd);
    cmd.env("APPDATA", cwd);
    cmd.env("LOCALAPPDATA", cwd);
    cmd.env("PYTHONSAFEPATH", "1");
    cmd.env("PYTHONDONTWRITEBYTECODE", "1");
    cmd.env("DOTNET_CLI_HOME", cwd);
    cmd.env("DOTNET_CLI_TELEMETRY_OPTOUT", "1");
    cmd.env("COMPlus_EnableDiagnostics", "0");
}

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

pub fn lint(source: &str, lang: crate::codelang::CodeLang) -> Vec<Diag> {
    match lang {
        crate::codelang::CodeLang::Java => lint_java(source),
        crate::codelang::CodeLang::CSharp => lint_csharp(source),
        crate::codelang::CodeLang::Python => lint_python(source),
    }
}

fn lint_java(source: &str) -> Vec<Diag> {
    let mut errors = student_mistakes(source);
    let Ok(work) = TempWork::create() else {
        return errors;
    };
    match compile_in(&work.path, source) {
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
pub fn format_errors(errors: &[Diag], lang: crate::codelang::CodeLang) -> String {
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
        s.push_str(explain(&e.message, lang));
        s.push('\n');
    }
    s
}

pub enum CompileStatus {
    Ok,
    NoJdk,
    Errors(Vec<Diag>),
}

pub fn compile_debug(source: &str) -> Result<TempWork, CompileStatus> {
    let work = TempWork::create().map_err(|_| CompileStatus::Errors(vec![Diag {
        line: 1,
        message: "No pude crear el directorio temporal.".into(),
        extra: String::new(),
    }]))?;
    match compile_in(&work.path, source) {
        CompileStatus::Ok => Ok(work),
        other => Err(other),
    }
}

fn compile_in(dir: &Path, source: &str) -> CompileStatus {
    if jdk_version().is_none() {
        return CompileStatus::NoJdk;
    }
    if fs::write(dir.join("Starter.java"), source).is_err() {
        return CompileStatus::Errors(vec![Diag {
            line: 1,
            message: "No pude escribir Starter.java.".into(),
            extra: String::new(),
        }]);
    }
    let mut compile = command("javac");
    harden(&mut compile, dir);
    let compile = compile
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

fn mistakes_cs(source: &str) -> Vec<Diag> {
    let mut out = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let n = i + 1;
        let t = line.trim_start();
        if t.contains("System.out.println") {
            out.push(Diag {
                line: n,
                message: "System.out.println no existe en C#".into(),
                extra: "Usa Console.WriteLine(...);".into(),
            });
        } else if t.starts_with("let ") {
            out.push(Diag {
                line: n,
                message: "let no existe en C#".into(),
                extra: "Escribe el tipo: string name = \"Ana\";".into(),
            });
        }
    }
    out
}

fn mistakes_py(source: &str) -> Vec<Diag> {
    let mut out = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let n = i + 1;
        let t = line.trim_start();
        if t.contains("System.out.println") || t.contains("Console.WriteLine") {
            out.push(Diag {
                line: n,
                message: "Eso no es Python".into(),
                extra: "Usa print(...)".into(),
            });
        } else if t.starts_with("String ") || t.starts_with("string ") {
            out.push(Diag {
                line: n,
                message: "Python no declara el tipo".into(),
                extra: String::new(),
            });
        }
    }
    out
}

pub fn evaluate(
    source: &str,
    needles: &[String],
    lang: crate::codelang::CodeLang,
) -> Outcome {
    match lang {
        crate::codelang::CodeLang::Java => evaluate_java(source, needles),
        crate::codelang::CodeLang::CSharp => evaluate_dotnet(source, needles),
        crate::codelang::CodeLang::Python => evaluate_python(source, needles),
    }
}

fn evaluate_java(source: &str, needles: &[String]) -> Outcome {
    if jdk_version().is_none() {
        return Outcome::NoJdk;
    }

    let Ok(work) = TempWork::create() else {
        return Outcome::CompileError {
            errors: vec![Diag {
                line: 1,
                message: "No pude crear el directorio temporal.".into(),
                extra: String::new(),
            }],
        };
    };
    match compile_in(&work.path, source) {
        CompileStatus::NoJdk => return Outcome::NoJdk,
        CompileStatus::Errors(errors) => return Outcome::CompileError { errors },
        CompileStatus::Ok => {}
    }

    match run_java(&work.path, Duration::from_secs(8)) {
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

fn run_java(dir: &Path, timeout: Duration) -> RunJava {
    let mut cmd = command("java");
    harden(&mut cmd, dir);
    let mut child = match cmd
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

pub fn explain(message: &str, lang: crate::codelang::CodeLang) -> &'static str {
    let m = message.to_ascii_lowercase();
    match lang {
        crate::codelang::CodeLang::Python => {
            if m.contains("is not defined") || m.contains("nameerror") {
                "Esa variable no existe. En Python se asigna con =  (no con :). Ejemplo: firstName = \"Ana\""
            } else if m.contains("syntaxerror") {
                "Python no entiende esa línea. Revisa : indentación, = y paréntesis."
            } else if m.contains("usa = no") || m.contains("no declara el tipo") {
                "En Python: nombre = \"valor\"  — sin tipo y sin punto y coma."
            } else {
                "Lee el mensaje de Python: te dice el archivo, la línea y qué no entiende."
            }
        }
        crate::codelang::CodeLang::CSharp => {
            if m.contains("does not exist") || m.contains("cs0103") {
                "Esa variable no existe. Créala: string nombre = \"Ana\";"
            } else if m.contains("';' expected") {
                "Falta un punto y coma al final de la línea."
            } else {
                "Lee el mensaje del compilador de C# (dotnet): línea y qué no entiende."
            }
        }
        crate::codelang::CodeLang::Java => {
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
    }
}

fn check_needles(stdout: String, needles: &[String]) -> Outcome {
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

fn evaluate_dotnet(source: &str, needles: &[String]) -> Outcome {
    if crate::codelang::detect(crate::codelang::CodeLang::CSharp).is_none() {
        return Outcome::NoJdk;
    }
    let Ok(work) = TempWork::create() else {
        return Outcome::CompileError {
            errors: vec![Diag {
                line: 1,
                message: "No pude crear el directorio temporal.".into(),
                extra: String::new(),
            }],
        };
    };
    if write_cs(&work.path, source).is_err() {
        return Outcome::CompileError {
            errors: vec![Diag {
                line: 1,
                message: "No pude escribir Starter.cs.".into(),
                extra: String::new(),
            }],
        };
    }
    let mut compile = command("dotnet");
    compile.current_dir(&work.path);
    let compile = compile.args(["build", "-nologo", "-v", "q"]).output();
    let compile = match compile {
        Ok(o) => o,
        Err(_) => return Outcome::NoJdk,
    };
    if !compile.status.success() {
        let err = format!(
            "{}{}",
            String::from_utf8_lossy(&compile.stdout),
            String::from_utf8_lossy(&compile.stderr)
        );
        return Outcome::CompileError {
            errors: parse_dotnet(&err),
        };
    }
    let mut run = command("dotnet");
    harden(&mut run, &work.path);
    run.args(["run", "--no-build", "-nologo"]);
    match run_cmd(run, Duration::from_secs(8)) {
        RunJava::Timeout => Outcome::Timeout,
        RunJava::Failed { stdout, stderr } => Outcome::RuntimeError {
            line: parse_cs_runtime(&stderr).0,
            message: parse_cs_runtime(&stderr).1,
            stdout,
        },
        RunJava::Ok { stdout } => check_needles(stdout, needles),
    }
}

fn evaluate_python(source: &str, needles: &[String]) -> Outcome {
    let Some((bin, extra)) = crate::codelang::python_invocation() else {
        return Outcome::NoJdk;
    };
    let Ok(work) = TempWork::create() else {
        return Outcome::CompileError {
            errors: vec![Diag {
                line: 1,
                message: "No pude crear el directorio temporal.".into(),
                extra: String::new(),
            }],
        };
    };
    if fs::write(work.path.join("Starter.py"), source).is_err() {
        return Outcome::CompileError {
            errors: vec![Diag {
                line: 1,
                message: "No pude escribir Starter.py.".into(),
                extra: String::new(),
            }],
        };
    }
    let mut cmd = command(&bin);
    harden(&mut cmd, &work.path);
    for a in &extra {
        cmd.arg(a);
    }
    cmd.arg("-I");
    cmd.arg("Starter.py");
    match run_cmd(cmd, Duration::from_secs(8)) {
        RunJava::Timeout => Outcome::Timeout,
        RunJava::Failed { stdout, stderr } => {
            let (line, message) = parse_python(&stderr);
            let line = Some(relocate_py(source, line, &message));
            if stderr.to_ascii_lowercase().contains("syntaxerror") {
                Outcome::CompileError {
                    errors: vec![Diag {
                        line: line.unwrap_or(1),
                        message,
                        extra: String::new(),
                    }],
                }
            } else {
                Outcome::RuntimeError {
                    line,
                    message,
                    stdout,
                }
            }
        }
        RunJava::Ok { stdout } => check_needles(stdout, needles),
    }
}

fn write_cs(dir: &Path, source: &str) -> std::io::Result<()> {
    let csproj = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <RollForward>LatestMajor</RollForward>
    <StartupObject>Starter</StartupObject>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>disable</Nullable>
  </PropertyGroup>
</Project>
"#;
    fs::write(dir.join("Starter.csproj"), csproj)?;
    fs::write(dir.join("Starter.cs"), source)
}

fn lint_csharp(source: &str) -> Vec<Diag> {
    let mut errors = mistakes_cs(source);
    let Ok(work) = TempWork::create() else {
        return errors;
    };
    if write_cs(&work.path, source).is_err() {
        return errors;
    }
    let mut compile = command("dotnet");
    compile.current_dir(&work.path);
    let Ok(out) = compile.args(["build", "-nologo", "-v", "q"]).output() else {
        return errors;
    };
    if !out.status.success() {
        let err = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        for e in parse_dotnet(&err) {
            if !errors.iter().any(|a| a.line == e.line) {
                errors.push(e);
            }
        }
        errors.sort_by_key(|e| e.line);
    }
    errors
}

fn lint_python(source: &str) -> Vec<Diag> {
    let mut errors = mistakes_py(source);
    let Some((bin, extra)) = crate::codelang::python_invocation() else {
        return errors;
    };
    let Ok(work) = TempWork::create() else {
        return errors;
    };
    let file = work.path.join("Starter.py");
    if fs::write(&file, source).is_err() {
        return errors;
    }
    let mut cmd = command(&bin);
    harden(&mut cmd, &work.path);
    for a in extra {
        cmd.arg(a);
    }
    cmd.args(["-I", "-m", "py_compile"]).arg(&file);
    let Ok(out) = cmd.output() else {
        return errors;
    };
    if out.status.success() {
        return errors;
    }
    let err = String::from_utf8_lossy(&out.stderr);
    let (line, message) = parse_python(&err);
    errors.push(Diag {
        line: line.unwrap_or(1),
        message,
        extra: String::new(),
    });
    errors
}

fn ident_not_defined(message: &str) -> Option<String> {
    let re = Regex::new(r"name '([^']+)' is not defined").ok()?;
    Some(re.captures(message)?.get(1)?.as_str().to_string())
}

fn relocate_py(source: &str, line: Option<usize>, message: &str) -> usize {
    if let Some(name) = ident_not_defined(message) {
        let pat = format!(r"(?m)^\s*{}\s*[:=]", regex::escape(&name));
        if let Ok(re) = Regex::new(&pat) {
            if let Some(n) = source
                .lines()
                .enumerate()
                .filter(|(_, l)| re.is_match(l))
                .map(|(i, _)| i + 1)
                .next()
            {
                return n;
            }
        }
    }
    line.unwrap_or(1)
}

fn parse_dotnet(stderr: &str) -> Vec<Diag> {
    let re = Regex::new(r"\((\d+),\d+\):\s*error\s+\w+:\s*(.+)$").expect("cs");
    let mut errors = Vec::new();
    for line in stderr.lines() {
        if let Some(c) = re.captures(line) {
            errors.push(Diag {
                line: c.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1),
                message: c.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default(),
                extra: String::new(),
            });
        }
    }
    if errors.is_empty() && !stderr.trim().is_empty() {
        errors.push(Diag {
            line: 1,
            message: stderr.trim().lines().next().unwrap_or("error").to_string(),
            extra: String::new(),
        });
    }
    errors
}

fn parse_cs_runtime(stderr: &str) -> (Option<usize>, String) {
    let re = Regex::new(r"Starter\.cs:line\s+(\d+)").expect("cs rt");
    let line = re
        .captures(stderr)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());
    let message = stderr
        .lines()
        .find(|l| l.contains("Exception") || l.contains("Error"))
        .unwrap_or(stderr.trim())
        .trim()
        .to_string();
    (line, message)
}

fn parse_python(stderr: &str) -> (Option<usize>, String) {
    let re = Regex::new(r#"line (\d+)"#).expect("py");
    let line = re
        .captures(stderr)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());
    let message = stderr
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or(stderr.trim())
        .trim()
        .to_string();
    (line, message)
}

fn run_cmd(mut cmd: Command, timeout: Duration) -> RunJava {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => {
            return RunJava::Failed {
                stdout: String::new(),
                stderr: "No pude ejecutar el comando.".into(),
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

pub(crate) fn command(name: &str) -> Command {
    let mut cmd = Command::new(name);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}
