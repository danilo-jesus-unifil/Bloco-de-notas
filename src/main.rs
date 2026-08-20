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
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([520.0, 320.0])
            .with_title("Bloco de notas"),
        ..Default::default()
    };

    eframe::run_native(
        "Bloco de notas",
        options,
        Box::new(|creation_context| Ok(Box::new(app::NotepadApp::new(creation_context)))),
    )
}
