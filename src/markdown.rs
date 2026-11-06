use crate::theme::{hair, pal};
use eframe::egui::{self, FontFamily, FontId, RichText};

pub fn show(ui: &mut egui::Ui, md: &str) {
    let p = pal();
    let mut in_code = false;
    let mut code = String::new();

    for line in md.lines() {
        if line.trim_start().starts_with("```") {
            if in_code {
                show_code(ui, &code);
                code.clear();
                in_code = false;
            } else {
                in_code = true;
            }
            continue;
        }
        if in_code {
            code.push_str(line);
            code.push('\n');
            continue;
        }
        if line.starts_with("# ") {
            ui.add_space(4.0);
            ui.label(
                RichText::new(line.trim_start_matches("# "))
                    .font(FontId::new(18.0, FontFamily::Proportional))
                    .color(p.text),
            );
            ui.add_space(6.0);
            continue;
        }
        if line.starts_with("## ") {
            ui.add_space(10.0);
            ui.label(
                RichText::new(line.trim_start_matches("## ").to_uppercase())
                    .font(FontId::new(11.0, FontFamily::Monospace))
                    .color(p.accent),
            );
            let rect = ui.available_rect_before_wrap();
            ui.painter()
                .hline(rect.x_range(), rect.top(), hair(p.line));
            ui.add_space(6.0);
            continue;
        }
        if line.starts_with("### ") {
            ui.label(
                RichText::new(line.trim_start_matches("### "))
                    .font(FontId::new(13.0, FontFamily::Proportional))
                    .color(p.text),
            );
            continue;
        }
        if let Some(rest) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            ui.horizontal(|ui| {
                ui.label(RichText::new("▪").color(p.accent).monospace());
                inline(ui, rest);
            });
            continue;
        }
        if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            && line.contains(". ")
        {
            if let Some((n, rest)) = line.split_once(". ") {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("{n}."))
                            .monospace()
                            .color(p.accent)
                            .size(12.0),
                    );
                    inline(ui, rest);
                });
                continue;
            }
        }
        if line.starts_with('|') {
            ui.label(RichText::new(line).monospace().size(11.0).color(p.dim));
            continue;
        }
        if line.trim().is_empty() {
            ui.add_space(6.0);
            continue;
        }
        inline(ui, line);
    }
    if in_code && !code.is_empty() {
        show_code(ui, &code);
    }
}

fn show_code(ui: &mut egui::Ui, code: &str) {
    let p = pal();
    egui::Frame::NONE
        .fill(p.panel_2)
        .stroke(hair(p.line))
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(code.trim_end())
                    .monospace()
                    .size(12.0)
                    .color(p.text),
            );
        });
    ui.add_space(6.0);
}

fn inline(ui: &mut egui::Ui, text: &str) {
    let p = pal();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let mut rest = text;
        while !rest.is_empty() {
            if let Some(after) = rest.strip_prefix("**") {
                if let Some(end) = after.find("**") {
                    ui.label(RichText::new(&after[..end]).strong().color(p.text));
                    rest = &after[end + 2..];
                    continue;
                }
            }
            if let Some(after) = rest.strip_prefix('`') {
                if let Some(end) = after.find('`') {
                    ui.label(
                        RichText::new(&after[..end])
                            .monospace()
                            .size(12.0)
                            .color(p.accent)
                            .background_color(p.panel_2),
                    );
                    rest = &after[end + 1..];
                    continue;
                }
            }
            let next = rest.find('`').unwrap_or(rest.len()).min(
                rest.find("**").unwrap_or(rest.len()),
            );
            if next == 0 {
                ui.label(RichText::new(&rest[..1]).color(p.text));
                rest = &rest[1..];
            } else {
                ui.label(RichText::new(&rest[..next]).color(p.text));
                rest = &rest[next..];
            }
        }
    });
}
