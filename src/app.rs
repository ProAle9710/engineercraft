use crate::catalog::{self, Exercise};
use crate::codelang::{self, CodeLang};
use crate::complete::{self, Completion};
use crate::debug;
use crate::highlight;
use crate::i18n::{self, tr, Lang};
use crate::markdown;
use crate::practice;
use crate::progress::Progress;
use crate::runner::{self, Diag, Outcome};
use crate::theme::{self, hair, pal};
use crate::update;
use eframe::egui::text::{CCursor, CCursorRange, LayoutJob};
use eframe::egui::text_edit::TextEditState;
use eframe::egui::{
    self, Color32, Context, FontFamily, FontId, Frame, Key, Label, LayerId, Margin, Modifiers,
    Order, Pos2, RichText, ScrollArea, Sense, Stroke, TextEdit, Ui, Vec2,
};
use std::collections::HashSet;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};


#[derive(Clone, Copy, PartialEq, Eq)]
enum ConsoleKind {
    Idle,
    Run,
    Ok,
    Err,
    Debug,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WatchKind {
    New,
    Updated,
    Same,
}

struct WatchVar {
    name: String,
    value: String,
    kind: WatchKind,
    flash_at: Instant,
    history: Vec<String>,
}

pub struct EngineerCraft {
    base: Vec<Exercise>,
    catalog: Vec<Exercise>,
    code_lang: CodeLang,
    load_error: Option<String>,
    selected: usize,
    code: String,
    filter: String,
    console: String,
    console_kind: ConsoleKind,
    output_lines: Vec<String>,
    error_cards: Vec<Diag>,
    error_line: Option<usize>,
    error_lines: HashSet<usize>,
    progress: Progress,
    jdk: Option<codelang::Toolchain>,
    running: bool,
    tx: Sender<Outcome>,
    rx: Receiver<Outcome>,
    lint_tx: Sender<(u64, Vec<Diag>)>,
    lint_rx: Receiver<(u64, Vec<Diag>)>,
    lint_gen: u64,
    linting: bool,
    dirty_at: Option<Instant>,
    saved_at: Option<Instant>,
    window_focused: bool,
    completions: Vec<Completion>,
    completion_open: bool,
    completion_sel: usize,
    completion_start: usize,
    cursor_bytes: usize,
    popup_pos: Option<Pos2>,
    editor_id: egui::Id,
    pending_cursor: Option<usize>,
    show_result: bool,
    debug_tx: Option<Sender<debug::Cmd>>,
    debug_rx: Option<Receiver<debug::Event>>,
    debug_line: Option<usize>,
    debug_exec_line: Option<usize>,
    debug_locals: Vec<(String, String)>,
    debug_watch: Vec<WatchVar>,
    debug_watch_open: bool,
    debug_hist_open: Vec<String>,
    debug_busy: bool,
    practice_hint: String,
    snap_line: Option<usize>,
    lang: Lang,
    lang_anim: Option<LangAnim>,
    update_tx: Sender<update::Event>,
    update_rx: Receiver<update::Event>,
    update_pending: Option<update::Pending>,
    update_busy: bool,
    update_msg: Option<String>,
    full_file: bool,
    theme_anim: Option<ThemeAnim>,
    show_news: bool,
    show_settings: bool,
}

struct ThemeAnim {
    from: theme::Palette,
    to: theme::Palette,
    t: f32,
    origin: Pos2,
}

struct LangAnim {
    to: Lang,
    t: f32,
    swapped: bool,
}

impl EngineerCraft {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::install_fonts(&cc.egui_ctx);

        let (tx, rx) = mpsc::channel();
        let (lint_tx, lint_rx) = mpsc::channel();
        let (update_tx, update_rx) = mpsc::channel();
        {
            let tx = update_tx.clone();
            let ctx = cc.egui_ctx.clone();
            thread::spawn(move || {
                let _ = tx.send(update::check_latest(crate::version::APP_VERSION));
                ctx.request_repaint();
            });
        }
        let loaded = catalog::load();
        let (base, load_error) = match loaded {
            Ok(c) => (c, None),
            Err(e) => (Vec::new(), Some(e)),
        };
        let progress = Progress::load();
        let code_lang = CodeLang::from_saved(&progress.code_lang);
        let catalog = crate::transpile::adapt_all(&base, code_lang);
        theme::apply(&cc.egui_ctx, theme::Palette::get(progress.dark));
        let lang = Lang::from_saved(&progress.lang);
        let selected = catalog
            .iter()
            .position(|e| !e.is_free())
            .unwrap_or(0);
        let mut code = catalog
            .get(selected)
            .map(|ex| {
                catalog::body_from_draft(ex, catalog::draft_of(&progress, code_lang, &ex.id))
            })
            .unwrap_or_default();
        if lang == Lang::Es {
            code = i18n::translate_comments(&code, Lang::Es);
        }
        let full_file = progress.full_file;
        if full_file {
            if let Some(ex) = catalog.get(selected) {
                if !ex.is_free() && !catalog::is_full_source(ex, &code) {
                    code = catalog::assemble(ex, &code);
                }
            }
        }
        let write_at = practice::first_write_cursor(&code);
        let show_news = progress.last_seen_version != crate::version::APP_VERSION;

        Self {
            base,
            catalog,
            code_lang,
            load_error,
            selected,
            code,
            filter: String::new(),
            console: tr(lang, "Completa los TODO y pulsa CORRER  ·  F10 paso a paso", "Fill the TODOs and press RUN  ·  F10 to step").into(),
            console_kind: ConsoleKind::Idle,
            output_lines: Vec::new(),
            error_cards: Vec::new(),
            error_line: None,
            error_lines: HashSet::new(),
            progress,
            jdk: codelang::detect(code_lang),
            running: false,
            tx,
            rx,
            lint_tx,
            lint_rx,
            lint_gen: 0,
            linting: false,
            dirty_at: Some(Instant::now()),
            saved_at: None,
            window_focused: true,
            completions: Vec::new(),
            completion_open: false,
            completion_sel: 0,
            completion_start: 0,
            cursor_bytes: 0,
            popup_pos: None,
            editor_id: egui::Id::new("java_editor"),
            pending_cursor: Some(write_at),
            show_result: full_file,
            debug_tx: None,
            debug_rx: None,
            debug_line: None,
            debug_exec_line: None,
            debug_locals: Vec::new(),
            debug_watch: Vec::new(),
            debug_watch_open: false,
            debug_hist_open: Vec::new(),
            debug_busy: false,
            practice_hint: if full_file {
                code_lang.file_hint(lang)
            } else {
                code_lang.todo_hint(lang).into()
            },
            snap_line: None,
            lang,
            lang_anim: None,
            update_tx,
            update_rx,
            update_pending: None,
            update_busy: false,
            update_msg: None,
            full_file,
            theme_anim: None,
            show_news,
            show_settings: false,
        }
    }

    fn poll_update(&mut self, ctx: &Context) {
        while let Ok(ev) = self.update_rx.try_recv() {
            match ev {
                update::Event::Available(p) => {
                    self.update_pending = Some(p);
                    self.update_busy = false;
                }
                update::Event::UpToDate => {
                    self.update_pending = None;
                    self.update_busy = false;
                }
                update::Event::Failed(e) => {
                    self.update_busy = false;
                    self.update_msg = Some(e);
                }
                update::Event::Installing => {
                    self.update_busy = false;
                    self.update_pending = None;
                    self.update_msg = Some(
                        tr(self.lang, "Instalador abierto (UAC)", "Installer opened (UAC)")
                            .into(),
                    );
                }
                update::Event::Done => {
                    self.update_busy = false;
                }
            }
            ctx.request_repaint();
        }
    }

    fn start_update(&mut self, ctx: &Context) {
        let Some(pending) = self.update_pending.clone() else {
            return;
        };
        self.update_busy = true;
        self.update_msg = None;
        let tx = self.update_tx.clone();
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = tx.send(update::apply(&pending));
            ctx.request_repaint();
        });
    }

    fn current(&self) -> Option<&Exercise> {
        self.catalog.get(self.selected)
    }

    fn is_free(&self) -> bool {
        self.current().is_some_and(|e| e.is_free())
    }

    fn advanced(&self) -> bool {
        self.is_free() || self.full_file
    }

    fn full_source(&self) -> String {
        if self.advanced() {
            return self.code.clone();
        }
        match self.current() {
            Some(ex) => catalog::assemble(ex, &self.code),
            None => self.code.clone(),
        }
    }

    fn map_line(&self, full_line: usize) -> usize {
        if self.advanced() {
            return full_line.max(1);
        }
        let off = self.current().map(|e| e.prefix_newlines).unwrap_or(0);
        let mapped = full_line.saturating_sub(off).max(1);
        let max = self.code.lines().count().max(1);
        mapped.min(max)
    }

    fn set_full_file(&mut self, on: bool) {
        if self.is_free() || on == self.full_file {
            return;
        }
        self.stop_debug();
        let prefix_len = self.current().map(|e| e.prefix.len()).unwrap_or(0);
        if on {
            if let Some(ex) = self.current() {
                self.code = catalog::assemble(ex, &self.code);
            }
            self.full_file = true;
            self.show_result = true;
            self.cursor_bytes = self.cursor_bytes.saturating_add(prefix_len);
            self.practice_hint = self.code_lang.file_hint(self.lang);
        } else {
            if let Some(ex) = self.current() {
                self.code = catalog::body_of(ex, &self.code);
            }
            self.full_file = false;
            self.cursor_bytes = self.cursor_bytes.saturating_sub(prefix_len).min(self.code.len());
            self.practice_hint = self.code_lang.todo_hint(self.lang).into();
        }
        self.progress.full_file = self.full_file;
        self.pending_cursor = Some(self.cursor_bytes.min(self.code.len()));
        self.snap_line = Some(practice::line_index(&self.code, self.cursor_bytes));
        self.persist_draft();
    }

    fn remap_diags(&self, errors: Vec<Diag>) -> Vec<Diag> {
        errors
            .into_iter()
            .map(|mut d| {
                d.line = self.map_line(d.line);
                d
            })
            .collect()
    }

    fn canvas_bg(&self) -> Color32 {
        let p = pal();
        match self.console_kind {
            ConsoleKind::Ok => p.bg_ok,
            ConsoleKind::Err => p.bg_err,
            ConsoleKind::Run => p.bg_run,
            ConsoleKind::Debug => p.bg_debug,
            ConsoleKind::Idle => p.bg,
        }
    }

    fn set_dark(&mut self, ctx: &Context, dark: bool) {
        if pal().dark == dark && self.theme_anim.is_none() {
            return;
        }
        let screen = ctx.screen_rect();
        let origin = ctx
            .pointer_latest_pos()
            .unwrap_or(Pos2::new(screen.right() - 28.0, screen.top() + 28.0));
        self.theme_anim = Some(ThemeAnim {
            from: pal(),
            to: theme::Palette::get(dark),
            t: 0.0,
            origin,
        });
        self.progress.dark = dark;
        self.progress.save();
    }

    fn tick_theme(&mut self, ctx: &Context) {
        let Some(anim) = &mut self.theme_anim else {
            return;
        };
        let dt = ctx.input(|i| i.stable_dt).clamp(0.0, 0.05);
        anim.t = (anim.t + dt / 0.42).min(1.0);
        let t = 1.0 - (1.0 - anim.t).powi(3);
        let p = theme::Palette::lerp(anim.from, anim.to, t);
        let done = anim.t >= 1.0;
        let to = anim.to;
        theme::apply(ctx, p);
        if done {
            theme::apply(ctx, to);
            self.theme_anim = None;
        }
        ctx.request_repaint();
    }

    fn theme_overlay(&self, ctx: &Context) {
        let Some(anim) = &self.theme_anim else {
            return;
        };
        let screen = ctx.screen_rect();
        let t = 1.0 - (1.0 - anim.t.clamp(0.0, 1.0)).powi(3);
        let diag = screen.width().hypot(screen.height());
        let r = t * diag * 1.15;
        let a = ((1.0 - t) * 70.0) as u8;
        let c = anim.to.bg;
        let fill = Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a);
        egui::Area::new(egui::Id::new("theme_wipe"))
            .order(egui::Order::Foreground)
            .fixed_pos(screen.min)
            .interactable(false)
            .show(ctx, |ui| {
                ui.painter().circle_filled(anim.origin, r, fill);
            });
    }

    fn suggested_zoom(ctx: &Context) -> f32 {
        let w = ctx.input(|i| i.screen_rect.width()) * ctx.zoom_factor();
        if w >= 3000.0 {
            1.6
        } else if w >= 2400.0 {
            1.4
        } else if w >= 1900.0 {
            1.22
        } else {
            1.0
        }
    }

    fn effective_zoom(&self, ctx: &Context) -> f32 {
        if self.progress.ui_zoom >= 0.5 {
            self.progress.ui_zoom.clamp(0.75, 2.0)
        } else {
            Self::suggested_zoom(ctx)
        }
    }

    fn apply_zoom(&self, ctx: &Context) {
        let z = self.effective_zoom(ctx);
        if (ctx.zoom_factor() - z).abs() > 0.02 {
            ctx.set_zoom_factor(z);
        }
    }

    fn settings_overlay(&mut self, ctx: &Context) {
        if !self.show_settings {
            return;
        }
        let lang = self.lang;
        let auto = self.progress.ui_zoom < 0.5;
        let suggested = Self::suggested_zoom(ctx);
        let mut close = false;
        let mut save = false;
        egui::Window::new(tr(lang, "AJUSTES", "SETTINGS"))
            .id(egui::Id::new("settings_win"))
            .anchor(egui::Align2::RIGHT_TOP, [-16.0, 72.0])
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .frame(
                Frame::NONE
                    .fill(pal().panel)
                    .stroke(hair(pal().accent))
                    .inner_margin(Margin::same(18)),
            )
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.set_width(320.0);
                ui.label(
                    RichText::new(tr(lang, "AJUSTES", "SETTINGS"))
                        .monospace()
                        .size(10.0)
                        .color(pal().accent),
                );
                ui.label(
                    RichText::new(tr(
                        lang,
                        "Escala y tipografía",
                        "Scale and type",
                    ))
                    .font(FontId::new(18.0, theme::display_family()))
                    .color(pal().text),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(tr(
                        lang,
                        "En pantallas 2K/4K sube la escala para que botones y paneles no queden diminutos.",
                        "On 2K/4K displays raise the scale so controls are not tiny.",
                    ))
                    .size(13.0)
                    .color(pal().dim),
                );
                ui.add_space(12.0);
                let mut auto_on = auto;
                if ui
                    .checkbox(
                        &mut auto_on,
                        tr(lang, "Escala automática (resolución)", "Auto scale (resolution)"),
                    )
                    .changed()
                {
                    self.progress.ui_zoom = if auto_on { 0.0 } else { suggested };
                    save = true;
                }
                ui.add_space(8.0);
                ui.label(
                    RichText::new(tr(lang, "Escala de la interfaz", "UI scale"))
                        .monospace()
                        .size(11.0)
                        .color(pal().muted),
                );
                let mut zoom = if auto { suggested } else { self.progress.ui_zoom };
                if ui
                    .add(egui::Slider::new(&mut zoom, 0.85..=1.85).text("×"))
                    .changed()
                {
                    self.progress.ui_zoom = zoom;
                    save = true;
                }
                ui.label(
                    RichText::new(format!(
                        "{} {suggested:.2}×",
                        tr(lang, "Sugerido para esta pantalla:", "Suggested for this screen:")
                    ))
                    .size(12.0)
                    .color(pal().dim),
                );
                ui.add_space(12.0);
                ui.label(
                    RichText::new(tr(lang, "Tamaño de fuente del editor", "Editor font size"))
                        .monospace()
                        .size(11.0)
                        .color(pal().muted),
                );
                let mut fs = self.progress.font_size;
                if ui
                    .add(egui::Slider::new(&mut fs, 11.0..=24.0).suffix(" pt"))
                    .changed()
                {
                    self.progress.font_size = fs;
                    save = true;
                }
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(tr(lang, "RESTAURAR TAMAÑOS", "RESET SIZES"))
                                    .monospace()
                                    .size(11.0)
                                    .color(pal().text),
                            )
                            .stroke(hair(pal().line)),
                        )
                        .clicked()
                    {
                        self.progress.ui_zoom = 0.0;
                        self.progress.font_size = 13.5;
                        save = true;
                    }
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(tr(lang, "CERRAR", "CLOSE"))
                                    .monospace()
                                    .size(11.0)
                                    .color(pal().bg),
                            )
                            .fill(pal().accent),
                        )
                        .clicked()
                    {
                        close = true;
                    }
                });
            });
        if save {
            self.progress.save();
        }
        if close {
            self.show_settings = false;
        }
    }

    fn dismiss_news(&mut self) {
        self.show_news = false;
        self.progress.last_seen_version = crate::version::APP_VERSION.into();
        self.progress.save();
    }

    fn news_overlay(&mut self, ctx: &Context) {
        if !self.show_news {
            return;
        }
        let lang = self.lang;
        let rel = crate::news::for_version(crate::version::APP_VERSION);
        let items = crate::news::items(rel, lang);
        let mut close = false;
        let screen = ctx.screen_rect();
        egui::Area::new(egui::Id::new("news_scrim"))
            .order(egui::Order::Foreground)
            .fixed_pos(screen.min)
            .interactable(true)
            .show(ctx, |ui| {
                let resp = ui.allocate_rect(screen, Sense::click());
                ui.painter().rect_filled(
                    screen,
                    0.0,
                    Color32::from_rgba_unmultiplied(8, 8, 7, 150),
                );
                if resp.clicked() {
                    close = true;
                }
            });
        egui::Window::new(tr(lang, "NOVEDADES", "WHAT'S NEW"))
            .id(egui::Id::new("news_win"))
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .frame(
                Frame::NONE
                    .fill(pal().panel)
                    .stroke(hair(pal().accent))
                    .inner_margin(Margin::same(20)),
            )
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.set_max_width(400.0);
                ui.label(
                    RichText::new(tr(lang, "NOVEDADES", "WHAT'S NEW"))
                        .monospace()
                        .size(10.0)
                        .color(pal().accent),
                );
                ui.label(
                    RichText::new(format!("{} v{}", crate::version::APP_NAME, rel.version))
                        .font(FontId::new(20.0, FontFamily::Proportional))
                        .color(pal().text),
                );
                ui.add_space(12.0);
                for line in items {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("▸").color(pal().accent).monospace());
                        ui.label(RichText::new(*line).size(13.5).color(pal().text));
                    });
                    ui.add_space(6.0);
                }
                ui.add_space(10.0);
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(tr(lang, "ENTENDIDO", "GOT IT"))
                                .monospace()
                                .size(12.0)
                                .color(pal().bg),
                        )
                        .fill(pal().accent)
                        .stroke(Stroke::NONE),
                    )
                    .clicked()
                {
                    close = true;
                }
            });
        if close {
            self.dismiss_news();
        }
    }

    fn tool_ready(&self) -> bool {
        self.jdk.is_some()
    }

    fn reload_editor_code(&mut self) {
        let Some(ex) = self.current().cloned() else {
            return;
        };
        self.code = catalog::body_from_draft(
            &ex,
            catalog::draft_of(&self.progress, self.code_lang, &ex.id),
        );
        if self.lang == Lang::Es {
            self.code = i18n::translate_comments(&self.code, Lang::Es);
        }
        if self.full_file && !ex.is_free() && !catalog::is_full_source(&ex, &self.code) {
            self.code = catalog::assemble(&ex, &self.code);
        }
        self.pending_cursor = Some(practice::first_write_cursor(&self.code));
    }

    fn set_code_lang(&mut self, lang: CodeLang) {
        if lang == self.code_lang {
            self.jdk = codelang::detect(lang);
            return;
        }
        if self.tool_ready() {
            self.persist_draft();
        }
        let keep = self.current().map(|e| e.id.clone());
        self.code_lang = lang;
        self.progress.code_lang = lang.id().into();
        self.catalog = crate::transpile::adapt_all(&self.base, lang);
        self.jdk = codelang::detect(lang);
        if let Some(id) = keep {
            if let Some(i) = self.catalog.iter().position(|e| e.id == id) {
                self.selected = i;
            }
        }
        self.stop_debug();
        if self.tool_ready() {
            self.reload_editor_code();
            self.mark_dirty();
        }
        self.progress.save();
    }

    fn persist_draft(&mut self) {
        self.progress.full_file = self.full_file;
        self.progress.dark = pal().dark;
        self.progress.code_lang = self.code_lang.id().into();
        if let Some(ex) = self.current() {
            let id = ex.id.clone();
            let keep_empty = ex.is_free();
            let text = if self.full_file && !keep_empty {
                catalog::body_of(ex, &self.code)
            } else {
                self.code.clone()
            };
            let key = self.code_lang.draft_key(&id);
            if text.trim().is_empty() && !keep_empty {
                self.progress.drafts.remove(&key);
            } else {
                self.progress.drafts.insert(key, text);
            }
        }
        self.progress.save();
    }

    fn request_lang(&mut self, to: Lang) {
        if self.lang == to || self.lang_anim.is_some() {
            return;
        }
        self.lang_anim = Some(LangAnim {
            to,
            t: 0.0,
            swapped: false,
        });
    }

    fn apply_lang(&mut self, to: Lang) {
        self.lang = to;
        self.code = i18n::translate_comments(&self.code, to);
        self.progress.lang = to.as_saved().into();
        self.progress.save();
        self.practice_hint = practice::hint_at(&self.code, self.cursor_bytes, to, self.code_lang);
        if matches!(self.console_kind, ConsoleKind::Idle) {
            self.console = tr(
                to,
                "Completa los TODO y pulsa CORRER  ·  F10 paso a paso",
                "Fill the TODOs and press RUN  ·  F10 to step",
            )
            .into();
        }
    }

    fn tick_lang(&mut self, ctx: &Context) {
        let dt = ctx.input(|i| i.stable_dt).clamp(0.0, 0.05);
        let mut apply = None;
        if let Some(anim) = &mut self.lang_anim {
            anim.t += dt / 0.62;
            if !anim.swapped && anim.t >= 0.42 {
                anim.swapped = true;
                apply = Some(anim.to);
            }
        }
        if let Some(to) = apply {
            self.apply_lang(to);
        }
        if self.lang_anim.as_ref().is_some_and(|a| a.t >= 1.0) {
            self.lang_anim = None;
        } else if self.lang_anim.is_some() {
            ctx.request_repaint();
        }
    }

    fn lang_overlay(&self, ctx: &Context) {
        let Some(anim) = &self.lang_anim else {
            return;
        };
        let t = anim.t.clamp(0.0, 1.0);
        let wave = if t < 0.5 { t / 0.5 } else { (1.0 - t) / 0.5 };
        let wave = wave.clamp(0.0, 1.0);
        let a = (wave * 200.0) as u8;
        let screen = ctx.screen_rect();
        egui::Area::new(egui::Id::new("lang_fade"))
            .order(egui::Order::Foreground)
            .fixed_pos(screen.min)
            .interactable(false)
            .show(ctx, |ui| {
                ui.painter().rect_filled(
                    screen,
                    0.0,
                    {
                        let f = pal().fade;
                        Color32::from_rgba_unmultiplied(f.r(), f.g(), f.b(), a)
                    },
                );
            });
        if wave > 0.2 {
            let pos = screen.center() - Vec2::new(120.0, 16.0);
            egui::Area::new(egui::Id::new("lang_fade_msg"))
                .order(egui::Order::Foreground)
                .fixed_pos(pos)
                .interactable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(
                            RichText::new(anim.to.switching_to())
                                .size(18.0)
                                .color(pal().text),
                        );
                    });
                });
        }
    }

    fn select(&mut self, index: usize) {
        if index == self.selected || index >= self.catalog.len() {
            return;
        }
        self.persist_draft();
        self.selected = index;
        let ex = &self.catalog[index];
        self.code = catalog::body_from_draft(
            ex,
            catalog::draft_of(&self.progress, self.code_lang, &ex.id),
        );
        if self.lang == Lang::Es {
            self.code = i18n::translate_comments(&self.code, Lang::Es);
        }
        if self.full_file && !ex.is_free() && !catalog::is_full_source(ex, &self.code) {
            self.code = catalog::assemble(ex, &self.code);
        }
        self.error_line = None;
        self.error_lines.clear();
        self.error_cards.clear();
        self.output_lines.clear();
        self.completion_open = false;
        self.console = format!(
            "{} {} — {}",
            tr(self.lang, "Tarea", "Task"),
            ex.id,
            ex.title(self.lang)
        );
        self.console_kind = ConsoleKind::Idle;
        self.show_result = false;
        self.stop_debug();
        self.pending_cursor = Some(practice::first_write_cursor(&self.code));
        if self.advanced() {
            self.show_result = true;
            self.practice_hint = self.code_lang.file_hint(self.lang);
        } else {
            self.practice_hint = self.code_lang.todo_hint(self.lang).into();
        }
        self.mark_dirty();
    }

    fn reset_starter(&mut self) {
        if let Some(ex) = self.current() {
            let id = ex.id.clone();
            self.code = if self.advanced() && !ex.is_free() {
                catalog::assemble(ex, &ex.body)
            } else {
                ex.body.clone()
            };
            self.progress.drafts.remove(&self.code_lang.draft_key(&id));
            self.progress.save();
            self.error_line = None;
            self.error_lines.clear();
            self.error_cards.clear();
            self.output_lines.clear();
            self.console = tr(self.lang, "Plantilla original restaurada.", "Original template restored.").into();
            if self.lang == Lang::Es {
                self.code = i18n::translate_comments(&self.code, Lang::Es);
            }
            self.console_kind = ConsoleKind::Idle;
            self.pending_cursor = Some(practice::first_write_cursor(&self.code));
            self.mark_dirty();
        }
    }

    fn run(&mut self) {
        if self.running || !self.tool_ready() {
            return;
        }
        let needles = match self.current() {
            Some(ex) => ex.stdout_needles.clone(),
            None => return,
        };
        self.persist_draft();
        self.stop_debug();
        self.show_result = true;
        self.running = true;
        self.console_kind = ConsoleKind::Run;
        self.console = tr(self.lang, "Compilando…", "Compiling…").into();
        self.error_line = None;
        self.error_cards.clear();
        self.output_lines.clear();
        let code = self.full_source();
        let tx = self.tx.clone();
        let lang = self.code_lang;
        thread::spawn(move || {
            let _ = tx.send(runner::evaluate(&code, &needles, lang));
        });
    }

    fn start_debug(&mut self) {
        if self.running {
            return;
        }
        self.persist_draft();
        self.stop_debug();
        self.show_result = true;
        self.console_kind = ConsoleKind::Debug;
        self.console = tr(self.lang, "Cargando depurador…", "Loading debugger…").into();
        self.debug_busy = true;
        self.error_cards.clear();
        self.output_lines.clear();
        self.debug_line = None;
        self.debug_exec_line = None;
        self.debug_locals.clear();
        self.debug_watch.clear();
        self.debug_watch_open = true;
        self.debug_hist_open.clear();
        let source = self.full_source();
        let prefix = if self.advanced() {
            0
        } else {
            self.current().map(|e| e.prefix_newlines).unwrap_or(0)
        };
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (ev_tx, ev_rx) = mpsc::channel();
        self.debug_tx = Some(cmd_tx);
        self.debug_rx = Some(ev_rx);
        debug::start(source, prefix, self.code_lang, cmd_rx, ev_tx);
    }

    fn debug_step(&mut self) {
        if self.debug_busy {
            return;
        }
        if let Some(tx) = &self.debug_tx {
            self.debug_busy = true;
            self.console = tr(self.lang, "Cargando siguiente línea…", "Loading next line…").into();
            let _ = tx.send(debug::Cmd::Step);
        }
    }

    fn stop_debug(&mut self) {
        if let Some(tx) = self.debug_tx.take() {
            let _ = tx.send(debug::Cmd::Stop);
        }
        self.debug_rx = None;
        self.debug_line = None;
        self.debug_exec_line = None;
        self.debug_locals.clear();
        self.debug_watch.clear();
        self.debug_watch_open = false;
        self.debug_hist_open.clear();
        self.debug_busy = false;
        if matches!(self.console_kind, ConsoleKind::Debug) {
            self.console_kind = ConsoleKind::Idle;
        }
    }

    fn poll_debug(&mut self, ctx: &Context) {
        if self.debug_rx.is_none() {
            return;
        }
        ctx.request_repaint();
        let mut events = Vec::new();
        if let Some(rx) = &self.debug_rx {
            while let Ok(ev) = rx.try_recv() {
                events.push(ev);
            }
        }
        for ev in events {
            match ev {
                debug::Event::Paused {
                    line,
                    exec_line,
                    locals,
                } => {
                    self.debug_line = Some(line);
                    self.debug_exec_line = exec_line;
                    self.merge_watch(locals);
                    self.console_kind = ConsoleKind::Debug;
                    self.console = format!(
                        "{} {line}  ·  {}",
                        tr(self.lang, "Línea", "Line"),
                        tr(self.lang, "F10 siguiente", "F10 next")
                    );
                    self.show_result = true;
                    self.debug_watch_open = true;
                    self.debug_busy = false;
                    self.pending_cursor = Some(byte_at_line(&self.code, line));
                }
                debug::Event::Ended { output } => {
                    self.debug_tx = None;
                    self.debug_rx = None;
                    self.debug_line = None;
                    self.debug_exec_line = None;
                    self.debug_busy = false;
                    self.console_kind = ConsoleKind::Ok;
                    self.console = tr(self.lang, "El programa terminó", "The program finished").into();
                    self.output_lines = stdout_lines(&output);
                    self.show_result = true;
                }
                debug::Event::Failed { message } => {
                    self.debug_tx = None;
                    self.debug_rx = None;
                    self.debug_line = None;
                    self.debug_exec_line = None;
                    self.debug_busy = false;
                    self.console_kind = ConsoleKind::Err;
                    self.console = tr(self.lang, "No se pudo depurar", "Could not debug").into();
                    self.error_cards = vec![Diag {
                        line: 1,
                        message,
                        extra: tr(
                            self.lang,
                            "Completa los TODO hasta que compile, luego PASO A PASO F10.",
                            "Finish the TODOs until it compiles, then STEP F10.",
                        )
                        .into(),
                    }];
                    self.show_result = true;
                }
            }
        }
    }

    fn merge_watch(&mut self, locals: Vec<(String, String)>) {
        if locals.is_empty() {
            return;
        }
        let now = Instant::now();
        let mut next = Vec::new();
        for (name, value) in locals {
            if let Some(old) = self.debug_watch.iter().find(|w| w.name == name) {
                if old.value == value {
                    next.push(WatchVar {
                        name,
                        value,
                        kind: WatchKind::Same,
                        flash_at: old.flash_at,
                        history: old.history.clone(),
                    });
                } else {
                    let mut history = old.history.clone();
                    if history.last() != Some(&value) {
                        history.push(value.clone());
                    }
                    next.push(WatchVar {
                        name,
                        value,
                        kind: WatchKind::Updated,
                        flash_at: now,
                        history,
                    });
                }
            } else {
                next.push(WatchVar {
                    history: vec![value.clone()],
                    name,
                    value,
                    kind: WatchKind::New,
                    flash_at: now,
                });
            }
        }
        self.debug_locals = next
            .iter()
            .map(|w| (w.name.clone(), w.value.clone()))
            .collect();
        self.debug_watch = next;
    }

    fn watch_window(&mut self, ctx: &Context) {
        if !self.debug_watch_open && self.debug_tx.is_none() && self.debug_watch.is_empty() {
            self.history_windows(ctx);
            return;
        }
        if self.debug_watch.iter().any(|w| {
            w.kind != WatchKind::Same && w.flash_at.elapsed() < Duration::from_millis(1600)
        }) {
            ctx.request_repaint();
        }
        let screen = ctx.screen_rect();
        let mut open = self.debug_watch_open;
        let lang = self.lang;
        egui::Window::new(tr(lang, "Variables", "Variables"))
            .id(egui::Id::new("debug_watch"))
            .open(&mut open)
            .resizable(true)
            .collapsible(false)
            .default_size(Vec2::new(300.0, 260.0))
            .min_size(Vec2::new(200.0, 120.0))
            .default_pos(Pos2::new(screen.right() - 340.0, 72.0))
            .frame(
                Frame::NONE
                    .fill(pal().panel)
                    .stroke(hair(pal().warn))
                    .inner_margin(Margin::symmetric(12, 10))
                    .corner_radius(4.0),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(tr(lang, "PASO A PASO", "STEP DEBUG"))
                            .monospace()
                            .size(10.0)
                            .color(pal().warn),
                    );
                    if self.debug_busy {
                        ui.spinner();
                        ui.label(
                            RichText::new(tr(lang, "leyendo…", "reading…"))
                                .monospace()
                                .size(11.0)
                                .color(pal().warn),
                        );
                    }
                });
                ui.add_space(6.0);
                if self.debug_busy {
                    ui.label(
                        RichText::new(tr(
                            lang,
                            "Espera. No pulses F10 otra vez.",
                            "Wait. Don’t press F10 again.",
                        ))
                            .size(13.0)
                            .color(pal().muted),
                    );
                    ui.add_space(8.0);
                }
                if self.debug_watch.is_empty() && !self.debug_busy {
                    ui.label(
                        RichText::new(tr(
                            lang,
                            "Aún no hay variables. Pulsa F10.",
                            "No variables yet. Press F10.",
                        ))
                            .size(13.0)
                            .color(pal().muted),
                    );
                    return;
                }
                ScrollArea::vertical()
                    .id_salt("watch_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        let mut open_hist = None::<String>;
                        for w in &self.debug_watch {
                            let hot = w.kind != WatchKind::Same
                                && w.flash_at.elapsed() < Duration::from_millis(1600);
                            let fill = if !hot {
                                pal().panel_2
                            } else if w.kind == WatchKind::New {
                                pal().bg_ok
                            } else {
                                pal().bg_debug
                            };
                            Frame::NONE
                                .fill(fill)
                                .inner_margin(Margin::symmetric(8, 6))
                                .corner_radius(3.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.add(
                                            Label::new(
                                                RichText::new(format!("{}:{}", w.name, w.value))
                                                    .monospace()
                                                    .size(14.0)
                                                    .color(if hot { pal().warn } else { pal().text }),
                                            )
                                            .wrap(),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if ui
                                                    .small_button("⋯")
                                                    .on_hover_text(tr(lang, "Historial", "History"))
                                                    .clicked()
                                                {
                                                    open_hist = Some(w.name.clone());
                                                }
                                            },
                                        );
                                    });
                                });
                            ui.add_space(4.0);
                        }
                        if let Some(name) = open_hist {
                            if !self.debug_hist_open.iter().any(|n| n == &name) {
                                self.debug_hist_open.push(name);
                            }
                        }
                    });
            });
        self.debug_watch_open = open;
        self.history_windows(ctx);
    }

    fn history_windows(&mut self, ctx: &Context) {
        let names = self.debug_hist_open.clone();
        let mut still = Vec::new();
        for name in names {
            let history = self
                .debug_watch
                .iter()
                .find(|w| w.name == name)
                .map(|w| w.history.clone())
                .unwrap_or_default();
            let mut open = true;
            let screen = ctx.screen_rect();
            let lang = self.lang;
            egui::Window::new(format!(
                "{} {name}",
                tr(lang, "Historial de", "History of")
            ))
                .id(egui::Id::new(("debug_hist", name.clone())))
                .open(&mut open)
                .resizable(true)
                .collapsible(false)
                .default_size(Vec2::new(280.0, 200.0))
                .default_pos(Pos2::new(screen.right() - 640.0, 120.0))
                .frame(
                    Frame::NONE
                        .fill(pal().panel)
                        .stroke(hair(pal().warn))
                        .inner_margin(Margin::symmetric(12, 10))
                        .corner_radius(4.0),
                )
                .show(ctx, |ui| {
                    ui.label(
                        RichText::new(format!("{} · {name}", tr(lang, "VALORES", "VALUES")))
                            .monospace()
                            .size(10.0)
                            .color(pal().warn),
                    );
                    ui.add_space(6.0);
                    if history.is_empty() {
                        ui.label(
                            RichText::new(tr(lang, "Sin historial todavía.", "No history yet."))
                                .size(13.0)
                                .color(pal().muted),
                        );
                    } else {
                        ScrollArea::vertical()
                            .id_salt(("hist_scroll", &name))
                            .show(ui, |ui| {
                                for (i, val) in history.iter().enumerate() {
                                    let last = i + 1 == history.len();
                                    ui.label(
                                        RichText::new(format!("{}.  {val}", i + 1))
                                            .monospace()
                                            .size(13.5)
                                            .color(if last { pal().warn } else { pal().text }),
                                    );
                                }
                            });
                    }
                });
            if open {
                still.push(name);
            }
        }
        self.debug_hist_open = still;
    }

    fn set_error_lines(&mut self, errors: &[Diag]) {
        self.error_lines = errors.iter().map(|e| e.line).collect();
        self.error_line = errors.first().map(|e| e.line);
    }

    fn mark_dirty(&mut self) {
        self.dirty_at = Some(Instant::now());
    }

    fn poll_autosave(&mut self, ctx: &Context) {
        let focused = ctx.input(|i| i.focused);
        let lost_focus = self.window_focused && !focused;
        self.window_focused = focused;
        let due = self.dirty_at.is_some_and(|t| t.elapsed() >= Duration::from_millis(600));
        let unsaved = match (self.dirty_at, self.saved_at) {
            (Some(d), Some(s)) => d > s,
            (Some(_), None) => true,
            _ => false,
        };
        if lost_focus || (due && unsaved) {
            self.persist_draft();
            self.saved_at = Some(Instant::now());
        }
        if unsaved {
            ctx.request_repaint_after(Duration::from_millis(200));
        }
    }

    fn poll_lint(&mut self, ctx: &Context) {
        while let Ok((gen, errors)) = self.lint_rx.try_recv() {
            if gen != self.lint_gen {
                continue;
            }
            self.linting = false;
            if self.running {
                continue;
            }
            let errors = self.remap_diags(errors);
            self.set_error_lines(&errors);
            if matches!(self.console_kind, ConsoleKind::Debug) {
                continue;
            }
            if errors.is_empty() {
                if !self.running {
                    self.error_cards.clear();
                }
            } else {
                self.error_cards = errors;
            }
        }

        let due = self
            .dirty_at
            .map(|t| t.elapsed() >= Duration::from_millis(450))
            .unwrap_or(false);
        if due && !self.linting && !self.running && self.tool_ready() {
            self.dirty_at = None;
            self.linting = true;
            self.lint_gen += 1;
            let gen = self.lint_gen;
            let code = self.full_source();
            let tx = self.lint_tx.clone();
            let lang = self.code_lang;
            thread::spawn(move || {
                let _ = tx.send((gen, runner::lint(&code, lang)));
            });
        }
        if self.linting || self.dirty_at.is_some() {
            ctx.request_repaint_after(Duration::from_millis(80));
        }
    }

    fn poll_run(&mut self, ctx: &Context) {
        if !self.running {
            return;
        }
        ctx.request_repaint();
        if let Ok(outcome) = self.rx.try_recv() {
            self.running = false;
            self.apply_outcome(outcome);
        }
    }

    fn apply_outcome(&mut self, outcome: Outcome) {
        let id = self.current().map(|e| e.id.clone());
        match outcome {
            Outcome::NoJdk => {
                self.console_kind = ConsoleKind::Err;
                let (msg, extra) = match self.code_lang {
                    CodeLang::Python => (
                        tr(self.lang, "No encuentro Python", "Cannot find Python"),
                        "Instala Python 3 y pulsa AJUSTES o YA LO INSTALÉ.",
                    ),
                    CodeLang::CSharp => (
                        tr(self.lang, "No encuentro el SDK de .NET", "Cannot find the .NET SDK"),
                        "Instala .NET SDK y pulsa YA LO INSTALÉ.",
                    ),
                    CodeLang::Java => (
                        tr(self.lang, "No encuentro javac/java", "Cannot find javac/java"),
                        "Instala un JDK y reinicia EngineerCraft.",
                    ),
                };
                self.console = msg.into();
                self.error_cards = vec![Diag {
                    line: 1,
                    message: extra.into(),
                    extra: String::new(),
                }];
                self.output_lines.clear();
            }
            Outcome::CompileError { errors } => {
                self.console_kind = ConsoleKind::Err;
                let errors = self.remap_diags(errors);
                self.set_error_lines(&errors);
                self.console = tr(self.lang, "No compiló", "Did not compile").into();
                self.error_cards = errors;
                self.output_lines.clear();
            }
            Outcome::RuntimeError { line, message, stdout } => {
                self.console_kind = ConsoleKind::Err;
                let line = line.map(|n| self.map_line(n));
                self.error_lines = line.into_iter().collect();
                self.error_line = line;
                self.console = tr(self.lang, "Falló al ejecutar", "Runtime error").into();
                self.error_cards = vec![Diag {
                    line: line.unwrap_or(1),
                    message,
                    extra: String::new(),
                }];
                self.output_lines = stdout_lines(&stdout);
            }
            Outcome::Timeout => {
                self.console_kind = ConsoleKind::Err;
                self.console = tr(self.lang, "Tiempo agotado", "Timed out").into();
                self.error_cards = vec![Diag {
                    line: 1,
                    message: "El programa no terminó en 8s.".into(),
                    extra: "¿Olvidaste i++ en un while?".into(),
                }];
            }
            Outcome::FailedChecks { stdout, missing } => {
                self.console_kind = ConsoleKind::Err;
                self.console = tr(self.lang, "Compiló, pero la salida no es la esperada", "Compiled, but output is wrong").into();
                self.error_cards = missing
                    .into_iter()
                    .map(|m| Diag {
                        line: 1,
                        message: format!("Falta en la salida: {m}"),
                        extra: String::new(),
                    })
                    .collect();
                self.output_lines = stdout_lines(&stdout);
            }
            Outcome::Success { stdout } => {
                self.error_lines.clear();
                self.error_line = None;
                self.error_cards.clear();
                self.output_lines = stdout_lines(&stdout);
                if self.is_free() {
                    self.console_kind = ConsoleKind::Idle;
                    self.console = if stdout.trim().is_empty() {
                        tr(self.lang, "(sin salida)", "(no output)").into()
                    } else {
                        String::new()
                    };
                } else if self.full_file {
                    self.console_kind = ConsoleKind::Ok;
                    if let Some(id) = &id {
                        self.progress.mark_complete(&self.code_lang.draft_key(id));
                        self.progress.save();
                    }
                    self.console = if stdout.trim().is_empty() {
                        tr(self.lang, "Correcto  ·  (sin salida)", "OK  ·  (no output)").into()
                    } else {
                        tr(self.lang, "Correcto", "OK").into()
                    };
                } else {
                    self.console_kind = ConsoleKind::Ok;
                    if let Some(id) = &id {
                        self.progress.mark_complete(&self.code_lang.draft_key(id));
                        self.progress.save();
                    }
                    let scored = self.catalog.iter().filter(|e| !e.is_free()).count();
                    self.console = format!(
                        "Correcto  ·  {}/{}",
                        self.progress.complete_count(self.code_lang),
                        scored
                    );
                }
            }
        }
    }

    fn handle_keys(&mut self, ctx: &Context) {
        let mut run = false;
        let mut complete = false;
        let mut accept = false;
        let mut reset_sel = None::<i32>;

        let mut step = false;
        ctx.input_mut(|i| {
            if i.key_pressed(Key::F5) || (i.modifiers.command && i.key_pressed(Key::Enter)) {
                run = true;
            }
            if i.key_pressed(Key::F10) {
                step = true;
            }
            if (i.modifiers.ctrl || i.modifiers.command) && i.key_pressed(Key::Space) {
                i.consume_key(Modifiers::CTRL, Key::Space);
                i.consume_key(Modifiers::COMMAND, Key::Space);
                complete = true;
            }
        });

        if self.completion_open {
            ctx.input_mut(|i| {
                if i.consume_key(Modifiers::NONE, Key::Tab)
                    || i.consume_key(Modifiers::NONE, Key::Enter)
                {
                    accept = true;
                }
                if i.consume_key(Modifiers::NONE, Key::Escape) {
                    self.completion_open = false;
                }
                if i.consume_key(Modifiers::NONE, Key::ArrowDown) {
                    reset_sel = Some(1);
                }
                if i.consume_key(Modifiers::NONE, Key::ArrowUp) {
                    reset_sel = Some(-1);
                }
            });
        }

        if let Some(delta) = reset_sel {
            if !self.completions.is_empty() {
                let len = self.completions.len() as i32;
                self.completion_sel =
                    ((self.completion_sel as i32 + delta).rem_euclid(len)) as usize;
            }
        }
        if run {
            self.run();
        }
        if step && !self.debug_busy {
            if self.debug_tx.is_some() {
                self.debug_step();
            } else {
                self.start_debug();
            }
        }
        if complete {
            self.open_completions();
        }
        if accept {
            self.accept_completion();
        }
    }

    fn open_completions(&mut self) {
        let prefix = complete::current_prefix(&self.code, self.cursor_bytes, self.code_lang);
        self.completion_start = prefix.start;
        self.completions = complete::matching_forced(&prefix, &self.code, self.lang, self.code_lang);
        self.completion_open = !self.completions.is_empty();
        self.completion_sel = 0;
    }

    fn refresh_completions(&mut self) {
        let prefix = complete::current_prefix(&self.code, self.cursor_bytes, self.code_lang);
        self.completion_start = prefix.start;
        if prefix.text.is_empty() && !prefix.after_dot {
            self.completion_open = false;
            return;
        }
        self.completions = complete::matching(&prefix, &self.code, self.lang, self.code_lang);
        self.completion_open = !self.completions.is_empty();
        if self.completion_sel >= self.completions.len() {
            self.completion_sel = 0;
        }
    }

    fn accept_completion(&mut self) {
        let Some(item) = self.completions.get(self.completion_sel).cloned() else {
            self.completion_open = false;
            return;
        };
        let (next, caret) = complete::apply(&self.code, self.completion_start, self.cursor_bytes, &item);
        self.code = next;
        self.cursor_bytes = caret;
        self.pending_cursor = Some(caret);
        self.completion_open = false;
    }
}

impl eframe::App for EngineerCraft {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.poll_run(ctx);
        self.poll_lint(ctx);
        self.poll_debug(ctx);
        self.poll_update(ctx);
        self.handle_keys(ctx);
        self.tick_lang(ctx);
        self.tick_theme(ctx);
        self.apply_zoom(ctx);
        self.poll_autosave(ctx);

        let bg = self.canvas_bg();
        egui::TopBottomPanel::top("top_bar")
            .frame(Frame::NONE.fill(bg).stroke(hair(pal().line)))
            .show(ctx, |ui| self.top_bar(ui));

        egui::SidePanel::left("exercises")
            .resizable(true)
            .default_width(280.0)
            .min_width(170.0)
            .frame(Frame::NONE.fill(pal().panel).stroke(hair(pal().line)))
            .show(ctx, |ui| self.sidebar(ui));

        egui::SidePanel::right("brief")
            .resizable(true)
            .default_width(320.0)
            .min_width(180.0)
            .frame(Frame::NONE.fill(pal().panel).stroke(hair(pal().line)))
            .show(ctx, |ui| self.brief(ui));

        if self.show_result {
            egui::TopBottomPanel::bottom("result")
                .resizable(true)
                .default_height(240.0)
                .min_height(110.0)
                .frame(Frame::NONE.fill(pal().panel).stroke(hair(pal().line)))
                .show(ctx, |ui| self.console_panel(ui));
        }

        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(bg))
            .show(ctx, |ui| {
                paint_grid(ui);
                self.editor(ui);
            });

        self.completion_popup(ctx);
        self.watch_window(ctx);
        self.lang_overlay(ctx);
        self.theme_overlay(ctx);
        self.news_overlay(ctx);
        self.settings_overlay(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_debug();
        self.persist_draft();
    }
}

impl EngineerCraft {
    fn top_bar(&mut self, ui: &mut Ui) {
        let lang = self.lang;
        let done = self
            .catalog
            .iter()
            .filter(|e| !e.is_free() && self.progress.is_complete(self.code_lang, &e.id))
            .count();
        let total = self.catalog.iter().filter(|e| !e.is_free()).count().max(1);
        let ratio = done as f32 / total as f32;

        Frame::NONE
            .inner_margin(Margin::symmetric(16, 10))
            .show(ui, |ui| {
                ui.spacing_mut().button_padding = Vec2::new(8.0, 4.0);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("01 / STUDIO")
                                .monospace()
                                .size(10.0)
                                .color(pal().accent),
                        );
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("EngineerCraft")
                                    .font(FontId::new(22.0, theme::display_family()))
                                    .color(pal().text),
                            );
                            if ui
                                .add(
                                    Label::new(
                                        RichText::new(format!("v{}", crate::version::APP_VERSION))
                                            .monospace()
                                            .size(11.0)
                                            .color(pal().accent),
                                    )
                                    .sense(Sense::click()),
                                )
                                .on_hover_text(tr(lang, "Novedades", "What's new"))
                                .clicked()
                            {
                                self.show_news = true;
                            }
                            ui.add_space(8.0);
                            ui.add(
                                egui::Hyperlink::from_label_and_url(
                                    RichText::new("GitHub ↗")
                                        .monospace()
                                        .size(12.0)
                                        .color(pal().accent),
                                    update::REPO_URL,
                                )
                                .open_in_new_tab(true),
                            );
                            if self.update_busy {
                                ui.label(
                                    RichText::new(tr(lang, "DESCARGANDO…", "DOWNLOADING…"))
                                        .monospace()
                                        .size(11.0)
                                        .color(pal().warn),
                                );
                            } else if let Some(p) = &self.update_pending {
                                let label = format!(
                                    "{} v{}",
                                    tr(lang, "ACTUALIZAR", "UPDATE"),
                                    p.version
                                );
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(label).monospace().size(11.0).color(pal().bg),
                                        )
                                        .fill(pal().accent)
                                        .stroke(Stroke::NONE),
                                    )
                                    .clicked()
                                {
                                    self.start_update(ui.ctx());
                                }
                            }
                        });
                        self.medal_row(ui);
                    });
                    ui.add_space(16.0);
                    ui.vertical(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!(
                                "{done:02} / {total:02}  {}",
                                tr(lang, "COMPLETADOS", "DONE")
                            ))
                            .monospace()
                            .size(11.0)
                            .color(pal().dim),
                        );
                        let bar = ui.available_width().min(220.0);
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::new(bar, 3.0), Sense::hover());
                        ui.painter().rect_filled(rect, 0.0, pal().accent_dim);
                        let mut fill = rect;
                        fill.set_width(rect.width() * ratio);
                        ui.painter().rect_filled(fill, 0.0, pal().accent);
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let dark = pal().dark;
                        let theme_resp = theme_icon_button(ui, dark);
                        if theme_resp
                            .on_hover_text(if dark {
                                tr(lang, "Tema claro", "Light theme")
                            } else {
                                tr(lang, "Tema oscuro", "Dark theme")
                            })
                            .clicked()
                        {
                            self.set_dark(ui.ctx(), !dark);
                        }
                        ui.add_space(8.0);
                        let tool = self
                            .jdk
                            .as_ref()
                            .map(|t| t.label.clone())
                            .unwrap_or_else(|| {
                                tr(lang, "sin toolchain", "no toolchain").into()
                            });
                        ui.label(
                            RichText::new(tool)
                                .monospace()
                                .size(11.0)
                                .color(if self.jdk.is_some() { pal().ok } else { pal().err }),
                        );
                        ui.add_space(8.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(tr(lang, "AJUSTES", "SETTINGS"))
                                        .monospace()
                                        .size(11.0)
                                        .color(pal().text),
                                )
                                .fill(if self.show_settings {
                                    pal().accent_dim
                                } else {
                                    Color32::TRANSPARENT
                                })
                                .stroke(hair(pal().accent)),
                            )
                            .clicked()
                        {
                            self.show_settings = !self.show_settings;
                        }
                    });
                });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                        for cl in CodeLang::ALL {
                            let on = self.code_lang == cl;
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(cl.label())
                                            .monospace()
                                            .size(11.0)
                                            .color(if on { pal().bg } else { pal().text }),
                                    )
                                    .fill(if on {
                                        pal().accent
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(hair(if on { pal().accent } else { pal().line })),
                                )
                                .clicked()
                            {
                                self.set_code_lang(cl);
                            }
                        }
                        ui.add_space(12.0);
                        let lang_btn = if lang == Lang::Es { "ES · en" } else { "es · EN" };
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(lang_btn).monospace().size(11.0).color(pal().text),
                                )
                                .fill(if self.lang_anim.is_some() {
                                    pal().accent_dim
                                } else {
                                    Color32::TRANSPARENT
                                })
                                .stroke(hair(pal().accent)),
                            )
                            .clicked()
                        {
                            self.request_lang(lang.other());
                        }
                        ui.add_space(12.0);
                        let adv_on = self.advanced();
                        let adv = ui.add(
                            egui::Button::new(
                                RichText::new(tr(lang, "AVANZADO", "ADVANCED"))
                                    .monospace()
                                    .size(11.0)
                                    .color(if adv_on { pal().bg } else { pal().dim }),
                            )
                            .fill(if adv_on { pal().accent } else { Color32::TRANSPARENT })
                            .stroke(hair(if adv_on { pal().accent } else { pal().line })),
                        );
                        let simple = ui.add_enabled(
                            !self.is_free(),
                            egui::Button::new(
                                RichText::new(tr(lang, "SIMPLE", "SIMPLE"))
                                    .monospace()
                                    .size(11.0)
                                    .color(if adv_on { pal().dim } else { pal().bg }),
                            )
                            .fill(if adv_on {
                                Color32::TRANSPARENT
                            } else {
                                pal().accent
                            })
                            .stroke(hair(if adv_on { pal().line } else { pal().accent })),
                        );
                        if simple.clicked() {
                            self.set_full_file(false);
                        }
                        if adv.clicked() {
                            self.set_full_file(true);
                        }
                        ui.add_space(12.0);
                        let run_label = if self.running {
                            tr(lang, "COMPILANDO", "COMPILING")
                        } else {
                            tr(lang, "CORRER  F5", "RUN  F5")
                        };
                        let run = ui.add_enabled(
                            !self.running && self.current().is_some() && self.tool_ready(),
                            egui::Button::new(
                                RichText::new(run_label).monospace().size(12.0).color(pal().bg),
                            )
                            .fill(pal().accent)
                            .stroke(Stroke::NONE),
                        );
                        if run.clicked() {
                            self.run();
                        }
                        let debugging = self.debug_tx.is_some();
                        if debugging {
                            let step_label = if self.debug_busy {
                                tr(lang, "CARGANDO…", "LOADING…")
                            } else {
                                tr(lang, "SIGUIENTE  F10", "NEXT  F10")
                            };
                            if ui
                                .add_enabled(
                                    !self.debug_busy,
                                    egui::Button::new(
                                        RichText::new(step_label).monospace().size(11.0).color(pal().bg),
                                    )
                                    .fill(pal().warn)
                                    .stroke(Stroke::NONE),
                                )
                                .clicked()
                            {
                                self.debug_step();
                            }
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(tr(lang, "DETENER", "STOP")).monospace().size(11.0).color(pal().text),
                                    )
                                    .fill(Color32::TRANSPARENT)
                                    .stroke(hair(pal().err)),
                                )
                                .clicked()
                            {
                                self.stop_debug();
                                if !self.advanced() {
                                    self.show_result = false;
                                }
                            }
                        } else if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(tr(lang, "PASO A PASO  F10", "STEP  F10"))
                                        .monospace()
                                        .size(11.0)
                                        .color(pal().text),
                                )
                                .fill(Color32::TRANSPARENT)
                                .stroke(hair(pal().accent)),
                            )
                            .clicked()
                        {
                            self.start_debug();
                        }
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(tr(lang, "RESTAURAR", "RESET")).monospace().size(11.0).color(pal().dim),
                                )
                                .fill(Color32::TRANSPARENT)
                                .stroke(hair(pal().line)),
                            )
                            .clicked()
                        {
                            self.reset_starter();
                        }
                });
            });
    }

    fn sidebar(&mut self, ui: &mut Ui) {
                Frame::NONE
                    .fill(pal().panel)
                    .inner_margin(Margin::symmetric(14, 14))
                    .show(ui, |ui| {
                        ui.set_min_height(ui.available_height());
                        ui.set_width(ui.available_width());
                        ui.label(
                            RichText::new(tr(self.lang, "02 / EJERCICIOS", "02 / EXERCISES"))
                                .monospace()
                                .size(10.0)
                                .color(pal().accent),
                        );
                        ui.add_space(8.0);
                        ui.add(
                            TextEdit::singleline(&mut self.filter)
                                .hint_text(tr(self.lang, "filtrar", "filter"))
                                .font(FontId::monospace(12.0))
                                .desired_width(f32::INFINITY),
                        );
                        ui.add_space(8.0);

                        if let Some(err) = &self.load_error {
                            ui.colored_label(pal().err, err);
                            return;
                        }
                        if !self.tool_ready() {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(tr(
                                    self.lang,
                                    "Instala el lenguaje para ver los ejercicios.",
                                    "Install the language to see the exercises.",
                                ))
                                .color(pal().err)
                                .size(13.0),
                            );
                            return;
                        }

                        let filter = self.filter.to_ascii_lowercase();
                        let mut last_level = 0u8;
                        let mut click = None;
                        ScrollArea::vertical()
                            .id_salt("exercise_list")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for (idx, ex) in self.catalog.iter().enumerate() {
                                    if !filter.is_empty() {
                                        let hay = format!(
                                            "{} {} {}",
                                            ex.id,
                                            ex.title_es,
                                            ex.title_en
                                        )
                                        .to_ascii_lowercase();
                                        if !hay.contains(&filter) {
                                            continue;
                                        }
                                    }
                                    if ex.level != last_level {
                                        ui.add_space(10.0);
                                        ui.label(
                                            RichText::new(format!(
                                                "{} {}  ·  {}",
                                                tr(self.lang, "NIVEL", "LEVEL"),
                                                ex.level,
                                                ex.level_name(self.lang)
                                            ))
                                            .monospace()
                                            .size(10.0)
                                            .color(pal().muted),
                                        );
                                        ui.add_space(4.0);
                                        last_level = ex.level;
                                    }
                                    let done = self.progress.is_complete(self.code_lang, &ex.id);
                                    let selected = idx == self.selected;
                                    let row = exercise_row(ui, ex, done, selected, self.lang);
                                    if row.clicked() {
                                        click = Some(idx);
                                    }
                                }
                            });
                        if let Some(idx) = click {
                            self.select(idx);
                        }
                    });
    }

    fn requirements_panel(&mut self, ui: &mut Ui) {
        let g = codelang::guide(self.code_lang, self.lang);
        Frame::NONE
            .inner_margin(Margin::symmetric(24, 18))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    RichText::new(tr(self.lang, "REQUISITOS", "REQUIREMENTS"))
                        .monospace()
                        .size(10.0)
                        .color(pal().accent),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(g.title)
                        .font(FontId::new(22.0, FontFamily::Proportional))
                        .color(pal().text),
                );
                ui.add_space(8.0);
                ui.label(RichText::new(g.why).size(14.0).color(pal().dim));
                ui.add_space(14.0);
                ui.label(
                    RichText::new(tr(self.lang, "Sitio oficial", "Official site"))
                        .monospace()
                        .size(11.0)
                        .color(pal().muted),
                );
                if ui
                    .link(RichText::new(g.official_label).color(pal().accent).size(14.0))
                    .clicked()
                {
                    codelang::open_url(g.official);
                }
                ui.add_space(12.0);
                ui.label(
                    RichText::new(tr(self.lang, "Instalar en esta máquina", "Install on this machine"))
                        .monospace()
                        .size(11.0)
                        .color(pal().muted),
                );
                ui.label(
                    RichText::new(g.cmd)
                        .monospace()
                        .size(14.0)
                        .color(pal().text),
                );
                ui.add_space(8.0);
                if ui
                    .link(RichText::new(tr(self.lang, "Documentación de instalación ↗", "Install docs ↗")).color(pal().accent))
                    .clicked()
                {
                    codelang::open_url(g.docs);
                }
                ui.add_space(16.0);
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(tr(self.lang, "YA LO INSTALÉ — DETECTAR", "I INSTALLED IT — DETECT"))
                                .monospace()
                                .size(12.0)
                                .color(pal().bg),
                        )
                        .fill(pal().accent),
                    )
                    .clicked()
                {
                    self.jdk = codelang::detect(self.code_lang);
                    if self.tool_ready() {
                        self.reload_editor_code();
                        self.mark_dirty();
                    }
                }
            });
    }

    fn editor(&mut self, ui: &mut Ui) {
        if !self.tool_ready() {
            self.requirements_panel(ui);
            return;
        }
        let height = ui.available_height();
        Frame::NONE
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.set_min_height(height);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(if self.advanced() {
                            format!("03 / STARTER.{}", self.code_lang.ext().to_ascii_uppercase())
                        } else {
                            tr(self.lang, "03 / TU CÓDIGO", "03 / YOUR CODE").into()
                        })
                        .monospace()
                        .size(10.0)
                        .color(pal().accent),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(tr(
                                self.lang,
                                "Ctrl+Espacio autocompleta  ·  Tab acepta",
                                "Ctrl+Space to complete  ·  Tab accepts",
                            ))
                                .monospace()
                                .size(10.0)
                                .color(pal().muted),
                        );
                    });
                });
                ui.label(
                    RichText::new(&self.practice_hint)
                        .size(12.5)
                        .color(pal().accent),
                );
                ui.add_space(6.0);

                let font = FontId::monospace(self.progress.font_size.clamp(11.0, 28.0));
                let font_layout = font.clone();
                let error_lines = self.error_lines.clone();
                let debug_line = self.debug_line;
                let code_lang = self.code_lang;
                let mut layouter = move |ui: &Ui, text: &dyn egui::TextBuffer, _wrap: f32| {
                    let mut job: LayoutJob = highlight::java(
                        text.as_str(),
                        &error_lines,
                        debug_line,
                        font_layout.clone(),
                        code_lang,
                    );
                    job.wrap.max_width = f32::INFINITY;
                    ui.fonts(|f| f.layout_job(job))
                };

                let lines = self.code.lines().count().max(1);
                ScrollArea::both()
                    .id_salt("editor_scroll")
                    .max_height((height - 36.0).max(80.0))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            let mut nums = LayoutJob::default();
                            for i in 1..=lines.max(24) {
                                let bad = self.error_lines.contains(&i);
                                let dbg = self.debug_line == Some(i);
                                let inv = pal().opposite();
                                nums.append(
                                    &format!("{}{i:>3}\n", if dbg { "▶" } else { " " }),
                                    0.0,
                                    egui::text::TextFormat {
                                        font_id: font.clone(),
                                        color: if bad {
                                            inv.err
                                        } else if dbg {
                                            pal().warn
                                        } else {
                                            pal().muted
                                        },
                                        background: if bad {
                                            inv.bg
                                        } else if dbg {
                                            pal().debug_bg
                                        } else {
                                            Color32::TRANSPARENT
                                        },
                                        ..Default::default()
                                    },
                                );
                            }
                            let gutter = ui.add(Label::new(nums).selectable(false).sense(Sense::click()));
                            let rows = lines.max(24).max(1);
                            ui.add_space(8.0);

                            if let Some(byte) = self.pending_cursor.take() {
                                let chars = self.code[..byte.min(self.code.len())].chars().count();
                                let mut state =
                                    TextEditState::load(ui.ctx(), self.editor_id).unwrap_or_default();
                                state.cursor.set_char_range(Some(CCursorRange::one(
                                    CCursor::new(chars),
                                )));
                                state.store(ui.ctx(), self.editor_id);
                            }

                            let output = TextEdit::multiline(&mut self.code)
                                .id(self.editor_id)
                                .font(font.clone())
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .desired_rows(lines.max(24))
                                .frame(false)
                                .lock_focus(true)
                                .layouter(&mut layouter)
                                .show(ui);

                            let selecting = output
                                .cursor_range
                                .is_some_and(|r| r.primary != r.secondary);
                            if let Some(range) = output.cursor_range {
                                let char_idx = range.primary.index;
                                self.cursor_bytes = self
                                    .code
                                    .char_indices()
                                    .nth(char_idx)
                                    .map(|(i, _)| i)
                                    .unwrap_or(self.code.len());
                                let cursor_rect = output.galley.pos_from_cursor(range.primary);
                                self.popup_pos = Some(
                                    output.galley_pos
                                        + Vec2::new(cursor_rect.left(), cursor_rect.bottom() + 4.0),
                                );
                            }
                            if self.completion_open && output.response.clicked() {
                                self.refresh_completions();
                            }
                            if output.response.changed() && !self.running {
                                if let Some((next, cur)) =
                                    crate::pair::autoclose(
                                        &self.code,
                                        self.cursor_bytes,
                                        self.code_lang,
                                    )
                                {
                                    self.code = next;
                                    self.cursor_bytes = cur;
                                    self.pending_cursor = Some(cur);
                                }
                                self.refresh_completions();
                                self.mark_dirty();
                                if matches!(self.console_kind, ConsoleKind::Ok) {
                                    self.console_kind = ConsoleKind::Idle;
                                    self.console = tr(
                                        self.lang,
                                        "Código modificado. Vuelve a correr.",
                                        "Code changed. Run again.",
                                    )
                                    .into();
                                }
                            }
                            let pointer = ui.ctx().pointer_hover_pos();
                            let editor_line = pointer.and_then(|pos| {
                                if output.response.rect.contains(pos) {
                                    line_at_galley(
                                        output.galley.as_ref(),
                                        output.galley_pos,
                                        pos,
                                        &self.code,
                                    )
                                } else {
                                    None
                                }
                            });
                            let gutter_line = pointer.and_then(|pos| {
                                if gutter.rect.contains(pos) {
                                    let h = gutter.rect.height() / rows as f32;
                                    if h <= 0.5 {
                                        return None;
                                    }
                                    let i = ((pos.y - gutter.rect.top()) / h).floor() as usize + 1;
                                    Some(i.clamp(1, rows))
                                } else {
                                    None
                                }
                            });
                            let hover_err = gutter_line.or(editor_line);
                            if let Some(i) = hover_err {
                                if let Some(msg) = compiler_msg(&self.error_cards, i) {
                                    show_line_error(
                                        ui,
                                        output.galley.as_ref(),
                                        output.galley_pos,
                                        &self.code,
                                        i,
                                        msg,
                                    );
                                }
                            }
                            if self.debug_line.is_some() {
                                if let Some(line) = self.debug_line {
                                    paint_debug_line(
                                        ui,
                                        output.galley.as_ref(),
                                        output.galley_pos,
                                        &self.code,
                                        line,
                                    );
                                }
                            } else if !self.completion_open && !self.advanced() && !selecting {
                                if output.response.changed() {
                                    let g = practice::after_edit(
                                        &self.code,
                                        self.cursor_bytes,
                                        self.lang,
                                        self.code_lang,
                                    );
                                    self.practice_hint = g.hint;
                                    if g.code != self.code {
                                        self.code = g.code;
                                    }
                                    if g.cursor != self.cursor_bytes {
                                        self.cursor_bytes = g.cursor;
                                        self.pending_cursor = Some(g.cursor);
                                    }
                                }
                                let line = practice::line_index(
                                    &self.code,
                                    self.cursor_bytes,
                                );
                                if self.snap_line != Some(line) {
                                    self.snap_line = Some(line);
                                    let g = practice::on_arrive(
                                        &self.code,
                                        self.cursor_bytes,
                                        self.lang,
                                        self.code_lang,
                                    );
                                    self.practice_hint = g.hint;
                                    if g.code != self.code {
                                        self.code = g.code;
                                    }
                                    if g.cursor != self.cursor_bytes {
                                        self.cursor_bytes = g.cursor;
                                        self.pending_cursor = Some(g.cursor);
                                    }
                                } else {
                                    self.practice_hint =
                                        practice::hint_at(
                                            &self.code,
                                            self.cursor_bytes,
                                            self.lang,
                                            self.code_lang,
                                        );
                                }
                            }
                        });
                    });
            });
    }

    fn console_panel(&mut self, ui: &mut Ui) {
        let raw = self.advanced();
        let (kicker_color, kicker) = if raw {
            match self.console_kind {
                ConsoleKind::Run => (pal().warn, tr(self.lang, "04 / CONSOLA  ·  COMPILANDO", "04 / CONSOLE  ·  COMPILING")),
                ConsoleKind::Err => (pal().err, tr(self.lang, "04 / CONSOLA  ·  ERROR", "04 / CONSOLE  ·  ERROR")),
                ConsoleKind::Debug => (pal().warn, tr(self.lang, "04 / CONSOLA  ·  DEBUG", "04 / CONSOLE  ·  DEBUG")),
                _ => (pal().dim, tr(self.lang, "04 / CONSOLA", "04 / CONSOLE")),
            }
        } else {
            match self.console_kind {
                ConsoleKind::Idle => (pal().dim, tr(self.lang, "04 / RESULTADO", "04 / RESULT")),
                ConsoleKind::Run => (pal().warn, tr(self.lang, "04 / RESULTADO  ·  COMPILANDO", "04 / RESULT  ·  COMPILING")),
                ConsoleKind::Ok => (pal().ok, tr(self.lang, "04 / RESULTADO  ·  CORRECTO", "04 / RESULT  ·  OK")),
                ConsoleKind::Err => (pal().err, tr(self.lang, "04 / RESULTADO  ·  ERROR", "04 / RESULT  ·  ERROR")),
                ConsoleKind::Debug => (pal().warn, tr(self.lang, "04 / RESULTADO  ·  DEBUG", "04 / RESULT  ·  DEBUG")),
            }
        };
        let fill = if raw {
            pal().console
        } else {
            match self.console_kind {
                ConsoleKind::Ok => pal().bg_ok,
                ConsoleKind::Err => pal().opposite().bg,
                ConsoleKind::Debug => pal().bg_debug,
                _ => pal().panel,
            }
        };
        Frame::NONE
            .fill(fill)
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.set_min_height(ui.available_height().max(120.0));
                ui.set_width(ui.available_width());
                let err_p = if !raw && self.console_kind == ConsoleKind::Err {
                    pal().opposite()
                } else {
                    pal()
                };
                ui.label(RichText::new(kicker).monospace().size(10.0).color(if !raw && self.console_kind == ConsoleKind::Err { err_p.err } else { kicker_color }));
                ui.add_space(4.0);
                if !self.console.is_empty() && !(raw && self.is_free()) {
                    ui.label(
                        RichText::new(&self.console)
                            .font(FontId::new(16.0, FontFamily::Proportional))
                            .color(err_p.text),
                    );
                    ui.add_space(8.0);
                }
                ScrollArea::vertical()
                    .id_salt("result_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        for card in &self.error_cards {
                            let inv = pal().opposite();
                            Frame::NONE
                                .fill(inv.bg)
                                .inner_margin(Margin::same(10))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.label(
                                        RichText::new(format!("Línea {}", card.line))
                                            .monospace()
                                            .size(11.0)
                                            .color(inv.err),
                                    );
                                    ui.add(
                                        Label::new(
                                            RichText::new(&card.message).color(inv.text).size(14.0),
                                        )
                                        .wrap(),
                                    );
                                    if !card.extra.is_empty() {
                                        ui.label(RichText::new(&card.extra).color(inv.dim).size(13.0));
                                    }
                                });
                            ui.add_space(6.0);
                        }
                        if raw {
                            let dump = if self.output_lines.is_empty() {
                                self.console.clone()
                            } else {
                                self.output_lines.join("\n")
                            };
                            ui.add(
                                Label::new(
                                    RichText::new(dump)
                                        .monospace()
                                        .size(13.0)
                                        .color(pal().stdout),
                                )
                                .wrap()
                                .selectable(true),
                            );
                        } else {
                            for line in &self.output_lines {
                                Frame::NONE
                                    .fill(pal().bg_ok)
                                    .inner_margin(Margin::symmetric(12, 8))
                                    .show(ui, |ui| {
                                        ui.set_width(ui.available_width());
                                        ui.add(
                                            Label::new(
                                                RichText::new(line)
                                                    .font(FontId::new(15.0, FontFamily::Proportional))
                                                    .color(pal().text),
                                            )
                                            .wrap(),
                                        );
                                    });
                                ui.add_space(4.0);
                            }
                        }
                    });
            });
    }

    fn brief(&mut self, ui: &mut Ui) {
                Frame::NONE
                    .fill(pal().panel)
                    .inner_margin(Margin::symmetric(16, 14))
                    .show(ui, |ui| {
                        ui.set_min_height(ui.available_height());
                        ui.set_width(ui.available_width());
                        ui.label(
                            RichText::new(tr(self.lang, "05 / BRIEF", "05 / BRIEF"))
                                .monospace()
                                .size(10.0)
                                .color(pal().accent),
                        );
                        ui.add_space(8.0);
                        if let Some(ex) = self.current() {
                            let brief = ex.brief(self.lang).to_string();
                            let title = format!("{}  {}", ex.id, ex.title(self.lang));
                            ui.label(
                                RichText::new(title)
                                    .font(FontId::new(16.0, FontFamily::Proportional))
                                    .color(pal().text),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(format!(
                                    "{} {} · {}",
                                    tr(self.lang, "NIVEL", "LEVEL"),
                                    ex.level,
                                    ex.level_name(self.lang)
                                ))
                                    .monospace()
                                    .size(10.0)
                                    .color(pal().dim),
                            );
                            ui.add_space(10.0);
                            ScrollArea::vertical()
                                .id_salt("brief_scroll")
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    markdown::show(ui, &brief);
                                });
                        } else {
                            ui.label(tr(self.lang, "No hay ejercicios cargados.", "No exercises loaded."));
                        }
                    });
    }

    fn completion_popup(&mut self, ctx: &Context) {
        if !self.completion_open || self.completions.is_empty() {
            return;
        }
        let Some(pos) = self.popup_pos else { return };
        let mut chosen = None;
        let popup = egui::Area::new(egui::Id::new("completion_popup"))
            .fixed_pos(pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                Frame::NONE
                    .fill(pal().panel_2)
                    .stroke(hair(pal().accent))
                    .inner_margin(Margin::same(6))
                    .show(ui, |ui| {
                        ui.set_min_width(360.0);
                        ui.set_max_width(420.0);
                        ui.label(
                            RichText::new("AUTOCOMPLETE")
                                .monospace()
                                .size(9.0)
                                .color(pal().accent),
                        );
                        ui.add_space(4.0);
                        for (i, item) in self.completions.iter().enumerate() {
                            let selected = i == self.completion_sel;
                            let bg = if selected { pal().accent_dim } else { Color32::TRANSPARENT };
                            let (rect, resp) = ui.allocate_exact_size(
                                Vec2::new(ui.available_width(), 22.0),
                                Sense::click(),
                            );
                            ui.painter().rect_filled(rect, 0.0, bg);
                            ui.painter().text(
                                rect.left_center() + Vec2::new(8.0, 0.0),
                                egui::Align2::LEFT_CENTER,
                                format!("{:<16} {}", item.label, item.detail),
                                FontId::monospace(12.0),
                                if selected { pal().text } else { pal().dim },
                            );
                            if resp.clicked() {
                                chosen = Some(i);
                            }
                        }
                        if let Some(item) = self.completions.get(self.completion_sel) {
                            ui.add_space(6.0);
                            let r = ui.available_rect_before_wrap();
                            ui.painter().hline(r.x_range(), r.top(), hair(pal().line));
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(&item.doc)
                                    .size(12.5)
                                    .color(pal().text),
                            );
                        }
                    });
            });
        if let Some(i) = chosen {
            self.completion_sel = i;
            self.accept_completion();
            return;
        }
        if ctx.input(|i| i.pointer.primary_pressed()) {
            if let Some(p) = ctx.pointer_latest_pos() {
                if !popup.response.rect.expand(6.0).contains(p) {
                    self.completion_open = false;
                }
            } else {
                self.completion_open = false;
            }
        }
    }
}

fn exercise_row(
    ui: &mut Ui,
    ex: &Exercise,
    done: bool,
    selected: bool,
    lang: Lang,
) -> egui::Response {
    let id = ui.id().with(("exercise_row", ex.number));
    let hovered = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));
    let bg = if selected {
        pal().accent_dim
    } else if hovered {
        pal().panel_2
    } else {
        Color32::TRANSPARENT
    };
    let mark = if done { "●" } else { "○" };
    let mark_color = if done { pal().ok } else { pal().muted };
    let title = format!("{:02}  {}", ex.number, ex.title(lang));
    let max_w = ui.available_width();
    let inner = Frame::NONE
        .fill(bg)
        .inner_margin(Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_max_width((max_w - 4.0).max(40.0));
            ui.horizontal(|ui| {
                ui.add(
                    Label::new(RichText::new(mark).monospace().color(mark_color)).selectable(false),
                );
                ui.add(
                    Label::new(RichText::new(title).color(pal().text).size(12.5))
                        .wrap()
                        .selectable(false)
                        .sense(Sense::click()),
                );
            });
        });
    let resp = inner.response.interact(Sense::click());
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    ui.data_mut(|d| d.insert_temp(id, resp.hovered()));
    resp
}

fn compiler_msg(cards: &[Diag], line: usize) -> Option<&str> {
    cards.iter().find(|c| c.line == line).map(|c| c.message.as_str())
}

fn line_at_galley(galley: &egui::Galley, origin: Pos2, pos: Pos2, code: &str) -> Option<usize> {
    let rel = pos - origin;
    let n = code.lines().count().max(1);
    for i in 1..=n {
        let ch = code[..byte_at_line(code, i)].chars().count();
        let r = galley.pos_from_cursor(CCursor::new(ch));
        if rel.y >= r.top() && rel.y < r.bottom() {
            return Some(i);
        }
    }
    None
}

fn show_line_error(
    ui: &Ui,
    galley: &egui::Galley,
    origin: Pos2,
    code: &str,
    line: usize,
    msg: &str,
) {
    let ch = code[..byte_at_line(code, line)].chars().count();
    let r = galley.pos_from_cursor(CCursor::new(ch));
    let pos = Pos2::new(origin.x, origin.y + r.bottom());
    let inv = pal().opposite();
    let mut job = LayoutJob::default();
    job.wrap.max_width = 460.0;
    job.append(
        &format!("Línea {line}\n"),
        0.0,
        egui::text::TextFormat {
            font_id: FontId::monospace(11.0),
            color: inv.err,
            ..Default::default()
        },
    );
    job.append(
        msg,
        0.0,
        egui::text::TextFormat {
            font_id: FontId::new(13.0, FontFamily::Proportional),
            color: inv.text,
            ..Default::default()
        },
    );
    let galley = ui.fonts(|f| f.layout_job(job));
    let size = galley.size() + Vec2::new(20.0, 16.0);
    let rect = egui::Rect::from_min_size(pos, size);
    let painter = ui.ctx().layer_painter(LayerId::new(Order::Foreground, egui::Id::new("line_err")));
    painter.rect_filled(rect, 0.0, inv.bg);
    painter.galley(rect.min + Vec2::new(10.0, 6.0), galley, inv.text);
}

fn byte_at_line(code: &str, line: usize) -> usize {
    let mut n = 1usize;
    for (i, _) in code.char_indices() {
        if n == line {
            return i;
        }
        if code.as_bytes().get(i) == Some(&b'\n') {
            n += 1;
        }
    }
    code.len()
}

fn paint_debug_line(ui: &Ui, galley: &egui::Galley, origin: Pos2, code: &str, line: usize) {
    let chars = byte_at_line(code, line);
    let char_idx = code[..chars.min(code.len())].chars().count();
    let r = galley.pos_from_cursor(CCursor::new(char_idx));
    let y0 = origin.y + r.top();
    let h = r.height().max(16.0);
    let rect = egui::Rect::from_min_max(
        Pos2::new(origin.x - 6.0, y0),
        Pos2::new(ui.max_rect().right(), y0 + h),
    );
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(214, 176, 110, 70));
    painter.rect_filled(
        egui::Rect::from_min_size(Pos2::new(origin.x - 8.0, y0), Vec2::new(5.0, h)),
        0.0,
        pal().warn,
    );
}

impl EngineerCraft {
    fn medal_row(&mut self, ui: &mut Ui) {
        let mut click = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            for cl in CodeLang::ALL {
                let n = self.progress.complete_count(cl);
                if n == 0 {
                    continue;
                }
                let on = self.code_lang == cl;
                let resp = medal_chip(ui, cl, n, on, self.lang);
                if resp.clicked() {
                    click = Some(cl);
                }
            }
        });
        if let Some(cl) = click {
            self.set_code_lang(cl);
        }
    }
}

fn medal_chip(ui: &mut Ui, lang: CodeLang, n: usize, on: bool, ui_lang: Lang) -> egui::Response {
    let label = format!("{} {n}", lang.label());
    let galley = ui.painter().layout_no_wrap(
        label.clone(),
        FontId::monospace(10.0),
        pal().text,
    );
    let w = 18.0 + galley.size().x + 8.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 16.0), Sense::click());
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let c = Pos2::new(rect.left() + 8.0, rect.center().y);
    paint_medal(ui, c, on);
    ui.painter().text(
        Pos2::new(c.x + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::monospace(10.0),
        if on { pal().accent } else { pal().dim },
    );
    resp.on_hover_text(tr(
        ui_lang,
        "Completados en este lenguaje",
        "Completed in this language",
    ))
}

fn paint_medal(ui: &Ui, c: Pos2, on: bool) {
    let gold = if on {
        pal().warn
    } else {
        pal().accent
    };
    let p = ui.painter();
    p.circle_filled(c, 6.2, gold);
    p.circle_stroke(c, 6.2, Stroke::new(1.1_f32, pal().text));
    p.circle_filled(c, 3.2, pal().bg);
    p.circle_filled(c, 1.6, gold);
    let ribbon = Color32::from_rgb(
        gold.r().saturating_sub(30),
        gold.g().saturating_sub(40),
        gold.b().saturating_sub(10),
    );
    p.line_segment(
        [c + Vec2::new(-2.2, 5.2), c + Vec2::new(-3.4, 9.5)],
        Stroke::new(2.0_f32, ribbon),
    );
    p.line_segment(
        [c + Vec2::new(2.2, 5.2), c + Vec2::new(3.4, 9.5)],
        Stroke::new(2.0_f32, ribbon),
    );
}

fn theme_icon_button(ui: &mut Ui, dark: bool) -> egui::Response {
    let size = Vec2::splat(28.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        ui.painter().rect_filled(rect, 3.0, pal().accent_dim);
    }
    let c = rect.center();
    let col = pal().accent;
    let painter = ui.painter();
    if dark {
        painter.circle_filled(c, 5.0, col);
        for i in 0..8 {
            let a = i as f32 * std::f32::consts::PI / 4.0;
            let (s, co) = a.sin_cos();
            let inner = 7.5;
            let outer = 11.5;
            painter.line_segment(
                [
                    Pos2::new(c.x + co * inner, c.y + s * inner),
                    Pos2::new(c.x + co * outer, c.y + s * outer),
                ],
                Stroke::new(1.6_f32, col),
            );
        }
    } else {
        painter.circle_filled(c + Vec2::new(-1.5, 0.0), 7.5, col);
        let cut = if resp.hovered() { pal().accent_dim } else { pal().bg };
        painter.circle_filled(c + Vec2::new(3.2, -2.0), 6.2, cut);
    }
    resp
}

fn paint_grid(ui: &Ui) {
    let rect = ui.max_rect();
    let painter = ui.painter();
    let mut x = rect.left();
    let mut n = 0;
    while x <= rect.right() {
        let color = if n % 5 == 0 { pal().grid_strong } else { pal().grid };
        painter.vline(x, rect.y_range(), hair(color));
        x += 24.0;
        n += 1;
    }
    let mut y = rect.top();
    n = 0;
    while y <= rect.bottom() {
        let color = if n % 5 == 0 { pal().grid_strong } else { pal().grid };
        painter.hline(rect.x_range(), y, hair(color));
        y += 24.0;
        n += 1;
    }
}

fn stdout_lines(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

