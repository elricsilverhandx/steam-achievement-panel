#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod steam_bridge;
mod steam_library;
mod theme;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Steam Achievement Panel")
            .with_inner_size([1220.0, 780.0])
            .with_min_inner_size([1020.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Steam Achievement Panel",
        native_options,
        Box::new(|creation_context| {
            theme::apply(&creation_context.egui_ctx);
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Ok(Box::<app::AchievementPanelApp>::default())
        }),
    )
}
