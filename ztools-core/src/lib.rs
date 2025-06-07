pub mod scripts;
pub mod zipper;
pub use scripts::run_script;

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ZToolsError {
    Io(io::Error),
    CompressionError(String),
    InvalidInput(String),
    PathError(String),
    SevenZipError(String),
    GzipError(String),
    UntarError(String),
    SpawnError(String),
    PermissionError(String),
}

impl fmt::Display for ZToolsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZToolsError::Io(e) => write!(f, "I/O error: {}", e),
            ZToolsError::CompressionError(e) => write!(f, "Compression error: {}", e),
            ZToolsError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
            ZToolsError::PathError(e) => write!(f, "Path error: {}", e),
            ZToolsError::SevenZipError(e) => write!(f, "7z compression error: {}", e),
            ZToolsError::GzipError(e) => write!(f, "Gzip error: {}", e),
            ZToolsError::UntarError(e) => write!(f, "Untar error: {}", e),
            ZToolsError::SpawnError(e) => write!(f, "Spawn error: {}", e),
            ZToolsError::PermissionError(e) => write!(f, "Permission error: {}", e),
        }
    }
}

impl std::error::Error for ZToolsError {}

impl From<io::Error> for ZToolsError {
    fn from(err: io::Error) -> Self {
        ZToolsError::Io(err)
    }
}
