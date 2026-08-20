#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppCommand {
    New,
    Open,
    Save,
    SaveAs,
    Close,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    Find,
    Replace,
}
