#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod catalog;
mod complete;
mod debug;
mod highlight;
mod i18n;
mod markdown;
mod practice;
mod progress;
mod runner;
mod theme;

fn load_icon() -> eframe::egui::IconData {
    let bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(bytes)
        .expect("icon png")
        .into_rgba8();
    let (width, height) = image.dimensions();
    eframe::egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1480.0, 920.0])
            .with_min_inner_size([1100.0, 720.0])
            .with_title("JavaCraft")
            .with_icon(load_icon()),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "JavaCraft",
        options,
        Box::new(|cc| Ok(Box::new(app::JavaCraft::new(cc)))),
    )
}
