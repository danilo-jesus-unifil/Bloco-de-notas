use eframe::egui::{self, FontId, Key, Modifiers, TextEdit};

use crate::app::NotepadApp;
use crate::commands::AppCommand;
use crate::theme;

pub fn render(app: &mut NotepadApp, ctx: &egui::Context) {
    let mut command = keyboard_command(ctx);

    egui::TopBottomPanel::top("menu-bar")
        .frame(
            egui::Frame::none()
                .fill(theme::SURFACE)
                .inner_margin(egui::Margin::symmetric(12.0, 6.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Arquivo", |ui| {
                    menu_item(ui, "Novo", "Ctrl+N", AppCommand::New, &mut command);
                    menu_item(ui, "Abrir…", "Ctrl+O", AppCommand::Open, &mut command);
                    ui.separator();
                    menu_item(ui, "Salvar", "Ctrl+S", AppCommand::Save, &mut command);
                    menu_item(
                        ui,
                        "Salvar como…",
                        "Ctrl+Shift+S",
                        AppCommand::SaveAs,
                        &mut command,
                    );
                    ui.separator();
                    menu_item(ui, "Fechar", "Alt+F4", AppCommand::Close, &mut command);
                });

                ui.menu_button("Editar", |ui| {
                    let can_undo = app.document.can_undo();
                    let can_redo = app.document.can_redo();
                    let has_selection = app.has_selection(ctx);
                    menu_item_enabled(
                        ui,
                        "Desfazer",
                        "Ctrl+Z",
                        AppCommand::Undo,
                        can_undo,
                        &mut command,
                    );
                    menu_item_enabled(
                        ui,
                        "Refazer",
                        "Ctrl+Y",
                        AppCommand::Redo,
                        can_redo,
                        &mut command,
                    );
                    ui.separator();
                    menu_item_enabled(
                        ui,
                        "Recortar",
                        "Ctrl+X",
                        AppCommand::Cut,
                        has_selection,
                        &mut command,
                    );
                    menu_item_enabled(
                        ui,
                        "Copiar",
                        "Ctrl+C",
                        AppCommand::Copy,
                        has_selection,
                        &mut command,
                    );
                    menu_item(ui, "Colar", "Ctrl+V", AppCommand::Paste, &mut command);
                    menu_item(
                        ui,
                        "Selecionar tudo",
                        "Ctrl+A",
                        AppCommand::SelectAll,
                        &mut command,
                    );
                });

                ui.menu_button("Localizar", |ui| {
                    menu_item(ui, "Localizar", "Ctrl+F", AppCommand::Find, &mut command);
                    menu_item(
                        ui,
                        "Substituir",
                        "Ctrl+H",
                        AppCommand::Replace,
                        &mut command,
                    );
                });

                ui.menu_button("Exibir", |ui| {
                    if ui
                        .checkbox(&mut app.word_wrap, "Quebra automática de linha")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Tamanho da fonte");
                        if ui.small_button("−").clicked() {
                            app.font_size = (app.font_size - 1.0).max(10.0);
                        }
                        ui.label(format!("{}", app.font_size as u32));
                        if ui.small_button("+").clicked() {
                            app.font_size = (app.font_size + 1.0).min(32.0);
                        }
                    });
                });
            });
        });

    if app.show_search {
        render_search_panel(app, ctx);
    }

    egui::TopBottomPanel::bottom("status-bar")
        .frame(
            egui::Frame::none()
                .fill(theme::SURFACE)
                .inner_margin(egui::Margin::symmetric(12.0, 5.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status = app.status.as_deref().unwrap_or("Pronto");
                ui.colored_label(theme::TEXT_SECONDARY, status);
                ui.separator();
                ui.colored_label(
                    theme::TEXT_SECONDARY,
                    format!("{} caracteres", app.document.text().chars().count()),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if app.document.is_dirty() {
                        ui.colored_label(theme::ACCENT, "Não salvo");
                    } else {
                        ui.colored_label(theme::TEXT_SECONDARY, "Salvo");
                    }
                });
            });
        });

    egui::CentralPanel::default()
        .frame(
            egui::Frame::none()
                .fill(theme::BACKGROUND)
                .inner_margin(egui::Margin::same(12.0)),
        )
        .show(ctx, |ui| {
            let available_size = ui.available_size();
            let desired_width = if app.word_wrap {
                available_size.x
            } else {
                1_000_000.0
            };
            let response = ui.add_sized(
                available_size,
                TextEdit::multiline(app.document.text_mut())
                    .id(app.text_edit_id)
                    .font(FontId::monospace(app.font_size))
                    .text_color(theme::TEXT_PRIMARY)
                    .desired_width(desired_width)
                    .desired_rows(20)
                    .frame(false)
                    .margin(egui::Margin::same(8.0))
                    .lock_focus(true)
                    .hint_text("Comece a digitar…"),
            );
            if response.changed() {
                app.handle_editor_change();
            }
        });

    if let Some(command) = command {
        app.dispatch(command, ctx);
    }
}

fn render_search_panel(app: &mut NotepadApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("search-panel")
        .frame(
            egui::Frame::none()
                .fill(theme::SURFACE)
                .inner_margin(egui::Margin::symmetric(12.0, 8.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(
                    theme::TEXT_SECONDARY,
                    if app.replace_mode {
                        "Substituir"
                    } else {
                        "Localizar"
                    },
                );
                let search_response = ui.add_sized(
                    [230.0, 28.0],
                    TextEdit::singleline(&mut app.search.query).hint_text("Texto para localizar"),
                );
                if search_response.lost_focus() && ui.input(|input| input.key_pressed(Key::Enter)) {
                    app.find_next(ctx);
                }
                if ui.button("Localizar próximo").clicked() {
                    app.find_next(ctx);
                }

                if app.replace_mode {
                    ui.add_sized(
                        [230.0, 28.0],
                        TextEdit::singleline(&mut app.search.replacement)
                            .hint_text("Substituir por"),
                    );
                    if ui.button("Substituir").clicked() {
                        app.replace_one(ctx);
                    }
                    if ui.button("Substituir tudo").clicked() {
                        app.replace_all(ctx);
                    }
                }

                if ui.button("Fechar").clicked() {
                    app.close_search();
                }
                if let Some(message) = &app.search.message {
                    ui.colored_label(theme::TEXT_SECONDARY, message);
                }
            });
        });
}

fn menu_item(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    app_command: AppCommand,
    command: &mut Option<AppCommand>,
) {
    menu_item_enabled(ui, label, shortcut, app_command, true, command);
}

fn menu_item_enabled(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    app_command: AppCommand,
    enabled: bool,
    command: &mut Option<AppCommand>,
) {
    let response = ui.add_enabled(
        enabled,
        egui::Button::new(format!("{label:<20} {shortcut}")),
    );
    if response.clicked() {
        *command = Some(app_command);
        ui.close_menu();
    }
}

fn keyboard_command(ctx: &egui::Context) -> Option<AppCommand> {
    ctx.input_mut(|input| {
        if input.consume_key(Modifiers::COMMAND, Key::N) {
            Some(AppCommand::New)
        } else if input.consume_key(Modifiers::COMMAND, Key::O) {
            Some(AppCommand::Open)
        } else if input.modifiers.shift && input.consume_key(Modifiers::COMMAND, Key::S) {
            Some(AppCommand::SaveAs)
        } else if input.consume_key(Modifiers::COMMAND, Key::S) {
            Some(AppCommand::Save)
        } else if input.consume_key(Modifiers::COMMAND, Key::Z) {
            Some(AppCommand::Undo)
        } else if input.consume_key(Modifiers::COMMAND, Key::Y) {
            Some(AppCommand::Redo)
        } else if input.consume_key(Modifiers::COMMAND, Key::X) {
            Some(AppCommand::Cut)
        } else if input.consume_key(Modifiers::COMMAND, Key::C) {
            Some(AppCommand::Copy)
        } else if input.consume_key(Modifiers::COMMAND, Key::V) {
            Some(AppCommand::Paste)
        } else if input.consume_key(Modifiers::COMMAND, Key::A) {
            Some(AppCommand::SelectAll)
        } else if input.consume_key(Modifiers::COMMAND, Key::F) {
            Some(AppCommand::Find)
        } else if input.consume_key(Modifiers::COMMAND, Key::H) {
            Some(AppCommand::Replace)
        } else {
            None
        }
    })
}
