use std::fmt::{Display, Formatter};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    Clipboard(String),
    User(String),
}

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Io {
                operation,
                path,
                source,
            } => {
                format!(
                    "Não foi possível {operation} ‘{}’: {source}",
                    path.display()
                )
            }
            Self::Clipboard(message) => {
                format!("Não foi possível acessar a área de transferência: {message}")
            }
            Self::User(message) => message.clone(),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.user_message())
    }
}

impl std::error::Error for AppError {}
