fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "EngineerCraft");
        res.set("FileDescription", "EngineerCraft — Entrenador de programación");
        res.set("CompanyName", "Alejandro Infante");
        res.set("LegalCopyright", "Copyright © 2026 Alejandro Infante");
        res.set("OriginalFilename", "engineercraft.exe");
        res.set("InternalName", "EngineerCraft");
        if let Ok(v) = std::env::var("CARGO_PKG_VERSION") {
            res.set("FileVersion", &v);
            res.set("ProductVersion", &v);
        }
        let _ = res.compile();
    }
    println!("cargo:rerun-if-changed=assets/icon.ico");
}
