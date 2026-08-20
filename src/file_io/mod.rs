use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::document::LineEnding;
use crate::error::AppError;

#[derive(Debug)]
pub struct LoadedDocument {
    pub text: String,
    pub utf8_bom: bool,
    pub line_ending: LineEnding,
}

pub fn load(path: &Path) -> Result<LoadedDocument, AppError> {
    let bytes = fs::read(path).map_err(|source| AppError::Io {
        operation: "abrir o arquivo",
        path: path.to_path_buf(),
        source,
    })?;

    let (utf8_bom, content) = match bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        Some(content) => (true, content),
        None => (false, bytes.as_slice()),
    };
    let raw_text = String::from_utf8(content.to_vec()).map_err(|_| {
        AppError::User(format!(
            "O arquivo ‘{}’ não está em UTF-8 válido. O conteúdo original foi preservado.",
            path.display()
        ))
    })?;
    let line_ending = detect_line_ending(&raw_text);

    Ok(LoadedDocument {
        text: normalize_line_endings(&raw_text),
        utf8_bom,
        line_ending,
    })
}

pub fn save(
    path: &Path,
    text: &str,
    utf8_bom: bool,
    line_ending: LineEnding,
) -> Result<(), AppError> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let temporary_path = temporary_path(parent, path);
    let result = write_temporary_file(&temporary_path, text, utf8_bom, line_ending)
        .and_then(|()| replace_file(&temporary_path, path));

    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }

    result.map_err(|source| AppError::Io {
        operation: "salvar o arquivo",
        path: path.to_path_buf(),
        source,
    })
}

fn write_temporary_file(
    path: &Path,
    text: &str,
    utf8_bom: bool,
    line_ending: LineEnding,
) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    if utf8_bom {
        file.write_all(&[0xEF, 0xBB, 0xBF])?;
    }
    file.write_all(serialize_line_endings(text, line_ending).as_bytes())?;
    file.flush()?;
    file.sync_all()?;
    drop(file);
    Ok(())
}

fn replace_file(temporary_path: &Path, target_path: &Path) -> io::Result<()> {
    match fs::rename(temporary_path, target_path) {
        Ok(()) => Ok(()),
        Err(rename_error) if target_path.exists() => {
            fs::remove_file(target_path)?;
            fs::rename(temporary_path, target_path).map_err(|_| rename_error)
        }
        Err(error) => Err(error),
    }
}

fn temporary_path(parent: &Path, target: &Path) -> PathBuf {
    let file_name = target
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("document");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    parent.join(format!(
        ".{file_name}.bloco-tmp-{}-{timestamp}",
        std::process::id()
    ))
}

fn detect_line_ending(text: &str) -> LineEnding {
    if text.contains("\r\n") {
        LineEnding::CrLf
    } else if text.contains('\r') {
        LineEnding::Cr
    } else {
        LineEnding::Lf
    }
}

fn normalize_line_endings(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            normalized.push('\n');
        } else {
            normalized.push(character);
        }
    }

    normalized
}

fn serialize_line_endings(text: &str, line_ending: LineEnding) -> String {
    match line_ending {
        LineEnding::Lf => text.to_owned(),
        LineEnding::CrLf => text.replace('\n', "\r\n"),
        LineEnding::Cr => text.replace('\n', "\r"),
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_line_ending, normalize_line_endings, serialize_line_endings};
    use crate::document::LineEnding;

    #[test]
    fn normalizes_common_line_endings_to_lf() {
        assert_eq!(normalize_line_endings("a\r\nb\rc\nd"), "a\nb\nc\nd");
    }

    #[test]
    fn detects_and_serializes_line_endings() {
        assert_eq!(detect_line_ending("a\r\nb"), LineEnding::CrLf);
        assert_eq!(serialize_line_endings("a\nb", LineEnding::CrLf), "a\r\nb");
        assert_eq!(serialize_line_endings("a\nb", LineEnding::Cr), "a\rb");
    }
}
