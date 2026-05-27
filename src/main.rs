#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod steam_bridge;
mod theme;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Steam Achievement Panel")
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([980.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Steam Achievement Panel",
        native_options,
        Box::new(|creation_context| {
            theme::apply(&creation_context.egui_ctx);
            Ok(Box::<app::AchievementPanelApp>::default())
        }),
    )
}
