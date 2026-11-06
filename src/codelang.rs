use crate::i18n::Lang;
use crate::runner;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CodeLang {
    Java,
    CSharp,
    Python,
}

impl CodeLang {
    pub const ALL: [Self; 3] = [Self::Java, Self::CSharp, Self::Python];

    pub fn id(self) -> &'static str {
        match self {
            Self::Java => "java",
            Self::CSharp => "csharp",
            Self::Python => "python",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Java => "Java",
            Self::CSharp => "C#",
            Self::Python => "Python",
        }
    }

    pub fn from_saved(s: &str) -> Self {
        match s {
            "csharp" | "cs" | "c#" => Self::CSharp,
            "python" | "py" => Self::Python,
            _ => Self::Java,
        }
    }

    pub fn draft_key(self, exercise_id: &str) -> String {
        format!("{}:{exercise_id}", self.id())
    }

    pub fn ext(self) -> &'static str {
        match self {
            Self::Java => "java",
            Self::CSharp => "cs",
            Self::Python => "py",
        }
    }
}

#[derive(Clone)]
pub struct Toolchain {
    pub label: String,
}

pub fn detect(lang: CodeLang) -> Option<Toolchain> {
    match lang {
        CodeLang::Java => runner::jdk_version().map(|label| Toolchain { label }),
        CodeLang::CSharp => dotnet_version().map(|v| Toolchain {
            label: format!(".NET {v}"),
        }),
        CodeLang::Python => python_version().map(|label| Toolchain { label }),
    }
}

fn dotnet_version() -> Option<String> {
    let out = runner::command("dotnet").arg("--version").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let v = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}

fn python_version() -> Option<String> {
    python_probe().map(|(_, _, label)| label)
}

pub fn python_invocation() -> Option<(String, Vec<String>)> {
    python_probe().map(|(bin, args, _)| (bin, args))
}

fn python_probe() -> Option<(String, Vec<String>, String)> {
    let candidates: &[(&str, &[&str])] = &[
        ("python", &["--version"]),
        ("python3", &["--version"]),
        ("py", &["-3", "--version"]),
    ];
    for (bin, args) in candidates {
        let out = runner::command(bin).args(*args).output().ok();
        let Some(out) = out else { continue };
        if !out.status.success() {
            continue;
        }
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let line = text.lines().map(str::trim).find(|l| l.contains("Python"))?;
        let extra: Vec<String> = if *bin == "py" {
            vec!["-3".into()]
        } else {
            Vec::new()
        };
        return Some(((*bin).into(), extra, line.to_string()));
    }
    None
}

pub struct Guide {
    pub title: &'static str,
    pub why: &'static str,
    pub official: &'static str,
    pub official_label: &'static str,
    pub cmd: &'static str,
    pub docs: &'static str,
}

pub fn guide(lang: CodeLang, ui: Lang) -> Guide {
    let win = cfg!(windows);
    let mac = cfg!(target_os = "macos");
    match (lang, ui, win, mac) {
        (CodeLang::Java, Lang::Es, true, _) => Guide {
            title: "Java (JDK 17 o superior)",
            why: "Los ejercicios de Java se compilan con javac y se ejecutan con java. Sin JDK no hay lista de tareas ni CORRER.",
            official: "https://adoptium.net/",
            official_label: "adoptium.net — Eclipse Temurin",
            cmd: "winget install EclipseAdoptium.Temurin.17.JDK",
            docs: "https://learn.microsoft.com/windows/package-manager/winget/",
        },
        (CodeLang::Java, Lang::En, true, _) => Guide {
            title: "Java (JDK 17+)",
            why: "Java tasks compile with javac and run with java. Without a JDK there is no exercise list and RUN is disabled.",
            official: "https://adoptium.net/",
            official_label: "adoptium.net — Eclipse Temurin",
            cmd: "winget install EclipseAdoptium.Temurin.17.JDK",
            docs: "https://learn.microsoft.com/windows/package-manager/winget/",
        },
        (CodeLang::Java, Lang::Es, _, true) => Guide {
            title: "Java (JDK 17 o superior)",
            why: "Los ejercicios de Java se compilan con javac y se ejecutan con java. Sin JDK no hay lista de tareas ni CORRER.",
            official: "https://adoptium.net/",
            official_label: "adoptium.net — Eclipse Temurin",
            cmd: "brew install openjdk@17",
            docs: "https://formulae.brew.sh/formula/openjdk@17",
        },
        (CodeLang::Java, Lang::En, _, true) => Guide {
            title: "Java (JDK 17+)",
            why: "Java tasks compile with javac and run with java. Without a JDK there is no exercise list and RUN is disabled.",
            official: "https://adoptium.net/",
            official_label: "adoptium.net — Eclipse Temurin",
            cmd: "brew install openjdk@17",
            docs: "https://formulae.brew.sh/formula/openjdk@17",
        },
        (CodeLang::Java, Lang::Es, _, _) => Guide {
            title: "Java (JDK 17 o superior)",
            why: "Los ejercicios de Java se compilan con javac y se ejecutan con java. Sin JDK no hay lista de tareas ni CORRER.",
            official: "https://adoptium.net/",
            official_label: "adoptium.net — Eclipse Temurin",
            cmd: "sudo apt install openjdk-17-jdk",
            docs: "https://openjdk.org/install/",
        },
        (CodeLang::Java, Lang::En, _, _) => Guide {
            title: "Java (JDK 17+)",
            why: "Java tasks compile with javac and run with java. Without a JDK there is no exercise list and RUN is disabled.",
            official: "https://adoptium.net/",
            official_label: "adoptium.net — Eclipse Temurin",
            cmd: "sudo apt install openjdk-17-jdk",
            docs: "https://openjdk.org/install/",
        },
        (CodeLang::CSharp, Lang::Es, true, _) => Guide {
            title: "C# (.NET SDK 8 o superior)",
            why: "C# se compila y ejecuta con el SDK de .NET (`dotnet`). Sin SDK no se muestran los ejercicios de C#.",
            official: "https://dotnet.microsoft.com/download",
            official_label: "dotnet.microsoft.com/download",
            cmd: "winget install Microsoft.DotNet.SDK.8",
            docs: "https://learn.microsoft.com/dotnet/core/install/windows",
        },
        (CodeLang::CSharp, Lang::En, true, _) => Guide {
            title: "C# (.NET SDK 8+)",
            why: "C# builds and runs with the .NET SDK (`dotnet`). Without it, C# exercises stay hidden.",
            official: "https://dotnet.microsoft.com/download",
            official_label: "dotnet.microsoft.com/download",
            cmd: "winget install Microsoft.DotNet.SDK.8",
            docs: "https://learn.microsoft.com/dotnet/core/install/windows",
        },
        (CodeLang::CSharp, Lang::Es, _, true) => Guide {
            title: "C# (.NET SDK 8 o superior)",
            why: "C# se compila y ejecuta con el SDK de .NET (`dotnet`). Sin SDK no se muestran los ejercicios de C#.",
            official: "https://dotnet.microsoft.com/download",
            official_label: "dotnet.microsoft.com/download",
            cmd: "brew install dotnet",
            docs: "https://learn.microsoft.com/dotnet/core/install/macos",
        },
        (CodeLang::CSharp, Lang::En, _, true) => Guide {
            title: "C# (.NET SDK 8+)",
            why: "C# builds and runs with the .NET SDK (`dotnet`). Without it, C# exercises stay hidden.",
            official: "https://dotnet.microsoft.com/download",
            official_label: "dotnet.microsoft.com/download",
            cmd: "brew install dotnet",
            docs: "https://learn.microsoft.com/dotnet/core/install/macos",
        },
        (CodeLang::CSharp, Lang::Es, _, _) => Guide {
            title: "C# (.NET SDK 8 o superior)",
            why: "C# se compila y ejecuta con el SDK de .NET (`dotnet`). Sin SDK no se muestran los ejercicios de C#.",
            official: "https://dotnet.microsoft.com/download",
            official_label: "dotnet.microsoft.com/download",
            cmd: "sudo apt install dotnet-sdk-8.0",
            docs: "https://learn.microsoft.com/dotnet/core/install/linux",
        },
        (CodeLang::CSharp, Lang::En, _, _) => Guide {
            title: "C# (.NET SDK 8+)",
            why: "C# builds and runs with the .NET SDK (`dotnet`). Without it, C# exercises stay hidden.",
            official: "https://dotnet.microsoft.com/download",
            official_label: "dotnet.microsoft.com/download",
            cmd: "sudo apt install dotnet-sdk-8.0",
            docs: "https://learn.microsoft.com/dotnet/core/install/linux",
        },
        (CodeLang::Python, Lang::Es, true, _) => Guide {
            title: "Python 3",
            why: "Los ejercicios de Python se ejecutan con el intérprete `python`. Sin Python 3 no hay lista de tareas.",
            official: "https://www.python.org/downloads/",
            official_label: "python.org/downloads",
            cmd: "winget install Python.Python.3.12",
            docs: "https://docs.python.org/3/using/windows.html",
        },
        (CodeLang::Python, Lang::En, true, _) => Guide {
            title: "Python 3",
            why: "Python tasks run with the `python` interpreter. Without Python 3 there is no exercise list.",
            official: "https://www.python.org/downloads/",
            official_label: "python.org/downloads",
            cmd: "winget install Python.Python.3.12",
            docs: "https://docs.python.org/3/using/windows.html",
        },
        (CodeLang::Python, Lang::Es, _, true) => Guide {
            title: "Python 3",
            why: "Los ejercicios de Python se ejecutan con el intérprete `python3`. Sin Python 3 no hay lista de tareas.",
            official: "https://www.python.org/downloads/",
            official_label: "python.org/downloads",
            cmd: "brew install python",
            docs: "https://docs.python.org/3/using/mac.html",
        },
        (CodeLang::Python, Lang::En, _, true) => Guide {
            title: "Python 3",
            why: "Python tasks run with the `python3` interpreter. Without Python 3 there is no exercise list.",
            official: "https://www.python.org/downloads/",
            official_label: "python.org/downloads",
            cmd: "brew install python",
            docs: "https://docs.python.org/3/using/mac.html",
        },
        (CodeLang::Python, Lang::Es, _, _) => Guide {
            title: "Python 3",
            why: "Los ejercicios de Python se ejecutan con `python3`. Sin Python 3 no hay lista de tareas.",
            official: "https://www.python.org/downloads/",
            official_label: "python.org/downloads",
            cmd: "sudo apt install python3",
            docs: "https://docs.python.org/3/using/unix.html",
        },
        (CodeLang::Python, Lang::En, _, _) => Guide {
            title: "Python 3",
            why: "Python tasks run with `python3`. Without Python 3 there is no exercise list.",
            official: "https://www.python.org/downloads/",
            official_label: "python.org/downloads",
            cmd: "sudo apt install python3",
            docs: "https://docs.python.org/3/using/unix.html",
        },
    }
}

pub fn open_url(url: &str) {
    let _ = webbrowser::open(url);
}
