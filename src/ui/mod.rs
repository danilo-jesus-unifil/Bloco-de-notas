use eframe::egui::{self, FontId, Key, Modifiers, TextEdit};

use crate::app::{NotepadApp, ThemeMode};
use crate::commands::AppCommand;
use crate::theme;

pub fn render(app: &mut NotepadApp, ctx: &egui::Context) {
    let mut command = keyboard_command(ctx, app.text_edit_id());

    egui::TopBottomPanel::top("app-header")
        .frame(
            egui::Frame::none()
                .fill(theme::SURFACE)
                .inner_margin(egui::Margin::symmetric(14.0, 8.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(theme::ACCENT, "▣");
                ui.strong("Bloco de notas");
                ui.separator();
                for index in 0..app.tabs.len() {
                    if ui
                        .selectable_label(index == app.active_tab, app.tab_label(index))
                        .clicked()
                    {
                        app.select_tab(index);
                    }
                }
                if ui
                    .button(egui::RichText::new("+").size(18.0))
                    .on_hover_text("Nova aba")
                    .clicked()
                {
                    command = Some(AppCommand::New);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Configurações").clicked() {
                        command = Some(AppCommand::ToggleSettings);
                    }
                });
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.menu_button("Arquivo", |ui| {
                    menu_item(ui, "Novo", "Ctrl+N", AppCommand::New, &mut command);
                    ui.add_enabled(false, egui::Button::new("Nova janela"));
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
                    menu_item(
                        ui,
                        "Fechar aba",
                        "Ctrl+W",
                        AppCommand::CloseTab,
                        &mut command,
                    );
                    menu_item(ui, "Sair", "Alt+F4", AppCommand::Quit, &mut command);
                });

                ui.menu_button("Editar", |ui| {
                    let can_undo = app.document().can_undo();
                    let can_redo = app.document().can_redo();
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

                ui.menu_button("Exibir", |ui| {
                    menu_item(
                        ui,
                        "Aumentar zoom",
                        "Ctrl++",
                        AppCommand::ZoomIn,
                        &mut command,
                    );
                    menu_item(
                        ui,
                        "Diminuir zoom",
                        "Ctrl+-",
                        AppCommand::ZoomOut,
                        &mut command,
                    );
                    menu_item(
                        ui,
                        "Restaurar zoom",
                        "Ctrl+0",
                        AppCommand::ResetZoom,
                        &mut command,
                    );
                    ui.separator();
                    if ui
                        .checkbox(
                            &mut app.tabs[app.active_tab].word_wrap,
                            "Quebra automática de linha",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .checkbox(&mut app.show_status_bar, "Barra de status")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui.button("Configurações…").clicked() {
                        command = Some(AppCommand::ToggleSettings);
                        ui.close_menu();
                    }
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
            });
        });

    if app.show_search {
        render_search_panel(app, ctx);
    }
    if app.show_settings {
        render_settings_panel(app, ctx);
    }

    if app.show_status_bar {
        egui::TopBottomPanel::bottom("status-bar")
            .frame(
                egui::Frame::none()
                    .fill(theme::SURFACE)
                    .inner_margin(egui::Margin::symmetric(14.0, 5.0)),
            )
            .show(ctx, |ui| {
                let (line, column) = app.cursor_line_column(ctx);
                ui.horizontal(|ui| {
                    ui.colored_label(theme::TEXT_SECONDARY, format!("Ln {line}, Col {column}"));
                    ui.separator();
                    ui.colored_label(
                        theme::TEXT_SECONDARY,
                        format!("{} caracteres", app.document().text().chars().count()),
                    );
                    ui.separator();
                    ui.colored_label(
                        theme::TEXT_SECONDARY,
                        format!("{}%", zoom_percent(app.font_size())),
                    );
                    ui.separator();
                    ui.colored_label(theme::TEXT_SECONDARY, app.document().line_ending().label());
                    ui.separator();
                    ui.colored_label(theme::TEXT_SECONDARY, "UTF-8");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if app.document().is_dirty() {
                            ui.colored_label(theme::ACCENT, "Não salvo");
                        } else {
                            ui.colored_label(theme::TEXT_SECONDARY, "Salvo");
                        }
                    });
                });
            });
    }

    egui::CentralPanel::default()
        .frame(
            egui::Frame::none()
                .fill(theme::BACKGROUND)
                .inner_margin(egui::Margin::same(14.0)),
        )
        .show(ctx, |ui| {
            let available_size = ui.available_size();
            let desired_width = if app.word_wrap() {
                available_size.x
            } else {
                1_000_000.0
            };
            let editor_id = app.text_edit_id();
            let font_size = app.font_size();
            let response = ui.add_sized(
                available_size,
                TextEdit::multiline(app.document_mut().text_mut())
                    .id(editor_id)
                    .font(FontId::monospace(font_size))
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
    let mut action = None;
    egui::TopBottomPanel::top("search-panel")
        .frame(
            egui::Frame::none()
                .fill(theme::SURFACE)
                .inner_margin(egui::Margin::symmetric(14.0, 8.0)),
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
                    TextEdit::singleline(&mut app.search_mut().query)
                        .hint_text("Texto para localizar"),
                );
                if search_response.lost_focus() && ui.input(|input| input.key_pressed(Key::Enter)) {
                    action = Some(SearchAction::Next);
                }
                if ui.button("Próximo").clicked() {
                    action = Some(SearchAction::Next);
                }
                if ui.button("Anterior").clicked() {
                    action = Some(SearchAction::Previous);
                }

                if app.replace_mode {
                    ui.add_sized(
                        [230.0, 28.0],
                        TextEdit::singleline(&mut app.search_mut().replacement)
                            .hint_text("Substituir por"),
                    );
                    if ui.button("Substituir").clicked() {
                        action = Some(SearchAction::ReplaceOne);
                    }
                    if ui.button("Substituir tudo").clicked() {
                        action = Some(SearchAction::ReplaceAll);
                    }
                }

                if ui.button("Fechar").clicked() {
                    action = Some(SearchAction::Close);
                }
                if let Some(message) = &app.search().message {
                    ui.colored_label(theme::TEXT_SECONDARY, message);
                }
            });
        });

    match action {
        Some(SearchAction::Next) => app.find_next(ctx),
        Some(SearchAction::Previous) => app.find_previous(ctx),
        Some(SearchAction::ReplaceOne) => app.replace_one(ctx),
        Some(SearchAction::ReplaceAll) => app.replace_all(ctx),
        Some(SearchAction::Close) => app.close_search(),
        None => {}
    }
}

fn render_settings_panel(app: &mut NotepadApp, ctx: &egui::Context) {
    let mut close = false;
    egui::Window::new("Configurações")
        .collapsible(false)
        .resizable(false)
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.heading("Preferências");
            ui.separator();
            ui.label("Tema");
            ui.radio_value(&mut app.theme_mode, ThemeMode::Dark, "Escuro (padrão)");
            ui.add_enabled(false, egui::RadioButton::new(false, "Claro (futuro)"));
            ui.add_enabled(
                false,
                egui::RadioButton::new(false, "Seguir o sistema (futuro)"),
            );
            ui.separator();
            ui.label("Editor");
            ui.add(
                egui::Slider::new(&mut app.tabs[app.active_tab].font_size, 10.0..=32.0)
                    .text("Tamanho da fonte"),
            );
            ui.checkbox(
                &mut app.tabs[app.active_tab].word_wrap,
                "Quebra automática de linha",
            );
            ui.checkbox(&mut app.show_status_bar, "Mostrar barra de status");
            ui.separator();
            ui.checkbox(&mut app.open_in_new_tab, "Abrir arquivos em nova aba");
            ui.label("A fonte do editor permanece monoespaçada nesta versão.");
            if ui.button("Concluído").clicked() {
                close = true;
            }
        });
    if close {
        app.show_settings = false;
    }
}

#[derive(Clone, Copy)]
enum SearchAction {
    Next,
    Previous,
    ReplaceOne,
    ReplaceAll,
    Close,
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

fn keyboard_command(ctx: &egui::Context, editor_id: egui::Id) -> Option<AppCommand> {
    let document_editing_focused = ctx.memory(|memory| {
        memory
            .focused()
            .is_none_or(|focused_id| focused_id == editor_id)
    });

    ctx.input_mut(|input| {
        if input.consume_key(Modifiers::COMMAND, Key::N) {
            Some(AppCommand::New)
        } else if input.consume_key(Modifiers::COMMAND, Key::O) {
            Some(AppCommand::Open)
        } else if input.modifiers.shift && input.consume_key(Modifiers::COMMAND, Key::S) {
            Some(AppCommand::SaveAs)
        } else if input.consume_key(Modifiers::COMMAND, Key::S) {
            Some(AppCommand::Save)
        } else if input.consume_key(Modifiers::COMMAND, Key::W) {
            Some(AppCommand::CloseTab)
        } else if document_editing_focused && input.consume_key(Modifiers::COMMAND, Key::Z) {
            Some(AppCommand::Undo)
        } else if document_editing_focused && input.consume_key(Modifiers::COMMAND, Key::Y) {
            Some(AppCommand::Redo)
        } else if document_editing_focused && input.consume_key(Modifiers::COMMAND, Key::X) {
            Some(AppCommand::Cut)
        } else if document_editing_focused && input.consume_key(Modifiers::COMMAND, Key::C) {
            Some(AppCommand::Copy)
        } else if document_editing_focused && input.consume_key(Modifiers::COMMAND, Key::V) {
            Some(AppCommand::Paste)
        } else if document_editing_focused && input.consume_key(Modifiers::COMMAND, Key::A) {
            Some(AppCommand::SelectAll)
        } else if input.consume_key(Modifiers::COMMAND, Key::F) {
            Some(AppCommand::Find)
        } else if input.consume_key(Modifiers::COMMAND, Key::H) {
            Some(AppCommand::Replace)
        } else if input.consume_key(Modifiers::COMMAND, Key::Plus) {
            Some(AppCommand::ZoomIn)
        } else if input.consume_key(Modifiers::COMMAND, Key::Minus) {
            Some(AppCommand::ZoomOut)
        } else if input.consume_key(Modifiers::COMMAND, Key::Num0) {
            Some(AppCommand::ResetZoom)
        } else {
            None
        }
    })
}

fn zoom_percent(font_size: f32) -> u32 {
    ((font_size / 16.0) * 100.0).round() as u32
}
