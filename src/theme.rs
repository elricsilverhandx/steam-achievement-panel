use eframe::egui::{self, Color32, FontFamily, FontId, RichText, TextStyle, Vec2};

pub const BG: Color32 = Color32::from_rgb(10, 14, 24);
pub const PANEL: Color32 = Color32::from_rgb(18, 24, 38);
pub const PANEL_SOFT: Color32 = Color32::from_rgb(24, 32, 50);
pub const CARD: Color32 = Color32::from_rgb(26, 35, 56);
pub const CARD_HOVER: Color32 = Color32::from_rgb(32, 44, 70);
pub const ACCENT: Color32 = Color32::from_rgb(112, 139, 255);
pub const ACCENT_2: Color32 = Color32::from_rgb(0, 220, 190);
pub const DANGER: Color32 = Color32::from_rgb(255, 95, 110);
pub const WARN: Color32 = Color32::from_rgb(255, 190, 92);
pub const TEXT: Color32 = Color32::from_rgb(235, 240, 255);
pub const MUTED: Color32 = Color32::from_rgb(150, 164, 190);

pub fn apply(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(14.0, 9.0);
    style.spacing.window_margin = egui::Margin::same(16);

    style.visuals.dark_mode = true;
    style.visuals.window_fill = BG;
    style.visuals.panel_fill = BG;
    style.visuals.extreme_bg_color = Color32::from_rgb(7, 10, 18);
    style.visuals.faint_bg_color = PANEL;
    style.visuals.code_bg_color = PANEL_SOFT;
    style.visuals.hyperlink_color = ACCENT_2;
    style.visuals.selection.bg_fill = ACCENT.linear_multiply(0.45);

    style.visuals.widgets.noninteractive.bg_fill = PANEL;
    style.visuals.widgets.inactive.bg_fill = PANEL_SOFT;
    style.visuals.widgets.inactive.fg_stroke.color = TEXT;
    style.visuals.widgets.hovered.bg_fill = CARD_HOVER;
    style.visuals.widgets.hovered.fg_stroke.color = TEXT;
    style.visuals.widgets.active.bg_fill = ACCENT;
    style.visuals.widgets.active.fg_stroke.color = Color32::WHITE;
    style.visuals.widgets.open.bg_fill = CARD;

    style.visuals.window_corner_radius = egui::CornerRadius::same(18);
    style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);
    style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
    style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);

    style.text_styles = [
        (TextStyle::Heading, FontId::new(28.0, FontFamily::Proportional)),
        (TextStyle::Name("Title".into()), FontId::new(22.0, FontFamily::Proportional)),
        (TextStyle::Name("CardTitle".into()), FontId::new(17.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.5, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
    ]
    .into();

    ctx.set_style(style);
}

pub fn muted(text: impl ToString) -> RichText {
    RichText::new(text.to_string()).color(MUTED)
}

pub fn accent(text: impl ToString) -> RichText {
    RichText::new(text.to_string()).color(ACCENT_2).strong()
}

pub fn danger(text: impl ToString) -> RichText {
    RichText::new(text.to_string()).color(DANGER).strong()
}

pub fn warning(text: impl ToString) -> RichText {
    RichText::new(text.to_string()).color(WARN).strong()
}
