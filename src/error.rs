//! Error types for claude-list

use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Failed to read directory: {0}")]
    DirectoryReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Invalid component in {0}: {1}")]
    InvalidComponent(PathBuf, String),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Malformed frontmatter in {0}")]
    MalformedFrontmatter(PathBuf),
}

impl Error {
    pub fn is_fatal(&self) -> bool {
        matches!(self, Error::DirectoryNotFound(_))
    }
}
