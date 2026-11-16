use eframe::egui::{self, Color32, CornerRadius, Frame, Margin, Stroke, Ui};
use crate::models::{ItemStatus, LogLevel};

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Check Windows system fonts: Segoe UI (Clean modern sans-serif), Segoe UI Symbol, and Consolas (Monospace)
    let segoe_path = "C:\\Windows\\Fonts\\segoeui.ttf";
    let segoe_bold_path = "C:\\Windows\\Fonts\\segoeuib.ttf";
    let seguisym_path = "C:\\Windows\\Fonts\\seguisym.ttf";
    let consolas_path = "C:\\Windows\\Fonts\\consola.ttf";

    let mut has_segoe = false;
    if let Ok(data) = std::fs::read(segoe_path) {
        fonts.font_data.insert(
            "segoe_ui".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
        has_segoe = true;
    }

    if let Ok(data) = std::fs::read(segoe_bold_path) {
        fonts.font_data.insert(
            "segoe_ui_bold".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
    }

    let mut has_symbol = false;
    if let Ok(data) = std::fs::read(seguisym_path) {
        fonts.font_data.insert(
            "segoe_ui_symbol".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
        has_symbol = true;
    }

    let mut has_consolas = false;
    if let Ok(data) = std::fs::read(consolas_path) {
        fonts.font_data.insert(
            "consolas".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
        has_consolas = true;
    }

    if has_segoe {
        if let Some(prop) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            prop.insert(0, "segoe_ui".to_owned());
        }
    }

    if has_symbol {
        if let Some(prop) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            prop.push("segoe_ui_symbol".to_owned());
        }
    }

    if has_consolas {
        if let Some(mono) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            mono.insert(0, "consolas".to_owned());
        }
    }

    ctx.set_fonts(fonts);

    // Apply refined typography scale
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Name("Subheading".into()), egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Button, egui::FontId::new(13.5, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(12.5, egui::FontFamily::Monospace)),
        (egui::TextStyle::Small, egui::FontId::new(11.5, egui::FontFamily::Proportional)),
    ].into();

    // Spacious comfortable layout defaults
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(13.0, 7.0);
    style.spacing.window_margin = Margin::same(14);
    style.spacing.indent = 16.0;

    ctx.set_style(style);
}

pub fn bg_main(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(17, 20, 29)
    } else {
        Color32::from_rgb(243, 245, 248)
    }
}

pub fn bg_card(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(25, 31, 45)
    } else {
        Color32::from_rgb(255, 255, 255)
    }
}

pub fn bg_card_light(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(33, 41, 59)
    } else {
        Color32::from_rgb(248, 250, 252)
    }
}

pub fn border_color(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(46, 58, 82)
    } else {
        Color32::from_rgb(222, 226, 233)
    }
}

pub fn accent_color(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(56, 189, 248) // Sky blue
    } else {
        Color32::from_rgb(2, 132, 199) // Sky 600
    }
}

pub fn accent_hover_color(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(125, 211, 252)
    } else {
        Color32::from_rgb(3, 105, 161)
    }
}

pub fn text_primary(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(243, 244, 246)
    } else {
        Color32::from_rgb(17, 24, 39)
    }
}

pub fn text_muted(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(156, 163, 175)
    } else {
        Color32::from_rgb(100, 116, 139)
    }
}

pub fn text_dimmed(dark: bool) -> Color32 {
    if dark {
        Color32::from_rgb(107, 114, 128)
    } else {
        Color32::from_rgb(156, 163, 175)
    }
}

pub const COLOR_SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
pub const COLOR_WARNING: Color32 = Color32::from_rgb(245, 158, 11);
pub const COLOR_ERROR: Color32 = Color32::from_rgb(239, 68, 68);
pub const COLOR_TEAL: Color32 = Color32::from_rgb(20, 184, 166);

pub fn apply_custom_theme(ctx: &egui::Context, dark_mode: bool) {
    let mut visuals = if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    visuals.panel_fill = bg_main(dark_mode);
    visuals.window_fill = bg_card(dark_mode);
    visuals.faint_bg_color = bg_card_light(dark_mode);
    visuals.extreme_bg_color = if dark_mode {
        Color32::from_rgb(13, 16, 23)
    } else {
        Color32::from_rgb(241, 245, 249)
    };

    let border = border_color(dark_mode);
    let txt = text_primary(dark_mode);
    let card_bg = bg_card(dark_mode);
    let card_light = bg_card_light(dark_mode);
    let accent = accent_color(dark_mode);
    let accent_hov = accent_hover_color(dark_mode);

    visuals.widgets.noninteractive.bg_fill = card_bg;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, border);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, txt);

    visuals.widgets.inactive.bg_fill = card_light;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, border);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, txt);

    visuals.widgets.hovered.bg_fill = if dark_mode {
        Color32::from_rgb(45, 56, 78)
    } else {
        Color32::from_rgb(224, 231, 255)
    };
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, accent);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, if dark_mode { Color32::WHITE } else { Color32::BLACK });

    visuals.widgets.active.bg_fill = if dark_mode {
        Color32::from_rgb(56, 70, 98)
    } else {
        Color32::from_rgb(199, 210, 254)
    };
    visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, accent_hov);
    visuals.widgets.active.corner_radius = CornerRadius::same(8);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, if dark_mode { Color32::WHITE } else { Color32::BLACK });

    visuals.selection.bg_fill = if dark_mode {
        Color32::from_rgba_premultiplied(56, 189, 248, 60)
    } else {
        Color32::from_rgba_premultiplied(2, 132, 199, 50)
    };
    visuals.selection.stroke = Stroke::new(1.0_f32, accent);

    ctx.set_visuals(visuals);
}

pub fn card_frame(dark_mode: bool) -> Frame {
    Frame {
        inner_margin: Margin::symmetric(18, 16),
        outer_margin: Margin::symmetric(0, 6),
        corner_radius: CornerRadius::same(10),
        fill: bg_card(dark_mode),
        stroke: Stroke::new(1.0_f32, border_color(dark_mode)),
        ..Default::default()
    }
}

#[allow(dead_code)]
pub fn subcard_frame(dark_mode: bool) -> Frame {
    Frame {
        inner_margin: Margin::symmetric(14, 12),
        outer_margin: Margin::symmetric(0, 4),
        corner_radius: CornerRadius::same(8),
        fill: bg_card_light(dark_mode),
        stroke: Stroke::new(1.0_f32, border_color(dark_mode)),
        ..Default::default()
    }
}

pub fn log_panel_frame(dark_mode: bool) -> Frame {
    Frame {
        inner_margin: Margin::symmetric(14, 10),
        outer_margin: Margin::ZERO,
        corner_radius: CornerRadius::same(8),
        fill: if dark_mode {
            Color32::from_rgb(13, 16, 23)
        } else {
            Color32::from_rgb(248, 250, 252)
        },
        stroke: Stroke::new(1.0_f32, border_color(dark_mode)),
        ..Default::default()
    }
}

pub fn render_status_badge(ui: &mut Ui, status: &ItemStatus) {
    match status {
        ItemStatus::Ready => {
            render_badge(ui, "Ready", Color32::from_rgb(75, 85, 99), Color32::from_rgb(243, 244, 246));
        }
        ItemStatus::Modified => {
            render_badge(ui, "Modified", Color32::from_rgb(180, 83, 9), Color32::WHITE);
        }
        ItemStatus::Success(msg) => {
            render_badge(ui, &format!("✓ {msg}"), Color32::from_rgb(22, 101, 52), Color32::WHITE);
        }
        ItemStatus::Warning(msg) => {
            render_badge(ui, &format!("⚠ {msg}"), Color32::from_rgb(180, 83, 9), Color32::WHITE);
        }
        ItemStatus::Error(msg) => {
            render_badge(ui, &format!("✗ {msg}"), Color32::from_rgb(185, 28, 28), Color32::WHITE);
        }
    }
}

pub fn render_badge(ui: &mut Ui, text: &str, bg_color: Color32, text_color: Color32) {
    let frame = Frame {
        inner_margin: Margin::symmetric(10, 4),
        corner_radius: CornerRadius::same(6),
        fill: bg_color,
        ..Default::default()
    };
    frame.show(ui, |ui| {
        ui.label(egui::RichText::new(text).color(text_color).size(11.5).strong());
    });
}

pub fn log_level_color(level: &LogLevel, dark_mode: bool) -> Color32 {
    match level {
        LogLevel::Info => accent_color(dark_mode),
        LogLevel::Success => COLOR_SUCCESS,
        LogLevel::Warning => COLOR_WARNING,
        LogLevel::Error => COLOR_ERROR,
    }
}
