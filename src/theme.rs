use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, Style,
    TextStyle, Visuals,
};
use std::sync::Arc;

pub const BG: Color32 = Color32::from_rgb(10, 10, 9);
pub const BG_OK: Color32 = Color32::from_rgb(16, 52, 30);
pub const BG_ERR: Color32 = Color32::from_rgb(62, 14, 16);
pub const PANEL: Color32 = Color32::from_rgb(16, 16, 15);
pub const PANEL_2: Color32 = Color32::from_rgb(20, 20, 18);
pub const LINE: Color32 = Color32::from_rgb(42, 41, 37);
pub const GRID: Color32 = Color32::from_rgba_premultiplied(10, 9, 8, 10);
pub const GRID_STRONG: Color32 = Color32::from_rgba_premultiplied(18, 17, 16, 18);
pub const TEXT: Color32 = Color32::from_rgb(242, 237, 228);
pub const DIM: Color32 = Color32::from_rgb(138, 132, 120);
pub const MUTED: Color32 = Color32::from_rgb(92, 88, 78);
pub const ACCENT: Color32 = Color32::from_rgb(200, 184, 154);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(90, 78, 58);
pub const OK: Color32 = Color32::from_rgb(143, 179, 143);
pub const ERR: Color32 = Color32::from_rgb(224, 122, 95);
pub const ERR_BG: Color32 = Color32::from_rgba_premultiplied(28, 15, 12, 28);
pub const DEBUG_BG: Color32 = Color32::from_rgb(92, 68, 18);
pub const WARN: Color32 = Color32::from_rgb(214, 176, 110);
pub const SELECTION: Color32 = Color32::from_rgb(58, 50, 38);

pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    let mono_paths = [
        r"C:\Windows\Fonts\CascadiaMono.ttf",
        r"C:\Windows\Fonts\cascadiamono.ttf",
        r"C:\Windows\Fonts\consola.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        "/usr/share/fonts/TTF/JetBrainsMono-Regular.ttf",
        "/usr/share/fonts/truetype/jetbrains-mono/JetBrainsMono-Regular.ttf",
    ];
    let ui_paths = [
        r"C:\Windows\Fonts\segoeui.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    ];

    if let Some(bytes) = read_first(&mono_paths) {
        fonts.font_data.insert(
            "craft_mono".into(),
            Arc::new(FontData::from_owned(bytes)),
        );
        if let Some(fam) = fonts.families.get_mut(&FontFamily::Monospace) {
            fam.insert(0, "craft_mono".into());
        }
        if let Some(fam) = fonts.families.get_mut(&FontFamily::Proportional) {
            fam.insert(0, "craft_mono".into());
        }
    }

    if let Some(bytes) = read_first(&ui_paths) {
        fonts.font_data.insert(
            "craft_ui".into(),
            Arc::new(FontData::from_owned(bytes)),
        );
        if let Some(fam) = fonts.families.get_mut(&FontFamily::Proportional) {
            fam.insert(0, "craft_ui".into());
        }
    }

    ctx.set_fonts(fonts);
}

pub fn apply(ctx: &egui::Context) {
    let mut style = Style {
        visuals: dark_visuals(),
        ..Default::default()
    };
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.interact_size = egui::vec2(40.0, 22.0);
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.active.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.open.corner_radius = CornerRadius::ZERO;
    style.visuals.window_corner_radius = CornerRadius::ZERO;
    style.visuals.menu_corner_radius = CornerRadius::ZERO;

    let mono = FontId::new(13.0, FontFamily::Monospace);
    let ui = FontId::new(13.5, FontFamily::Proportional);
    style.text_styles.insert(TextStyle::Small, FontId::new(11.0, FontFamily::Monospace));
    style.text_styles.insert(TextStyle::Body, ui.clone());
    style.text_styles.insert(TextStyle::Button, FontId::new(12.5, FontFamily::Monospace));
    style.text_styles.insert(TextStyle::Heading, FontId::new(18.0, FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Monospace, mono);

    ctx.set_style(style);
}

fn dark_visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.dark_mode = true;
    v.override_text_color = Some(TEXT);
    v.panel_fill = BG;
    v.window_fill = PANEL;
    v.extreme_bg_color = Color32::from_rgb(8, 8, 7);
    v.faint_bg_color = PANEL_2;
    v.code_bg_color = Color32::from_rgb(14, 14, 13);
    v.hyperlink_color = ACCENT;
    v.warn_fg_color = WARN;
    v.error_fg_color = ERR;
    v.selection.bg_fill = SELECTION;
    v.selection.stroke = hair(ACCENT);
    v.widgets.noninteractive.bg_fill = PANEL;
    v.widgets.noninteractive.fg_stroke = hair(DIM);
    v.widgets.noninteractive.bg_stroke = hair(LINE);
    v.widgets.inactive.bg_fill = PANEL_2;
    v.widgets.inactive.fg_stroke = hair(TEXT);
    v.widgets.inactive.bg_stroke = hair(LINE);
    v.widgets.hovered.bg_fill = Color32::from_rgb(28, 26, 22);
    v.widgets.hovered.bg_stroke = hair(ACCENT);
    v.widgets.hovered.fg_stroke = hair(TEXT);
    v.widgets.active.bg_fill = ACCENT_DIM;
    v.widgets.active.bg_stroke = hair(ACCENT);
    v.widgets.open.bg_fill = PANEL_2;
    v.widgets.open.bg_stroke = hair(ACCENT);
    v
}

pub fn hair(color: Color32) -> Stroke {
    Stroke::new(1.0_f32, color)
}

fn read_first(paths: &[&str]) -> Option<Vec<u8>> {
    paths.iter().find_map(|p| std::fs::read(p).ok())
}
