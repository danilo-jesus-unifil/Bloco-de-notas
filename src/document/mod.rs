use std::path::{Path, PathBuf};

const MAX_HISTORY_ENTRIES: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
    Cr,
}

impl LineEnding {
    pub fn label(self) -> &'static str {
        match self {
            Self::Lf => "Unix (LF)",
            Self::CrLf => "Windows (CRLF)",
            Self::Cr => "Classic (CR)",
        }
    }
}

#[derive(Clone, Debug)]
struct Snapshot {
    text: String,
    revision: u64,
}

#[derive(Debug)]
pub struct Document {
    text: String,
    path: Option<PathBuf>,
    dirty: bool,
    utf8_bom: bool,
    line_ending: LineEnding,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    editor_baseline: String,
    revision: u64,
    saved_revision: u64,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    pub fn new() -> Self {
        let line_ending = if cfg!(windows) {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        Self {
            text: String::new(),
            path: None,
            dirty: false,
            utf8_bom: false,
            line_ending,
            undo: Vec::new(),
            redo: Vec::new(),
            editor_baseline: String::new(),
            revision: 0,
            saved_revision: 0,
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

    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    pub fn sync_editor_change(&mut self) -> bool {
        if self.editor_baseline == self.text {
            return false;
        }

        self.undo.push(Snapshot {
            text: self.editor_baseline.clone(),
            revision: self.revision,
        });
        self.trim_history();
        self.editor_baseline = self.text.clone();
        self.revision = self.revision.wrapping_add(1);
        self.dirty = self.revision != self.saved_revision;
        self.redo.clear();
        true
    }

    pub fn set_loaded_state(
        &mut self,
        path: PathBuf,
        text: String,
        utf8_bom: bool,
        line_ending: LineEnding,
    ) {
        self.text = text;
        self.editor_baseline = self.text.clone();
        self.revision = 0;
        self.saved_revision = 0;
        self.path = Some(path);
        self.dirty = false;
        self.utf8_bom = utf8_bom;
        self.line_ending = line_ending;
        self.undo.clear();
        self.redo.clear();
    }

    pub fn set_new_state(&mut self) {
        let line_ending = if cfg!(windows) {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        self.text.clear();
        self.editor_baseline.clear();
        self.revision = 0;
        self.saved_revision = 0;
        self.path = None;
        self.dirty = false;
        self.utf8_bom = false;
        self.line_ending = line_ending;
        self.undo.clear();
        self.redo.clear();
    }

    pub fn mark_saved(&mut self, path: PathBuf, utf8_bom: bool) {
        self.path = Some(path);
        self.utf8_bom = utf8_bom;
        self.dirty = false;
        self.editor_baseline = self.text.clone();
        self.saved_revision = self.revision;
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
            revision: self.revision,
        });
        self.text = snapshot.text;
        self.revision = snapshot.revision;
        self.editor_baseline = self.text.clone();
        self.dirty = self.revision != self.saved_revision;
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(snapshot) = self.redo.pop() else {
            return false;
        };

        self.undo.push(Snapshot {
            text: self.text.clone(),
            revision: self.revision,
        });
        self.trim_history();
        self.text = snapshot.text;
        self.revision = snapshot.revision;
        self.editor_baseline = self.text.clone();
        self.dirty = self.revision != self.saved_revision;
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
    use super::{Document, LineEnding};

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

    #[test]
    fn undoing_to_saved_content_clears_dirty_state() {
        let mut document = Document::new();
        document.text_mut().push_str("texto salvo");
        document.sync_editor_change();
        document.mark_saved("arquivo.txt".into(), false);

        document.text_mut().push_str(" alterado");
        document.sync_editor_change();
        assert!(document.is_dirty());
        assert!(document.undo());
        assert_eq!(document.text(), "texto salvo");
        assert!(!document.is_dirty());
        assert!(document.redo());
        assert!(document.is_dirty());
    }

    #[test]
    fn line_ending_labels_are_stable() {
        assert_eq!(LineEnding::CrLf.label(), "Windows (CRLF)");
        assert_eq!(LineEnding::Lf.label(), "Unix (LF)");
    }
}
