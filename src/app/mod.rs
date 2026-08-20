use std::path::{Path, PathBuf};

use arboard::Clipboard;
use eframe::egui::{
    self,
    text::{CCursor, CCursorRange},
    text_edit::TextEditState,
    Id, ViewportCommand,
};
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use serde::{Deserialize, Serialize};

use crate::commands::AppCommand;
use crate::document::Document;
use crate::editor::SearchState;
use crate::error::AppError;
use crate::file_io;
use crate::theme;

const TEXT_EDIT_ID: &str = "main-document-editor";
const SETTINGS_KEY: &str = "notepad-settings";
const DEFAULT_FONT_SIZE: f32 = 16.0;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct PersistedSettings {
    theme: ThemeMode,
    font_size: f32,
    word_wrap: bool,
    show_status_bar: bool,
    open_in_new_tab: bool,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            font_size: DEFAULT_FONT_SIZE,
            word_wrap: true,
            show_status_bar: true,
            open_in_new_tab: true,
        }
    }
}

pub(crate) struct NoteTab {
    pub(crate) document: Document,
    pub(crate) search: SearchState,
    pub(crate) font_size: f32,
    pub(crate) word_wrap: bool,
}

impl NoteTab {
    fn with_settings(settings: PersistedSettings) -> Self {
        Self {
            document: Document::new(),
            search: SearchState::default(),
            font_size: settings.font_size.clamp(10.0, 32.0),
            word_wrap: settings.word_wrap,
        }
    }
}

pub struct NotepadApp {
    pub(crate) tabs: Vec<NoteTab>,
    pub(crate) active_tab: usize,
    pub(crate) show_search: bool,
    pub(crate) replace_mode: bool,
    pub(crate) show_settings: bool,
    pub(crate) show_status_bar: bool,
    pub(crate) open_in_new_tab: bool,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) status: Option<String>,
}

impl NotepadApp {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let settings = creation_context
            .storage
            .and_then(|storage| eframe::get_value(storage, SETTINGS_KEY))
            .unwrap_or_default();
        theme::apply(&creation_context.egui_ctx);
        Self {
            tabs: vec![NoteTab::with_settings(settings)],
            active_tab: 0,
            show_search: false,
            replace_mode: false,
            show_settings: false,
            show_status_bar: settings.show_status_bar,
            open_in_new_tab: settings.open_in_new_tab,
            theme_mode: settings.theme,
            status: Some("Pronto".to_owned()),
        }
    }

    pub(crate) fn title(&self) -> String {
        let document = self.document();
        let dirty_marker = if document.is_dirty() { " *" } else { "" };
        format!("{}{} — Bloco de notas", document.file_name(), dirty_marker)
    }

    pub(crate) fn document(&self) -> &Document {
        &self.tabs[self.active_tab].document
    }

    pub(crate) fn document_mut(&mut self) -> &mut Document {
        &mut self.tabs[self.active_tab].document
    }

    pub(crate) fn search(&self) -> &SearchState {
        &self.tabs[self.active_tab].search
    }

    pub(crate) fn search_mut(&mut self) -> &mut SearchState {
        &mut self.tabs[self.active_tab].search
    }

    pub(crate) fn font_size(&self) -> f32 {
        self.tabs[self.active_tab].font_size
    }

    pub(crate) fn set_font_size(&mut self, value: f32) {
        self.tabs[self.active_tab].font_size = value.clamp(10.0, 32.0);
    }

    pub(crate) fn word_wrap(&self) -> bool {
        self.tabs[self.active_tab].word_wrap
    }

    pub(crate) fn text_edit_id(&self) -> Id {
        Id::new((TEXT_EDIT_ID, self.active_tab))
    }

    pub(crate) fn tab_label(&self, index: usize) -> String {
        let tab = &self.tabs[index];
        let marker = if tab.document.is_dirty() { " *" } else { "" };
        format!("{}{}", tab.document.file_name(), marker)
    }

    pub(crate) fn dispatch(&mut self, command: AppCommand, ctx: &egui::Context) {
        let result = match command {
            AppCommand::New => self.new_document(),
            AppCommand::Open => self.open_document(),
            AppCommand::Save => self.save_document(),
            AppCommand::SaveAs => self.save_document_as(),
            AppCommand::CloseTab => self.close_tab(),
            AppCommand::Quit => self.close_application(ctx),
            AppCommand::Undo => {
                if self.document_mut().undo() {
                    self.set_cursor_to_end(ctx);
                    self.status = Some("Desfeito".to_owned());
                }
                Ok(())
            }
            AppCommand::Redo => {
                if self.document_mut().redo() {
                    self.set_cursor_to_end(ctx);
                    self.status = Some("Refeito".to_owned());
                }
                Ok(())
            }
            AppCommand::Cut => self.cut_selection(ctx),
            AppCommand::Copy => self.copy_selection(ctx),
            AppCommand::Paste => self.paste_from_clipboard(ctx),
            AppCommand::SelectAll => {
                self.select_all(ctx);
                Ok(())
            }
            AppCommand::Find => {
                self.show_search = true;
                self.replace_mode = false;
                Ok(())
            }
            AppCommand::Replace => {
                self.show_search = true;
                self.replace_mode = true;
                Ok(())
            }
            AppCommand::ZoomIn => {
                self.set_font_size(self.font_size() + 1.0);
                Ok(())
            }
            AppCommand::ZoomOut => {
                self.set_font_size(self.font_size() - 1.0);
                Ok(())
            }
            AppCommand::ResetZoom => {
                self.set_font_size(DEFAULT_FONT_SIZE);
                Ok(())
            }
            AppCommand::ToggleSettings => {
                self.show_settings = !self.show_settings;
                Ok(())
            }
        };

        if let Err(error) = result {
            self.status = Some(error.user_message());
        }
    }

    pub(crate) fn handle_editor_change(&mut self) {
        if self.document_mut().sync_editor_change() {
            self.status = Some("Alterações não salvas".to_owned());
        }
    }

    pub(crate) fn close_search(&mut self) {
        self.show_search = false;
        self.search_mut().clear_result();
    }

    pub(crate) fn find_next(&mut self, ctx: &egui::Context) {
        let start = self.cursor_char_index(ctx);
        let text = self.document().text().to_owned();
        if let Some((start, end)) = self.search_mut().find_next(&text, start) {
            self.set_selection(ctx, start, end);
        }
    }

    pub(crate) fn find_previous(&mut self, ctx: &egui::Context) {
        let query = self.search().query.clone();
        if query.is_empty() {
            self.search_mut().message = Some("Digite um texto para localizar.".to_owned());
            return;
        }
        let text = self.document().text().to_owned();
        let cursor = self.cursor_char_index(ctx).min(text.len());
        let found = text[..cursor].rfind(&query).or_else(|| text.rfind(&query));
        match found {
            Some(start) => {
                self.search_mut().last_match = Some(start);
                self.search_mut().message = Some("Correspondência encontrada.".to_owned());
                self.set_selection(ctx, start, start + query.len());
            }
            None => {
                self.search_mut().last_match = None;
                self.search_mut().message = Some("Nenhuma correspondência encontrada.".to_owned());
            }
        }
    }

    pub(crate) fn replace_one(&mut self, ctx: &egui::Context) {
        let cursor = self.cursor_char_index(ctx);
        let text = self.document().text().to_owned();
        let replacement = self.search().replacement.clone();
        let mut search = SearchState {
            query: self.search().query.clone(),
            replacement,
            ..Default::default()
        };
        if let Some(replaced) = search.replace_first(&text, cursor) {
            *self.document_mut().text_mut() = replaced;
            self.document_mut().sync_editor_change();
            self.set_cursor_to_end(ctx);
            self.status = Some("Uma ocorrência foi substituída.".to_owned());
        }
        self.search_mut().message = search.message;
    }

    pub(crate) fn replace_all(&mut self, ctx: &egui::Context) {
        let text = self.document().text().to_owned();
        let mut search = SearchState {
            query: self.search().query.clone(),
            replacement: self.search().replacement.clone(),
            ..Default::default()
        };
        if let Some(replaced) = search.replace_all(&text) {
            *self.document_mut().text_mut() = replaced;
            self.document_mut().sync_editor_change();
            self.set_cursor_to_end(ctx);
            self.status = search.message.clone();
        }
        self.search_mut().message = search.message;
    }

    pub(crate) fn open_dropped_file(&mut self, path: PathBuf) {
        if !is_supported_text_path(&path) {
            self.status = Some(
                "Solte um arquivo de texto compatível (.txt, .md, .log, .csv, .ini ou .json)."
                    .to_owned(),
            );
            return;
        }
        if !self.open_in_new_tab && !self.confirm_discard("abrir o arquivo arrastado") {
            return;
        }
        if let Err(error) = self.open_path(path) {
            self.status = Some(error.user_message());
        }
    }

    pub(crate) fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
            self.show_search = false;
            self.status = Some(format!("Aba {} de {}", index + 1, self.tabs.len()));
        }
    }

    pub(crate) fn new_tab(&mut self) {
        let settings = PersistedSettings {
            theme: self.theme_mode,
            font_size: self.font_size(),
            word_wrap: self.word_wrap(),
            show_status_bar: self.show_status_bar,
            open_in_new_tab: self.open_in_new_tab,
        };
        self.tabs.push(NoteTab::with_settings(settings));
        self.active_tab = self.tabs.len() - 1;
        self.status = Some("Nova aba".to_owned());
    }

    fn new_document(&mut self) -> Result<(), AppError> {
        if self.open_in_new_tab
            || !self.document().text().is_empty()
            || self.document().path().is_some()
        {
            self.new_tab();
        } else if self.confirm_discard("criar um novo documento") {
            self.document_mut().set_new_state();
            self.status = Some("Novo documento".to_owned());
        }
        Ok(())
    }

    fn open_document(&mut self) -> Result<(), AppError> {
        if !self.confirm_discard("abrir outro documento") {
            return Ok(());
        }

        let Some(path) = FileDialog::new()
            .add_filter(
                "Arquivos de texto",
                &["txt", "md", "log", "csv", "ini", "json"],
            )
            .add_filter("Todos os arquivos", &["*"])
            .pick_file()
        else {
            return Ok(());
        };

        self.open_path(path)
    }

    fn open_path(&mut self, path: PathBuf) -> Result<(), AppError> {
        let loaded = file_io::load(&path)?;
        if self.open_in_new_tab
            && (self.document().is_dirty()
                || !self.document().text().is_empty()
                || self.document().path().is_some())
        {
            self.new_tab();
        }
        self.document_mut().set_loaded_state(
            path.clone(),
            loaded.text,
            loaded.utf8_bom,
            loaded.line_ending,
        );
        self.status = Some(format!("Aberto: {}", path.display()));
        Ok(())
    }

    fn save_document(&mut self) -> Result<(), AppError> {
        match self.document().path().map(PathBuf::from) {
            Some(path) => self.save_to_path(path),
            None => self.save_document_as(),
        }
    }

    fn save_document_as(&mut self) -> Result<(), AppError> {
        let Some(path) = FileDialog::new()
            .add_filter(
                "Arquivo de texto",
                &["txt", "md", "log", "csv", "ini", "json"],
            )
            .set_file_name(if self.document().file_name() == "Sem título" {
                "Sem título.txt"
            } else {
                self.document().file_name()
            })
            .save_file()
        else {
            return Ok(());
        };

        self.save_to_path(path)
    }

    fn save_to_path(&mut self, path: PathBuf) -> Result<(), AppError> {
        let text = self.document().text().to_owned();
        let utf8_bom = self.document().utf8_bom();
        let line_ending = self.document().line_ending();
        file_io::save(&path, &text, utf8_bom, line_ending)?;
        self.document_mut().mark_saved(path.clone(), utf8_bom);
        self.status = Some(format!("Salvo: {}", path.display()));
        Ok(())
    }

    fn close_tab(&mut self) -> Result<(), AppError> {
        if !self.confirm_discard("fechar esta aba") {
            return Ok(());
        }
        if self.tabs.len() == 1 {
            self.document_mut().set_new_state();
            self.status = Some("Aba limpa".to_owned());
        } else {
            self.tabs.remove(self.active_tab);
            self.active_tab = self.active_tab.min(self.tabs.len() - 1);
            self.status = Some("Aba fechada".to_owned());
        }
        Ok(())
    }

    fn close_application(&mut self, ctx: &egui::Context) -> Result<(), AppError> {
        if self.confirm_close_all() {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        Ok(())
    }

    fn confirm_discard(&mut self, action: &str) -> bool {
        if !self.document().is_dirty() {
            return true;
        }

        let result = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Alterações não salvas")
            .set_description(format!(
                "Há alterações não salvas. Deseja salvar antes de {action}?"
            ))
            .set_buttons(MessageButtons::YesNoCancel)
            .show();

        match result {
            MessageDialogResult::Yes => self.save_document().is_ok(),
            MessageDialogResult::No => true,
            MessageDialogResult::Cancel
            | MessageDialogResult::Ok
            | MessageDialogResult::Custom(_) => false,
        }
    }

    fn confirm_close_all(&mut self) -> bool {
        if !self.tabs.iter().any(|tab| tab.document.is_dirty()) {
            return true;
        }
        let result = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Alterações não salvas")
            .set_description("Há abas com alterações não salvas. Deseja salvar antes de sair?")
            .set_buttons(MessageButtons::YesNoCancel)
            .show();
        match result {
            MessageDialogResult::Yes => {
                for index in 0..self.tabs.len() {
                    if self.tabs[index].document.is_dirty() && !self.save_tab(index) {
                        return false;
                    }
                }
                true
            }
            MessageDialogResult::No => true,
            MessageDialogResult::Cancel
            | MessageDialogResult::Ok
            | MessageDialogResult::Custom(_) => false,
        }
    }

    fn save_tab(&mut self, index: usize) -> bool {
        let Some(path) = self.tabs[index].document.path().map(PathBuf::from) else {
            let previous_tab = self.active_tab;
            self.active_tab = index;
            let result = self.save_document().is_ok();
            self.active_tab = previous_tab;
            return result;
        };
        let text = self.tabs[index].document.text().to_owned();
        let bom = self.tabs[index].document.utf8_bom();
        let line_ending = self.tabs[index].document.line_ending();
        if file_io::save(&path, &text, bom, line_ending).is_ok() {
            self.tabs[index].document.mark_saved(path, bom);
            true
        } else {
            false
        }
    }

    pub(crate) fn has_selection(&self, ctx: &egui::Context) -> bool {
        self.selected_char_range(ctx).is_some()
    }

    fn selected_char_range(&self, ctx: &egui::Context) -> Option<(usize, usize)> {
        let state = TextEditState::load(ctx, self.text_edit_id())?;
        let [start, end] = state.cursor.char_range()?.sorted();
        (start.index < end.index).then_some((start.index, end.index))
    }

    fn copy_selection(&mut self, ctx: &egui::Context) -> Result<(), AppError> {
        let Some((start, end)) = self.selected_char_range(ctx) else {
            return Ok(());
        };
        let selected = slice_by_chars(self.document().text(), start, end).to_owned();
        let mut clipboard =
            Clipboard::new().map_err(|error| AppError::Clipboard(error.to_string()))?;
        clipboard
            .set_text(selected)
            .map_err(|error| AppError::Clipboard(error.to_string()))?;
        self.status = Some("Copiado".to_owned());
        Ok(())
    }

    fn cut_selection(&mut self, ctx: &egui::Context) -> Result<(), AppError> {
        let Some((start, end)) = self.selected_char_range(ctx) else {
            return Ok(());
        };
        self.copy_selection(ctx)?;
        let (byte_start, byte_end) = byte_range(self.document().text(), start, end);
        self.document_mut()
            .text_mut()
            .replace_range(byte_start..byte_end, "");
        self.document_mut().sync_editor_change();
        self.set_selection(ctx, start, start);
        self.status = Some("Recortado".to_owned());
        Ok(())
    }

    fn paste_from_clipboard(&mut self, ctx: &egui::Context) -> Result<(), AppError> {
        let mut clipboard =
            Clipboard::new().map_err(|error| AppError::Clipboard(error.to_string()))?;
        let pasted = clipboard
            .get_text()
            .map_err(|error| AppError::Clipboard(error.to_string()))?;
        let (start, end) = self.selected_char_range(ctx).unwrap_or_else(|| {
            let cursor = self.cursor_char_index(ctx);
            (cursor, cursor)
        });
        let (byte_start, byte_end) = byte_range(self.document().text(), start, end);
        self.document_mut()
            .text_mut()
            .replace_range(byte_start..byte_end, &pasted);
        self.document_mut().sync_editor_change();
        let new_cursor = start + pasted.chars().count();
        self.set_selection(ctx, new_cursor, new_cursor);
        self.status = Some("Colado".to_owned());
        Ok(())
    }

    fn select_all(&self, ctx: &egui::Context) {
        let end = self.document().text().chars().count();
        self.set_selection(ctx, 0, end);
    }

    pub(crate) fn cursor_line_column(&self, ctx: &egui::Context) -> (usize, usize) {
        let index = self.cursor_char_index(ctx);
        let before = self.document().text().chars().take(index);
        let mut line = 1;
        let mut column = 1;
        for character in before {
            if character == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }

    fn cursor_char_index(&self, ctx: &egui::Context) -> usize {
        TextEditState::load(ctx, self.text_edit_id())
            .and_then(|state| state.cursor.char_range())
            .map(|range| range.primary.index)
            .unwrap_or(0)
    }

    fn set_cursor_to_end(&self, ctx: &egui::Context) {
        let end = self.document().text().chars().count();
        self.set_selection(ctx, end, end);
    }

    fn set_selection(&self, ctx: &egui::Context, start: usize, end: usize) {
        let mut state = TextEditState::load(ctx, self.text_edit_id()).unwrap_or_default();
        state.cursor.set_char_range(Some(CCursorRange::two(
            CCursor::new(start),
            CCursor::new(end),
        )));
        state.store(ctx, self.text_edit_id());
        ctx.memory_mut(|memory| memory.request_focus(self.text_edit_id()));
    }

    pub(crate) fn accept_viewport_close(&mut self, ctx: &egui::Context) {
        if ctx.input(|input| input.viewport().close_requested()) && !self.confirm_close_all() {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
        }
    }

    pub(crate) fn persist_settings(&self, storage: &mut dyn eframe::Storage) {
        let settings = PersistedSettings {
            theme: self.theme_mode,
            font_size: self.font_size(),
            word_wrap: self.word_wrap(),
            show_status_bar: self.show_status_bar,
            open_in_new_tab: self.open_in_new_tab,
        };
        eframe::set_value(storage, SETTINGS_KEY, &settings);
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.accept_viewport_close(ctx);
        if let Some(path) = ctx.input(|input| {
            input
                .raw
                .dropped_files
                .iter()
                .find_map(|file| file.path.clone())
        }) {
            self.open_dropped_file(path);
        }
        ctx.send_viewport_cmd(ViewportCommand::Title(self.title()));
        crate::ui::render(self, ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.persist_settings(storage);
    }
}

fn is_supported_text_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "txt" | "md" | "log" | "csv" | "ini" | "json"
            )
        })
        .unwrap_or(false)
}

fn slice_by_chars(text: &str, start: usize, end: usize) -> &str {
    let (byte_start, byte_end) = byte_range(text, start, end);
    &text[byte_start..byte_end]
}

fn byte_range(text: &str, start: usize, end: usize) -> (usize, usize) {
    let byte_start = text
        .char_indices()
        .nth(start)
        .map_or(text.len(), |(index, _)| index);
    let byte_end = text
        .char_indices()
        .nth(end)
        .map_or(text.len(), |(index, _)| index);
    (byte_start, byte_end)
}
