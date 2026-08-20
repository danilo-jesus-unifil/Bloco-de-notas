#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod commands;
mod document;
mod editor;
mod error;
mod file_io;
mod theme;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 750.0])
            .with_min_inner_size([640.0, 400.0])
            .with_drag_and_drop(true)
            .with_app_id("bloco-de-notas")
            .with_title("Bloco de notas"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Bloco de notas",
        options,
        Box::new(|creation_context| Ok(Box::new(app::NotepadApp::new(creation_context)))),
    )
}
