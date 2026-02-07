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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_is_fatal_directory_not_found() {
        let error = Error::DirectoryNotFound(PathBuf::from("/test/path"));
        assert!(error.is_fatal());
    }

    #[test]
    fn test_error_is_fatal_directory_read_error() {
        let error = Error::DirectoryReadError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error",
        ));
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_error_is_fatal_json_parse_error() {
        let error = Error::JsonParseError(
            serde_json::from_str::<serde_json::Value>("invalid").unwrap_err(),
        );
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_error_is_fatal_invalid_component() {
        let error = Error::InvalidComponent(PathBuf::from("/test/path"), "test error".to_string());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_error_is_fatal_permission_denied() {
        let error = Error::PermissionDenied(PathBuf::from("/test/path"));
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_error_is_fatal_missing_field() {
        let error = Error::MissingField("name".to_string());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_error_is_fatal_malformed_frontmatter() {
        let error = Error::MalformedFrontmatter(PathBuf::from("/test/path"));
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_error_display_directory_not_found() {
        let error = Error::DirectoryNotFound(PathBuf::from("/test/path"));
        let error_string = format!("{}", error);
        assert!(error_string.contains("Directory not found"));
        assert!(error_string.contains("/test/path"));
    }

    #[test]
    fn test_error_display_permission_denied() {
        let error = Error::PermissionDenied(PathBuf::from("/test/path"));
        let error_string = format!("{}", error);
        assert!(error_string.contains("Permission denied"));
        assert!(error_string.contains("/test/path"));
    }

    #[test]
    fn test_error_display_missing_field() {
        let error = Error::MissingField("version".to_string());
        let error_string = format!("{}", error);
        assert!(error_string.contains("Missing required field"));
        assert!(error_string.contains("version"));
    }

    #[test]
    fn test_error_display_invalid_component() {
        let error =
            Error::InvalidComponent(PathBuf::from("/test/path"), "invalid format".to_string());
        let error_string = format!("{}", error);
        assert!(error_string.contains("Invalid component"));
        assert!(error_string.contains("/test/path"));
        assert!(error_string.contains("invalid format"));
    }
}
