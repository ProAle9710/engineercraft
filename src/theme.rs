use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, Style,
    TextStyle, Visuals,
};
use std::cell::Cell;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Palette {
    pub dark: bool,
    pub bg: Color32,
    pub bg_ok: Color32,
    pub bg_err: Color32,
    pub bg_run: Color32,
    pub bg_debug: Color32,
    pub panel: Color32,
    pub panel_2: Color32,
    pub line: Color32,
    pub grid: Color32,
    pub grid_strong: Color32,
    pub text: Color32,
    pub dim: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub accent_dim: Color32,
    pub ok: Color32,
    pub err: Color32,
    pub err_bg: Color32,
    pub debug_bg: Color32,
    pub warn: Color32,
    pub selection: Color32,
    pub kw: Color32,
    pub ty: Color32,
    pub str_: Color32,
    pub num: Color32,
    pub cm: Color32,
    pub todo: Color32,
    pub console: Color32,
    pub stdout: Color32,
    pub fade: Color32,
}

impl Palette {
    pub const DARK: Self = Self {
        dark: true,
        bg: Color32::from_rgb(10, 10, 9),
        bg_ok: Color32::from_rgb(16, 52, 30),
        bg_err: Color32::from_rgb(62, 14, 16),
        bg_run: Color32::from_rgb(42, 36, 14),
        bg_debug: Color32::from_rgb(36, 32, 16),
        panel: Color32::from_rgb(16, 16, 15),
        panel_2: Color32::from_rgb(20, 20, 18),
        line: Color32::from_rgb(42, 41, 37),
        grid: Color32::from_rgba_premultiplied(10, 9, 8, 10),
        grid_strong: Color32::from_rgba_premultiplied(18, 17, 16, 18),
        text: Color32::from_rgb(242, 237, 228),
        dim: Color32::from_rgb(138, 132, 120),
        muted: Color32::from_rgb(92, 88, 78),
        accent: Color32::from_rgb(200, 184, 154),
        accent_dim: Color32::from_rgb(90, 78, 58),
        ok: Color32::from_rgb(143, 179, 143),
        err: Color32::from_rgb(224, 122, 95),
        err_bg: Color32::from_rgba_premultiplied(28, 15, 12, 28),
        debug_bg: Color32::from_rgb(92, 68, 18),
        warn: Color32::from_rgb(214, 176, 110),
        selection: Color32::from_rgb(58, 50, 38),
        kw: Color32::from_rgb(200, 184, 154),
        ty: Color32::from_rgb(168, 191, 168),
        str_: Color32::from_rgb(214, 186, 140),
        num: Color32::from_rgb(184, 160, 144),
        cm: Color32::from_rgb(107, 103, 94),
        todo: Color32::from_rgb(224, 122, 95),
        console: Color32::from_rgb(12, 12, 12),
        stdout: Color32::from_rgb(200, 220, 180),
        fade: Color32::from_rgb(8, 8, 7),
    };

    pub const LIGHT: Self = Self {
        dark: false,
        bg: Color32::from_rgb(242, 238, 230),
        bg_ok: Color32::from_rgb(214, 232, 218),
        bg_err: Color32::from_rgb(244, 220, 214),
        bg_run: Color32::from_rgb(236, 226, 200),
        bg_debug: Color32::from_rgb(232, 224, 206),
        panel: Color32::from_rgb(250, 247, 240),
        panel_2: Color32::from_rgb(234, 228, 216),
        line: Color32::from_rgb(204, 196, 180),
        grid: Color32::from_rgba_premultiplied(80, 70, 50, 18),
        grid_strong: Color32::from_rgba_premultiplied(80, 70, 50, 28),
        text: Color32::from_rgb(32, 28, 22),
        dim: Color32::from_rgb(92, 84, 72),
        muted: Color32::from_rgb(130, 122, 108),
        accent: Color32::from_rgb(138, 108, 58),
        accent_dim: Color32::from_rgb(214, 200, 168),
        ok: Color32::from_rgb(46, 108, 64),
        err: Color32::from_rgb(176, 64, 48),
        err_bg: Color32::from_rgba_premultiplied(220, 180, 170, 40),
        debug_bg: Color32::from_rgb(232, 210, 150),
        warn: Color32::from_rgb(156, 108, 28),
        selection: Color32::from_rgb(220, 208, 176),
        kw: Color32::from_rgb(122, 72, 28),
        ty: Color32::from_rgb(36, 102, 72),
        str_: Color32::from_rgb(148, 72, 32),
        num: Color32::from_rgb(92, 64, 120),
        cm: Color32::from_rgb(130, 124, 112),
        todo: Color32::from_rgb(176, 64, 48),
        console: Color32::from_rgb(248, 245, 238),
        stdout: Color32::from_rgb(36, 92, 52),
        fade: Color32::from_rgb(242, 238, 230),
    };

    pub fn get(dark: bool) -> Self {
        if dark {
            Self::DARK
        } else {
            Self::LIGHT
        }
    }

    pub fn opposite(self) -> Self {
        Self::get(!self.dark)
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            dark: b.dark,
            bg: lerp_c(a.bg, b.bg, t),
            bg_ok: lerp_c(a.bg_ok, b.bg_ok, t),
            bg_err: lerp_c(a.bg_err, b.bg_err, t),
            bg_run: lerp_c(a.bg_run, b.bg_run, t),
            bg_debug: lerp_c(a.bg_debug, b.bg_debug, t),
            panel: lerp_c(a.panel, b.panel, t),
            panel_2: lerp_c(a.panel_2, b.panel_2, t),
            line: lerp_c(a.line, b.line, t),
            grid: lerp_c(a.grid, b.grid, t),
            grid_strong: lerp_c(a.grid_strong, b.grid_strong, t),
            text: lerp_c(a.text, b.text, t),
            dim: lerp_c(a.dim, b.dim, t),
            muted: lerp_c(a.muted, b.muted, t),
            accent: lerp_c(a.accent, b.accent, t),
            accent_dim: lerp_c(a.accent_dim, b.accent_dim, t),
            ok: lerp_c(a.ok, b.ok, t),
            err: lerp_c(a.err, b.err, t),
            err_bg: lerp_c(a.err_bg, b.err_bg, t),
            debug_bg: lerp_c(a.debug_bg, b.debug_bg, t),
            warn: lerp_c(a.warn, b.warn, t),
            selection: lerp_c(a.selection, b.selection, t),
            kw: lerp_c(a.kw, b.kw, t),
            ty: lerp_c(a.ty, b.ty, t),
            str_: lerp_c(a.str_, b.str_, t),
            num: lerp_c(a.num, b.num, t),
            cm: lerp_c(a.cm, b.cm, t),
            todo: lerp_c(a.todo, b.todo, t),
            console: lerp_c(a.console, b.console, t),
            stdout: lerp_c(a.stdout, b.stdout, t),
            fade: lerp_c(a.fade, b.fade, t),
        }
    }
}

fn lerp_c(a: Color32, b: Color32, t: f32) -> Color32 {
    let n = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_unmultiplied(n(a.r(), b.r()), n(a.g(), b.g()), n(a.b(), b.b()), n(a.a(), b.a()))
}

thread_local! {
    static PAL: Cell<Palette> = const { Cell::new(Palette::DARK) };
}

pub fn set_palette(p: Palette) {
    PAL.set(p);
}

pub fn pal() -> Palette {
    PAL.get()
}

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
    let display_paths = [
        r"C:\Windows\Fonts\georgia.ttf",
        r"C:\Windows\Fonts\georgiai.ttf",
        r"C:\Windows\Fonts\constan.ttf",
        r"C:\Windows\Fonts\palab.ttf",
        r"C:\Windows\Fonts\SitkaVF.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSerif-Bold.ttf",
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

    if let Some(bytes) = read_first(&display_paths) {
        fonts.font_data.insert(
            "craft_display".into(),
            Arc::new(FontData::from_owned(bytes)),
        );
        fonts.families.insert(
            FontFamily::Name("craft_display".into()),
            vec!["craft_display".into(), "craft_ui".into(), "craft_mono".into()],
        );
    }

    ctx.set_fonts(fonts);
}

pub fn display_family() -> FontFamily {
    FontFamily::Name("craft_display".into())
}

pub fn apply(ctx: &egui::Context, p: Palette) {
    set_palette(p);
    let mut style = Style {
        visuals: visuals(p),
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

fn visuals(p: Palette) -> Visuals {
    let mut v = if p.dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    v.dark_mode = p.dark;
    v.override_text_color = Some(p.text);
    v.panel_fill = p.bg;
    v.window_fill = p.panel;
    v.extreme_bg_color = p.fade;
    v.faint_bg_color = p.panel_2;
    v.code_bg_color = p.panel_2;
    v.hyperlink_color = p.accent;
    v.warn_fg_color = p.warn;
    v.error_fg_color = p.err;
    v.selection.bg_fill = p.selection;
    v.selection.stroke = hair(p.accent);
    v.widgets.noninteractive.bg_fill = p.panel;
    v.widgets.noninteractive.fg_stroke = hair(p.dim);
    v.widgets.noninteractive.bg_stroke = hair(p.line);
    v.widgets.inactive.bg_fill = p.panel_2;
    v.widgets.inactive.fg_stroke = hair(p.text);
    v.widgets.inactive.bg_stroke = hair(p.line);
    v.widgets.hovered.bg_fill = p.accent_dim;
    v.widgets.hovered.bg_stroke = hair(p.accent);
    v.widgets.hovered.fg_stroke = hair(p.text);
    v.widgets.active.bg_fill = p.accent_dim;
    v.widgets.active.bg_stroke = hair(p.accent);
    v.widgets.open.bg_fill = p.panel_2;
    v.widgets.open.bg_stroke = hair(p.accent);
    v
}

pub fn hair(color: Color32) -> Stroke {
    Stroke::new(1.0_f32, color)
}

fn read_first(paths: &[&str]) -> Option<Vec<u8>> {
    paths.iter().find_map(|p| std::fs::read(p).ok())
}
