//! # errors.rs
//!
//! Custom error type for `cli_error_handling`, plus its exit-code mapping.
//!
//! ## Design philosophy
//!
//! CLI tools should communicate failure through **both** a human-readable
//! message *and* a non-zero exit code.  Shell scripts (`$?`) and CI pipelines
//! rely on exit codes; humans rely on messages.  This module gives us both.
//!
//! | Variant             | Meaning                             | Exit code |
//! |---------------------|-------------------------------------|-----------|
//! | `FileNotFound`      | Path does not exist                 | 2         |
//! | `PermissionDenied`  | OS rejected access                  | 3         |
//! | `InvalidFormat`     | File contents are malformed         | 4         |
//! | `IoError`           | Any other I/O failure               | 1         |

use std::fmt;
use std::io;

// ---------------------------------------------------------------------------
// Error enum
// ---------------------------------------------------------------------------

/// All failure modes that the CLI can encounter.
#[derive(Debug)]
pub enum CliError {
    /// The requested path does not exist in the file system.
    FileNotFound(String),

    /// The current user does not have the necessary permissions.
    PermissionDenied { path: String, cause: String },

    /// The file exists but its contents are not in the expected format.
    InvalidFormat(String),

    /// A catch-all for unexpected I/O errors.
    IoError(io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::FileNotFound(path) => write!(f, "File not found: {path}"),
            CliError::PermissionDenied { path, cause } => {
                write!(f, "PermissionError\nDetails: Failed to access file: {path}\nCause: {cause}\nHint: Check file permissions or run with elevated privileges")
            }
            CliError::InvalidFormat(reason) => write!(f, "Invalid file format: {reason}"),
            CliError::IoError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        use io::ErrorKind;
        match e.kind() {
            ErrorKind::NotFound => {
                CliError::FileNotFound("(path unknown at conversion site)".to_string())
            }
            ErrorKind::PermissionDenied => CliError::PermissionDenied {
                path: "(path unknown at conversion site)".to_string(),
                cause: e.to_string(),
            },
            _ => CliError::IoError(e),
        }
    }
}

impl CliError {
    /// Maps each error variant to a POSIX-style exit code.
    ///
    /// Convention:
    /// - `0` = success (never returned here)
    /// - `1` = generic / unexpected error
    /// - `2` = file not found
    /// - `3` = permission denied
    /// - `4` = invalid format
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::IoError(_) => 1,
            CliError::FileNotFound(_) => 2,
            CliError::PermissionDenied { .. } => 3,
            CliError::InvalidFormat(_) => 4,
        }
    }
}