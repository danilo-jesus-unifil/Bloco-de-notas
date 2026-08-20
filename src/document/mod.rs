use std::path::{Path, PathBuf};

const MAX_HISTORY_ENTRIES: usize = 128;

#[derive(Clone, Debug)]
struct Snapshot {
    text: String,
}

#[derive(Debug)]
pub struct Document {
    text: String,
    path: Option<PathBuf>,
    dirty: bool,
    utf8_bom: bool,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    editor_baseline: String,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            path: None,
            dirty: false,
            utf8_bom: false,
            undo: Vec::new(),
            redo: Vec::new(),
            editor_baseline: String::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn file_name(&self) -> &str {
        self.path
            .as_deref()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .unwrap_or("Sem título")
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn utf8_bom(&self) -> bool {
        self.utf8_bom
    }

    pub fn sync_editor_change(&mut self) -> bool {
        if self.editor_baseline == self.text {
            return false;
        }

        self.undo.push(Snapshot {
            text: self.editor_baseline.clone(),
        });
        self.trim_history();
        self.editor_baseline = self.text.clone();
        self.dirty = true;
        self.redo.clear();
        true
    }

    pub fn set_loaded_state(&mut self, path: PathBuf, text: String, utf8_bom: bool) {
        self.text = text;
        self.editor_baseline = self.text.clone();
        self.path = Some(path);
        self.dirty = false;
        self.utf8_bom = utf8_bom;
        self.undo.clear();
        self.redo.clear();
    }

    pub fn set_new_state(&mut self) {
        self.text.clear();
        self.editor_baseline.clear();
        self.path = None;
        self.dirty = false;
        self.utf8_bom = false;
        self.undo.clear();
        self.redo.clear();
    }

    pub fn mark_saved(&mut self, path: PathBuf, utf8_bom: bool) {
        self.path = Some(path);
        self.utf8_bom = utf8_bom;
        self.dirty = false;
        self.editor_baseline = self.text.clone();
        self.undo.clear();
        self.redo.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.undo.pop() else {
            return false;
        };

        self.redo.push(Snapshot {
            text: self.text.clone(),
        });
        self.text = snapshot.text;
        self.editor_baseline = self.text.clone();
        self.dirty = true;
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(snapshot) = self.redo.pop() else {
            return false;
        };

        self.undo.push(Snapshot {
            text: self.text.clone(),
        });
        self.trim_history();
        self.text = snapshot.text;
        self.editor_baseline = self.text.clone();
        self.dirty = true;
        true
    }

    fn trim_history(&mut self) {
        if self.undo.len() > MAX_HISTORY_ENTRIES {
            self.undo.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Document;

    #[test]
    fn editing_marks_document_dirty_and_undo_restores_text() {
        let mut document = Document::new();
        document.text_mut().push_str("primeiro texto");
        document.sync_editor_change();
        assert!(document.is_dirty());
        assert!(document.undo());
        assert_eq!(document.text(), "");
        assert!(document.redo());
        assert_eq!(document.text(), "primeiro texto");
    }

    #[test]
    fn saving_clears_history_and_dirty_state() {
        let mut document = Document::new();
        document.text_mut().push_str("texto");
        document.sync_editor_change();
        document.mark_saved("arquivo.txt".into(), false);

        assert!(!document.is_dirty());
        assert!(!document.can_undo());
        assert_eq!(document.file_name(), "arquivo.txt");
    }
}
