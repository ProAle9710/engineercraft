fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "JavaCraft");
        res.set("FileDescription", "JavaCraft — Entrenador de Java");
        res.set("CompanyName", "Alejandro Infante");
        res.set("LegalCopyright", "Copyright © 2026 Alejandro Infante");
        res.set("OriginalFilename", "javacraft.exe");
        res.set("InternalName", "JavaCraft");
        let _ = res.compile();
    }
    println!("cargo:rerun-if-changed=assets/icon.ico");
}
