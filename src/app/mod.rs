use std::path::PathBuf;

use arboard::Clipboard;
use eframe::egui::{
    self,
    text::{CCursor, CCursorRange},
    text_edit::TextEditState,
    Id, ViewportCommand,
};
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

use crate::commands::AppCommand;
use crate::document::Document;
use crate::editor::SearchState;
use crate::error::AppError;
use crate::file_io;
use crate::theme;

const TEXT_EDIT_ID: &str = "main-document-editor";

pub struct NotepadApp {
    pub(crate) document: Document,
    pub(crate) search: SearchState,
    pub(crate) show_search: bool,
    pub(crate) replace_mode: bool,
    pub(crate) status: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) word_wrap: bool,
    pub(crate) text_edit_id: Id,
}

impl NotepadApp {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&creation_context.egui_ctx);
        Self {
            document: Document::new(),
            search: SearchState::default(),
            show_search: false,
            replace_mode: false,
            status: Some("Pronto".to_owned()),
            font_size: 16.0,
            word_wrap: true,
            text_edit_id: Id::new(TEXT_EDIT_ID),
        }
    }

    pub(crate) fn title(&self) -> String {
        let dirty_marker = if self.document.is_dirty() { " *" } else { "" };
        format!(
            "{}{} — Bloco de notas",
            self.document.file_name(),
            dirty_marker
        )
    }

    pub(crate) fn dispatch(&mut self, command: AppCommand, ctx: &egui::Context) {
        let result = match command {
            AppCommand::New => self.new_document(),
            AppCommand::Open => self.open_document(),
            AppCommand::Save => self.save_document(),
            AppCommand::SaveAs => self.save_document_as(),
            AppCommand::Close => self.close_document(ctx),
            AppCommand::Undo => {
                if self.document.undo() {
                    self.set_cursor_to_end(ctx);
                    self.status = Some("Desfeito".to_owned());
                }
                Ok(())
            }
            AppCommand::Redo => {
                if self.document.redo() {
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
        };

        if let Err(error) = result {
            self.status = Some(error.user_message());
        }
    }

    pub(crate) fn handle_editor_change(&mut self) {
        if self.document.sync_editor_change() {
            self.status = Some("Alterações não salvas".to_owned());
        }
    }

    pub(crate) fn close_search(&mut self) {
        self.show_search = false;
        self.search.clear_result();
    }

    pub(crate) fn find_next(&mut self, ctx: &egui::Context) {
        let start = self.cursor_char_index(ctx);
        if let Some((start, end)) = self.search.find_next(self.document.text(), start) {
            self.set_selection(ctx, start, end);
        }
    }

    pub(crate) fn replace_one(&mut self, ctx: &egui::Context) {
        let cursor = self.cursor_char_index(ctx);
        if let Some(replaced) = self.search.replace_first(self.document.text(), cursor) {
            *self.document.text_mut() = replaced;
            self.document.sync_editor_change();
            self.set_cursor_to_end(ctx);
            self.status = Some("Uma ocorrência foi substituída.".to_owned());
        }
    }

    pub(crate) fn replace_all(&mut self, ctx: &egui::Context) {
        if let Some(replaced) = self.search.replace_all(self.document.text()) {
            *self.document.text_mut() = replaced;
            self.document.sync_editor_change();
            self.set_cursor_to_end(ctx);
            self.status = self.search.message.clone();
        }
    }

    fn new_document(&mut self) -> Result<(), AppError> {
        if !self.confirm_discard("criar um novo documento") {
            return Ok(());
        }
        self.document.set_new_state();
        self.status = Some("Novo documento".to_owned());
        Ok(())
    }

    fn open_document(&mut self) -> Result<(), AppError> {
        if !self.confirm_discard("abrir outro documento") {
            return Ok(());
        }

        let Some(path) = FileDialog::new()
            .add_filter("Arquivos de texto", &["txt", "text"])
            .add_filter("Todos os arquivos", &["*"])
            .pick_file()
        else {
            return Ok(());
        };

        let loaded = file_io::load(&path)?;
        self.document
            .set_loaded_state(path.clone(), loaded.text, loaded.utf8_bom);
        self.status = Some(format!("Aberto: {}", path.display()));
        Ok(())
    }

    fn save_document(&mut self) -> Result<(), AppError> {
        match self.document.path().map(PathBuf::from) {
            Some(path) => self.save_to_path(path),
            None => self.save_document_as(),
        }
    }

    fn save_document_as(&mut self) -> Result<(), AppError> {
        let Some(path) = FileDialog::new()
            .add_filter("Arquivo de texto", &["txt"])
            .set_file_name(if self.document.file_name() == "Sem título" {
                "Sem título.txt"
            } else {
                self.document.file_name()
            })
            .save_file()
        else {
            return Ok(());
        };

        self.save_to_path(path)
    }

    fn save_to_path(&mut self, path: PathBuf) -> Result<(), AppError> {
        file_io::save(&path, self.document.text(), self.document.utf8_bom())?;
        self.document
            .mark_saved(path.clone(), self.document.utf8_bom());
        self.status = Some(format!("Salvo: {}", path.display()));
        Ok(())
    }

    fn close_document(&mut self, ctx: &egui::Context) -> Result<(), AppError> {
        if self.confirm_discard("fechar o aplicativo") {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        Ok(())
    }

    fn confirm_discard(&mut self, action: &str) -> bool {
        if !self.document.is_dirty() {
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

    pub(crate) fn has_selection(&self, ctx: &egui::Context) -> bool {
        self.selected_char_range(ctx).is_some()
    }

    fn selected_char_range(&self, ctx: &egui::Context) -> Option<(usize, usize)> {
        let state = TextEditState::load(ctx, self.text_edit_id)?;
        let [start, end] = state.cursor.char_range()?.sorted();
        (start.index < end.index).then_some((start.index, end.index))
    }

    fn copy_selection(&mut self, ctx: &egui::Context) -> Result<(), AppError> {
        let Some((start, end)) = self.selected_char_range(ctx) else {
            return Ok(());
        };
        let selected = slice_by_chars(self.document.text(), start, end).to_owned();
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
        let (byte_start, byte_end) = byte_range(self.document.text(), start, end);
        self.document
            .text_mut()
            .replace_range(byte_start..byte_end, "");
        self.document.sync_editor_change();
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
        let (byte_start, byte_end) = byte_range(self.document.text(), start, end);
        self.document
            .text_mut()
            .replace_range(byte_start..byte_end, &pasted);
        self.document.sync_editor_change();
        let new_cursor = start + pasted.chars().count();
        self.set_selection(ctx, new_cursor, new_cursor);
        self.status = Some("Colado".to_owned());
        Ok(())
    }

    fn select_all(&self, ctx: &egui::Context) {
        let end = self.document.text().chars().count();
        self.set_selection(ctx, 0, end);
    }

    fn cursor_char_index(&self, ctx: &egui::Context) -> usize {
        TextEditState::load(ctx, self.text_edit_id)
            .and_then(|state| state.cursor.char_range())
            .map(|range| range.primary.index)
            .unwrap_or(0)
    }

    fn set_cursor_to_end(&self, ctx: &egui::Context) {
        let end = self.document.text().chars().count();
        self.set_selection(ctx, end, end);
    }

    fn set_selection(&self, ctx: &egui::Context, start: usize, end: usize) {
        let mut state = TextEditState::load(ctx, self.text_edit_id).unwrap_or_default();
        state.cursor.set_char_range(Some(CCursorRange::two(
            CCursor::new(start),
            CCursor::new(end),
        )));
        state.store(ctx, self.text_edit_id);
        ctx.memory_mut(|memory| memory.request_focus(self.text_edit_id));
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(ViewportCommand::Title(self.title()));
        crate::ui::render(self, ctx);
    }
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
