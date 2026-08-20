use eframe::egui::{self, Color32, Rounding, Stroke, Visuals};

pub const BACKGROUND: Color32 = Color32::from_rgb(14, 15, 17);
pub const SURFACE: Color32 = Color32::from_rgb(24, 26, 30);
pub const SURFACE_HOVER: Color32 = Color32::from_rgb(34, 37, 42);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(235, 237, 240);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(157, 163, 173);
pub const BORDER: Color32 = Color32::from_rgb(55, 59, 67);
pub const SELECTION: Color32 = Color32::from_rgb(55, 96, 145);
pub const ACCENT: Color32 = Color32::from_rgb(91, 156, 255);

pub fn apply(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(TEXT_PRIMARY);
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = SURFACE;
    visuals.faint_bg_color = SURFACE;
    visuals.extreme_bg_color = Color32::from_rgb(9, 10, 12);
    visuals.code_bg_color = SURFACE;
    visuals.selection.bg_fill = SELECTION;
    visuals.selection.stroke = Stroke::new(1.0_f32, ACCENT);
    visuals.widgets.noninteractive.bg_fill = SURFACE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_SECONDARY);
    visuals.widgets.inactive.bg_fill = SURFACE;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT_SECONDARY);
    visuals.widgets.hovered.bg_fill = SURFACE_HOVER;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.open.bg_fill = SURFACE_HOVER;
    visuals.widgets.open.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.window_rounding = Rounding::same(8.0);
    visuals.menu_rounding = Rounding::same(6.0);
    visuals.window_stroke = Stroke::new(1.0_f32, BORDER);
    visuals.popup_shadow = egui::epaint::Shadow::NONE;
    ctx.set_visuals(visuals);
}
