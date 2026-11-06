use crate::runner;
use regex::Regex;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub enum Cmd {
    Step,
    Stop,
}

pub enum Event {
    Paused {
        line: usize,
        exec_line: Option<usize>,
        locals: Vec<(String, String)>,
    },
    Ended {
        output: String,
    },
    Failed {
        message: String,
    },
}

pub fn start(
    source: String,
    prefix_newlines: usize,
    lang: crate::codelang::CodeLang,
    cmd_rx: Receiver<Cmd>,
    ev_tx: Sender<Event>,
) {
    thread::spawn(move || {
        let result = match lang {
            crate::codelang::CodeLang::Python => {
                session_python(&source, prefix_newlines, &cmd_rx, &ev_tx)
            }
            crate::codelang::CodeLang::CSharp => {
                let src = crate::transpile::cs_to_java(&source);
                session(&src, prefix_newlines, &cmd_rx, &ev_tx)
            }
            crate::codelang::CodeLang::Java => {
                session(&source, prefix_newlines, &cmd_rx, &ev_tx)
            }
        };
        if let Err(message) = result {
            let _ = ev_tx.send(Event::Failed { message });
        }
    });
}

fn session(
    source: &str,
    prefix_newlines: usize,
    cmd_rx: &Receiver<Cmd>,
    ev_tx: &Sender<Event>,
) -> Result<(), String> {
    let work = prepare(source)?;
    let dir = work.path.clone();
    let mut dbg = spawn_debuggee(&dir)?;
    let port = match wait_port(&dbg.port_rx, Duration::from_secs(8)) {
        Ok(p) => p,
        Err(e) => {
            let _ = dbg.java.kill();
            return Err(e);
        }
    };

    let mut jdb = spawn_jdb(port)?;
    let out_rx = jdb.out_rx;
    let mut stdin = jdb.stdin;

    let ready = wait_contains(&out_rx, &["Initializing jdb", "VM Started"], Duration::from_secs(8))?;
    if ready.contains("Fatal error") || ready.contains("Unable to attach") {
        shutdown(&mut stdin, &mut jdb.child, &mut dbg.java);
        return Err(format!("jdb no se conectó:\n{}", ready.trim()));
    }

    send(&mut stdin, "stop in Starter.main")?;
    let _ = wait_contains(
        &out_rx,
        &["Set breakpoint", "Deferring breakpoint"],
        Duration::from_secs(5),
    )?;
    thread::sleep(Duration::from_millis(120));
    send(&mut stdin, "resume")?;
    let hit = wait_pause(&out_rx, Duration::from_secs(10))?;
    if hit.contains("The application exited") {
        let _ = ev_tx.send(Event::Ended {
            output: drain_program(&dbg.prog_rx),
        });
        shutdown(&mut stdin, &mut jdb.child, &mut dbg.java);
        return Ok(());
    }

    let at = parse_line(&hit).unwrap_or(prefix_newlines + 1);
    let names = crate::complete::declared_before(source, at);
    let locals = fetch_watch(&mut stdin, &out_rx, &names);
    let mut prev_full = emit_pause(&hit, locals, prefix_newlines, None, ev_tx);

    loop {
        match cmd_rx.recv_timeout(Duration::from_millis(150)) {
            Ok(Cmd::Stop) => {
                shutdown(&mut stdin, &mut jdb.child, &mut dbg.java);
                return Ok(());
            }
            Ok(Cmd::Step) => {
                send(&mut stdin, "next")?;
                let step = wait_pause(&out_rx, Duration::from_secs(8))?;
                if step.contains("The application exited") {
                    let _ = ev_tx.send(Event::Ended {
                        output: drain_program(&dbg.prog_rx),
                    });
                    shutdown(&mut stdin, &mut jdb.child, &mut dbg.java);
                    return Ok(());
                }
                thread::sleep(Duration::from_millis(40));
                let at = parse_line(&step).unwrap_or(prefix_newlines + 1);
                let names = crate::complete::declared_before(source, at);
                let locals = fetch_watch(&mut stdin, &out_rx, &names);
                prev_full = emit_pause(&step, locals, prefix_newlines, prev_full, ev_tx);
            }
            Err(RecvTimeoutError::Timeout) => {
                if let Ok(Some(status)) = dbg.java.try_wait() {
                    if !status.success() {
                        let _ = ev_tx.send(Event::Ended {
                            output: drain_program(&dbg.prog_rx),
                        });
                    } else {
                        let _ = ev_tx.send(Event::Ended {
                            output: drain_program(&dbg.prog_rx),
                        });
                    }
                    let _ = jdb.child.kill();
                    return Ok(());
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                shutdown(&mut stdin, &mut jdb.child, &mut dbg.java);
                return Ok(());
            }
        }
    }
}

fn map_line(full: usize, prefix_newlines: usize) -> usize {
    full.saturating_sub(prefix_newlines).max(1)
}

fn emit_pause(
    blob: &str,
    locals: Vec<(String, String)>,
    prefix_newlines: usize,
    prev_full: Option<usize>,
    ev_tx: &Sender<Event>,
) -> Option<usize> {
    let full = parse_line(blob).unwrap_or(prefix_newlines + 1);
    send_pause(full, locals, prefix_newlines, prev_full, ev_tx)
}

fn send_pause(
    full: usize,
    locals: Vec<(String, String)>,
    prefix_newlines: usize,
    prev_full: Option<usize>,
    ev_tx: &Sender<Event>,
) -> Option<usize> {
    let line = map_line(full, prefix_newlines);
    let exec_line = prev_full.map(|p| map_line(p, prefix_newlines));
    let _ = ev_tx.send(Event::Paused {
        line,
        exec_line,
        locals,
    });
    Some(full)
}

fn prepare(source: &str) -> Result<runner::TempWork, String> {
    match runner::compile_debug(source) {
        Ok(work) => Ok(work),
        Err(runner::CompileStatus::NoJdk) => Err("No encuentro JDK / jdb.".into()),
        Err(runner::CompileStatus::Ok) => Err("No pude preparar el debug.".into()),
        Err(runner::CompileStatus::Errors(errors)) => {
            let msg = errors
                .iter()
                .map(|e| format!("Línea {}: {}", e.line, e.message))
                .collect::<Vec<_>>()
                .join("\n");
            Err(if msg.is_empty() {
                "El código no compila. Completa los TODO y vuelve a intentar.".into()
            } else {
                format!("El código no compila:\n{msg}")
            })
        }
    }
}

struct Debuggee {
    java: Child,
    port_rx: Receiver<String>,
    prog_rx: Receiver<String>,
}

struct Jdb {
    child: Child,
    stdin: ChildStdin,
    out_rx: Receiver<String>,
}

fn spawn_debuggee(dir: &PathBuf) -> Result<Debuggee, String> {
    let mut cmd = Command::new("java");
    runner::harden(&mut cmd, dir);
    cmd.args([
            "-agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=127.0.0.1:0",
            "-Dfile.encoding=UTF-8",
            "Starter",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    hide_window(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| format!("No pude lanzar java: {e}"))?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let (port_tx, port_rx) = mpsc::channel();
    let (prog_tx, prog_rx) = mpsc::channel();
    if let Some(err) = stderr {
        let port_tx = port_tx.clone();
        let prog_tx = prog_tx.clone();
        thread::spawn(move || pump(err, Some(port_tx), prog_tx));
    }
    if let Some(out) = stdout {
        thread::spawn(move || pump(out, Some(port_tx), prog_tx));
    }
    Ok(Debuggee {
        java: child,
        port_rx,
        prog_rx,
    })
}

fn spawn_jdb(port: u16) -> Result<Jdb, String> {
    let connect = format!("com.sun.jdi.SocketAttach:hostname=127.0.0.1,port={port}");
    let mut cmd = Command::new("jdb");
    cmd.arg("-connect")
        .arg(connect)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    hide_window(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| format!("No pude abrir jdb: {e}"))?;
    let stdin = child.stdin.take().ok_or("jdb sin stdin")?;
    let (tx, rx) = mpsc::channel();
    if let Some(out) = child.stdout.take() {
        let tx2 = tx.clone();
        thread::spawn(move || pump(out, None, tx2));
    }
    if let Some(err) = child.stderr.take() {
        thread::spawn(move || pump(err, None, tx));
    }
    Ok(Jdb {
        child,
        stdin,
        out_rx: rx,
    })
}

fn pump(mut stream: impl Read, port_tx: Option<Sender<String>>, text_tx: Sender<String>) {
    let mut buf = [0u8; 2048];
    let mut acc = String::new();
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                acc.push_str(&chunk);
                if let Some(tx) = &port_tx {
                    if let Some(p) = parse_port(&acc) {
                        let _ = tx.send(p.to_string());
                    }
                }
                let _ = text_tx.send(chunk.into_owned());
            }
            Err(_) => break,
        }
    }
}

fn parse_port(text: &str) -> Option<u16> {
    let re = Regex::new(r"address:\s*(\d+)").ok()?;
    re.captures(text)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

fn wait_port(rx: &Receiver<String>, timeout: Duration) -> Result<u16, String> {
    match rx.recv_timeout(timeout) {
        Ok(s) => s.parse().map_err(|_| format!("Puerto JDWP inválido: {s}")),
        Err(_) => Err("Java no abrió el puerto de debug (JDWP).".into()),
    }
}

fn wait_pause(rx: &Receiver<String>, timeout: Duration) -> Result<String, String> {
    let mut acc = String::new();
    let start = Instant::now();
    while start.elapsed() < timeout {
        match rx.recv_timeout(Duration::from_millis(80)) {
            Ok(chunk) => {
                acc.push_str(&chunk);
                if acc.contains("The application exited")
                    || acc.contains("Exception occurred")
                    || acc.contains("Exception:")
                {
                    return Ok(acc);
                }
                let paused = acc.contains("Step completed") || acc.contains("Breakpoint hit");
                if paused && acc.contains("line=") {
                    return Ok(acc);
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    if acc.contains("Step completed") || acc.contains("Breakpoint hit") {
        return Ok(acc);
    }
    Err(format!(
        "El depurador no respondió a tiempo.\n{}",
        acc.trim()
    ))
}

fn wait_contains(rx: &Receiver<String>, markers: &[&str], timeout: Duration) -> Result<String, String> {
    let mut acc = String::new();
    let start = Instant::now();
    while start.elapsed() < timeout {
        match rx.recv_timeout(Duration::from_millis(80)) {
            Ok(chunk) => {
                acc.push_str(&chunk);
                if markers.iter().any(|m| acc.contains(m)) {
                    return Ok(acc);
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    Err(format!(
        "El depurador no respondió a tiempo.\n{}",
        acc.trim()
    ))
}

fn send(stdin: &mut ChildStdin, line: &str) -> Result<(), String> {
    writeln!(stdin, "{line}").map_err(|e| e.to_string())?;
    stdin.flush().map_err(|e| e.to_string())
}

fn parse_line(text: &str) -> Option<usize> {
    let re = Regex::new(r"line=(\d+)").ok()?;
    re.captures_iter(text)
        .last()
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

fn drain_rx(rx: &Receiver<String>) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(50) {
        match rx.recv_timeout(Duration::from_millis(12)) {
            Ok(_) => {}
            Err(_) => break,
        }
    }
}

fn read_idle(rx: &Receiver<String>, max: Duration) -> String {
    let mut acc = String::new();
    let start = Instant::now();
    let mut idle = 0u32;
    while start.elapsed() < max {
        match rx.recv_timeout(Duration::from_millis(20)) {
            Ok(chunk) => {
                acc.push_str(&chunk);
                idle = 0;
            }
            Err(RecvTimeoutError::Timeout) => {
                idle += 1;
                if idle >= 3 && !acc.is_empty() {
                    break;
                }
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    acc
}

fn fetch_watch(
    stdin: &mut ChildStdin,
    rx: &Receiver<String>,
    names: &[String],
) -> Vec<(String, String)> {
    drain_rx(rx);
    let mut map = std::collections::HashMap::new();
    let _ = send(stdin, "locals");
    for (k, v) in parse_locals(&read_idle(rx, Duration::from_millis(700))) {
        if names.iter().any(|n| n == &k) {
            map.insert(k, v);
        }
    }
    for name in names {
        drain_rx(rx);
        let _ = send(stdin, &format!("print {name}"));
        let blob = read_idle(rx, Duration::from_millis(500));
        if let Some(v) = value_for(&blob, name) {
            map.insert(name.clone(), v);
        }
    }
    names
        .iter()
        .filter_map(|n| map.get(n).map(|v| (n.clone(), v.clone())))
        .collect()
}

fn value_for(text: &str, name: &str) -> Option<String> {
    let re = Regex::new(&format!(
        r"(?m)(?:main\[\d+\]\s*)?{}\s*=\s*(.+)$",
        regex::escape(name)
    ))
    .ok()?;
    let mut last = None;
    for cap in re.captures_iter(text) {
        let v = cap.get(1)?.as_str().trim();
        if v.contains("Name unknown")
            || v.contains("ParseException")
            || v.contains("not a valid")
            || v.starts_with("instance of")
        {
            continue;
        }
        last = Some(v.to_string());
    }
    last
}

fn parse_locals(text: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"(?m)(?:^|\n)(?:main\[\d+\]\s*)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.+)$")
        .expect("locals re");
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for cap in re.captures_iter(text) {
        let k = cap.get(1).unwrap().as_str();
        if k == "args" || k == "line" || k == "bc" || k == "thread" {
            continue;
        }
        let v = cap.get(2).unwrap().as_str().trim().to_string();
        if v.contains("Name unknown")
            || v.contains("ParseException")
            || v.contains("not a valid")
            || v.contains("identifier")
            || v.starts_with("instance of")
        {
            continue;
        }
        if !seen.insert(k.to_string()) {
            if let Some(slot) = out.iter_mut().find(|(n, _)| n == k) {
                slot.1 = v;
            }
            continue;
        }
        out.push((k.to_string(), v));
    }
    out
}

fn drain_program(rx: &Receiver<String>) -> String {
    let mut s = String::new();
    while let Ok(chunk) = rx.try_recv() {
        if chunk.contains("Listening for transport") || chunk.contains("address:") {
            continue;
        }
        s.push_str(&chunk);
    }
    s
}

fn shutdown(stdin: &mut ChildStdin, jdb: &mut Child, java: &mut Child) {
    let _ = writeln!(stdin, "exit");
    let _ = stdin.flush();
    let _ = jdb.kill();
    let _ = java.kill();
    let _ = jdb.wait();
    let _ = java.wait();
}

fn hide_window(cmd: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
}

// ---------- Python (bdb) ----------

enum PyEvt {
    Pause { full: usize, locals: Vec<(String, String)> },
    Ended,
}

fn session_python(
    source: &str,
    prefix_newlines: usize,
    cmd_rx: &Receiver<Cmd>,
    ev_tx: &Sender<Event>,
) -> Result<(), String> {
    let Some((bin, extra)) = crate::codelang::python_invocation() else {
        return Err("No encuentro Python 3.".into());
    };
    let work = runner::TempWork::create().map_err(|e| e.to_string())?;
    std::fs::write(work.path.join("Starter.py"), source)
        .map_err(|e| format!("No pude escribir Starter.py: {e}"))?;
    std::fs::write(work.path.join("_ecdbg.py"), PY_HARNESS)
        .map_err(|e| format!("No pude escribir el depurador: {e}"))?;

    let mut cmd = Command::new(&bin);
    runner::harden(&mut cmd, &work.path);
    for a in &extra {
        cmd.arg(a);
    }
    cmd.args(["-I", "-X", "utf8", "_ecdbg.py", "Starter.py"])
        .env("PYTHONIOENCODING", "utf-8")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    hide_window(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| format!("No pude lanzar python: {e}"))?;
    let mut stdin = child.stdin.take().ok_or("python sin stdin")?;
    let (proto_tx, proto_rx) = mpsc::channel();
    let (prog_tx, prog_rx) = mpsc::channel();
    if let Some(err) = child.stderr.take() {
        thread::spawn(move || pump(err, None, proto_tx));
    }
    if let Some(out) = child.stdout.take() {
        thread::spawn(move || pump(out, None, prog_tx));
    }

    let mut acc = String::new();
    let mut tail = String::new();
    let mut prev_full = match py_next(&proto_rx, &mut acc, &mut tail, Duration::from_secs(10))? {
        PyEvt::Pause { full, locals } => send_pause(full, locals, prefix_newlines, None, ev_tx),
        PyEvt::Ended => {
            let _ = ev_tx.send(Event::Ended {
                output: py_output(&prog_rx, &tail),
            });
            let _ = child.wait();
            return Ok(());
        }
    };

    loop {
        match cmd_rx.recv_timeout(Duration::from_millis(150)) {
            Ok(Cmd::Stop) => {
                let _ = send(&mut stdin, "stop");
                let _ = child.kill();
                let _ = child.wait();
                return Ok(());
            }
            Ok(Cmd::Step) => {
                let _ = send(&mut stdin, "next");
                match py_next(&proto_rx, &mut acc, &mut tail, Duration::from_secs(8))? {
                    PyEvt::Pause { full, locals } => {
                        prev_full = send_pause(full, locals, prefix_newlines, prev_full, ev_tx);
                    }
                    PyEvt::Ended => {
                        let _ = ev_tx.send(Event::Ended {
                            output: py_output(&prog_rx, &tail),
                        });
                        let _ = child.wait();
                        return Ok(());
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                if let Ok(Some(_)) = child.try_wait() {
                    let _ = ev_tx.send(Event::Ended {
                        output: py_output(&prog_rx, &tail),
                    });
                    let _ = child.wait();
                    return Ok(());
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                let _ = send(&mut stdin, "stop");
                let _ = child.kill();
                let _ = child.wait();
                return Ok(());
            }
        }
    }
}

fn py_next(
    rx: &Receiver<String>,
    acc: &mut String,
    tail: &mut String,
    timeout: Duration,
) -> Result<PyEvt, String> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        while let Some(pos) = acc.find('\n') {
            let line = acc[..pos].trim_end_matches('\r').to_string();
            acc.drain(..=pos);
            if let Some(rest) = line.strip_prefix("@@PAUSE ") {
                let (n, js) = rest.split_once(' ').unwrap_or((rest, "{}"));
                let full = n.trim().parse().unwrap_or(1);
                return Ok(PyEvt::Pause {
                    full,
                    locals: parse_py_locals(js),
                });
            }
            if line.starts_with("@@END") {
                return Ok(PyEvt::Ended);
            }
            if let Some(rest) = line.strip_prefix("@@ERR ") {
                let (n, msg) = rest.split_once(' ').unwrap_or(("1", rest));
                return Err(format!("Línea {}: {}", n.trim(), msg.trim()));
            }
            tail.push_str(&line);
            tail.push('\n');
        }
        match rx.recv_timeout(Duration::from_millis(60)) {
            Ok(chunk) => acc.push_str(&chunk),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                tail.push_str(acc);
                acc.clear();
                return Ok(PyEvt::Ended);
            }
        }
    }
    Err(format!(
        "El depurador no respondió a tiempo.\n{}",
        tail.trim()
    ))
}

fn parse_py_locals(js: &str) -> Vec<(String, String)> {
    let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(js) else {
        return Vec::new();
    };
    map.iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("?").to_string()))
        .collect()
}

fn py_output(prog_rx: &Receiver<String>, tail: &str) -> String {
    format!("{}{}", drain_program(prog_rx), tail)
}

const PY_HARNESS: &str = r#"import bdb
import json
import os
import sys
import traceback


class Stepper(bdb.Bdb):
    def __init__(self, target):
        bdb.Bdb.__init__(self)
        self.target = target
        self.done = False

    def user_line(self, frame):
        if self.done:
            raise bdb.BdbQuit
        if frame.f_code.co_filename != self.target:
            self.set_step()
            return
        loc = {}
        for k, v in frame.f_locals.items():
            if k.startswith("__"):
                continue
            try:
                loc[k] = repr(v)
            except Exception:
                loc[k] = "?"
        sys.stderr.write("@@PAUSE %d %s\n" % (frame.f_lineno, json.dumps(loc)))
        sys.stderr.flush()
        cmd = sys.stdin.readline().strip()
        if cmd != "next":
            self.done = True
            raise bdb.BdbQuit
        self.set_step()


def main():
    target = os.path.abspath(sys.argv[1])
    globs = {"__name__": "__main__", "__file__": target}
    try:
        src = open(target, encoding="utf-8").read()
        code = compile(src, target, "exec")
    except SyntaxError as e:
        sys.stderr.write("@@ERR %s %s\n" % (e.lineno or 1, e.msg))
        sys.stderr.flush()
        return
    d = Stepper(target)
    d.set_step()
    try:
        d.runctx("exec(_code)", {"_code": code}, globs)
    except bdb.BdbQuit:
        pass
    except Exception:
        traceback.print_exc()
    sys.stderr.write("@@END\n")
    sys.stderr.flush()


main()
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jdwp_can_pause() {
        let src = r#"
public class Starter {
    public static void main(String[] args) {
        int x = 1;
        String name = "Ana";
        System.out.println(name);
    }
}
"#;
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (ev_tx, ev_rx) = mpsc::channel();
        start(src.into(), 3, crate::codelang::CodeLang::Java, cmd_rx, ev_tx);
        let ev = ev_rx.recv_timeout(Duration::from_secs(15)).expect("no debug event");
        match ev {
            Event::Paused { line, .. } => {
                assert!(line >= 1, "line={line}");
                let _ = cmd_tx.send(Cmd::Stop);
            }
            Event::Failed { message } => panic!("failed: {message}"),
            Event::Ended { output } => panic!("ended early: {output}"),
        }
    }

    #[test]
    fn jdwp_can_pause_csharp() {
        let src = r#"
using System;

class Starter {
    static void Main(string[] args) {
        int x = 1;
        string name = "Ana";
        Console.WriteLine(name);
    }
}
"#;
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (ev_tx, ev_rx) = mpsc::channel();
        start(
            src.into(),
            3,
            crate::codelang::CodeLang::CSharp,
            cmd_rx,
            ev_tx,
        );
        let ev = ev_rx.recv_timeout(Duration::from_secs(15)).expect("no debug event");
        match ev {
            Event::Paused { line, .. } => {
                assert!(line >= 1, "line={line}");
                let _ = cmd_tx.send(Cmd::Stop);
            }
            Event::Failed { message } => panic!("failed: {message}"),
            Event::Ended { output } => panic!("ended early: {output}"),
        }
    }

    #[test]
    fn bdb_can_pause_python() {
        if crate::codelang::python_invocation().is_none() {
            return;
        }
        let src = "x = 1\nname = \"Ana\"\nprint(name)\n";
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (ev_tx, ev_rx) = mpsc::channel();
        start(
            src.into(),
            0,
            crate::codelang::CodeLang::Python,
            cmd_rx,
            ev_tx,
        );
        let ev = ev_rx.recv_timeout(Duration::from_secs(15)).expect("no debug event");
        match ev {
            Event::Paused { line, locals, .. } => {
                assert_eq!(line, 1, "line={line}");
                assert!(locals.is_empty() || locals.iter().all(|(k, _)| !k.is_empty()));
                let _ = cmd_tx.send(Cmd::Stop);
            }
            Event::Failed { message } => panic!("failed: {message}"),
            Event::Ended { output } => panic!("ended early: {output}"),
        }
    }

    #[test]
    fn parse_locals_formats() {
        let raw = r#"
main[1] locals
firstName = ""
lastName = "dsa"
main[1] print favoriteColor
favoriteColor = "sad"
main[1] print missing
com.sun.tools.example.debug.expr.ParseException: Name unknown: missing
"#;
        let got = parse_locals(raw);
        assert_eq!(got.len(), 3, "{got:?}");
        assert_eq!(got[0], ("firstName".into(), r#""""#.into()));
        assert_eq!(got[1], ("lastName".into(), r#""dsa""#.into()));
        assert_eq!(got[2], ("favoriteColor".into(), r#""sad""#.into()));
    }
}
