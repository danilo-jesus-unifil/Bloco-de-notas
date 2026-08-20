use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::document::LineEnding;
use crate::error::AppError;

const MAX_TEXT_FILE_BYTES: u64 = 128 * 1024 * 1024;

#[derive(Debug)]
pub struct LoadedDocument {
    pub text: String,
    pub utf8_bom: bool,
    pub line_ending: LineEnding,
}

pub fn load(path: &Path) -> Result<LoadedDocument, AppError> {
    let metadata = fs::metadata(path).map_err(|source| AppError::Io {
        operation: "inspecionar o arquivo",
        path: path.to_path_buf(),
        source,
    })?;
    if metadata.len() > MAX_TEXT_FILE_BYTES {
        return Err(AppError::User(format!(
            "O arquivo ‘{}’ excede o limite seguro de 128 MB para esta versão.",
            path.display()
        )));
    }

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

#[cfg(not(windows))]
fn replace_file(temporary_path: &Path, target_path: &Path) -> io::Result<()> {
    // On Unix-like targets, rename within one directory replaces atomically.
    fs::rename(temporary_path, target_path)
}

#[cfg(windows)]
fn replace_file(temporary_path: &Path, target_path: &Path) -> io::Result<()> {
    if !target_path.exists() {
        return fs::rename(temporary_path, target_path);
    }
    replace_existing_windows_file(temporary_path, target_path)
}

#[cfg(windows)]
fn replace_existing_windows_file(temporary_path: &Path, target_path: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null;
    use windows_sys::Win32::Storage::FileSystem::ReplaceFileW;

    let target_wide: Vec<u16> = target_path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
    let temporary_wide: Vec<u16> = temporary_path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
    // SAFETY: both UTF-16 buffers are NUL-terminated, live for the call, and the API does not retain them.
    let replaced = unsafe {
        ReplaceFileW(
            target_wide.as_ptr(),
            temporary_wide.as_ptr(),
            null(),
            0,
            null(),
            null(),
        )
    };
    if replaced == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
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

    #[test]
    fn rejects_files_larger_than_the_memory_guard() {
        let path = std::env::temp_dir().join(format!("bloco-large-{}", std::process::id()));
        let file = std::fs::File::create(&path).expect("test file should be created");
        file.set_len(super::MAX_TEXT_FILE_BYTES + 1)
            .expect("sparse test file should be resized");
        let result = super::load(&path);
        assert!(result.is_err());
        let _ = std::fs::remove_file(path);
    }
}
