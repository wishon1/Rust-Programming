//! # custom_errors
//!
//! Shows how to design, implement, and use a **custom error type** for a
//! file-processing application.
//!
//! ## Module layout
//!
//! ```
//! custom_errors/
//! ├── src/
//! │   ├── main.rs   ← you are here; exercises the public API
//! │   └── error.rs  ← defines FileProcessingError
//! └── Cargo.toml
//! ```

mod error;

use error::FileProcessingError;
use std::fs;

// ---------------------------------------------------------------------------
// File-processing functions
// ---------------------------------------------------------------------------

/// Attempts to open and read a file, returning a custom error on failure.
///
/// The `?` operator converts `io::Error` → `FileProcessingError::OpenError`
/// automatically, thanks to the `From<io::Error>` impl in `error.rs`.
///
/// # Arguments
///
/// * `path` – Filesystem path of the file to read.
///
/// # Errors
///
/// Returns [`FileProcessingError::OpenError`] if the file cannot be read.
fn open_file(path: &str) -> Result<String, FileProcessingError> {
    // `?` short-circuits and converts io::Error via the From impl.
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

/// Reads a file and validates that it starts with the expected header.
///
/// Real-world scenario: verifying a magic-byte / format tag before investing
/// time in full parsing (common in C binary protocol parsers).
///
/// # Arguments
///
/// * `path`   – File to read.
/// * `header` – Expected leading string (e.g. `"VALID_HEADER"`).
///
/// # Errors
///
/// - [`FileProcessingError::OpenError`]   – file cannot be read.
/// - [`FileProcessingError::ContentError`] – file exists but header mismatch.
fn process_file(path: &str, header: &str) -> Result<String, FileProcessingError> {
    let contents = open_file(path)?;

    if !contents.starts_with(header) {
        return Err(FileProcessingError::ContentError(
            "Invalid header format".to_string(),
        ));
    }

    Ok("File processed successfully".to_string())
}

/// Simulates a permission check before accessing a restricted file.
///
/// In a real application this might call `libc::access()` (via the `nix`
/// crate) or inspect Unix file mode bits.  Here we simulate it by checking
/// whether the filename contains the word "restricted".
///
/// # Arguments
///
/// * `path` – Path to check.
///
/// # Errors
///
/// Returns [`FileProcessingError::PermissionError`] for paths that are
/// simulated as restricted.
fn check_permission(path: &str) -> Result<(), FileProcessingError> {
    if path.contains("restricted") {
        return Err(FileProcessingError::PermissionError(path.to_string()));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Exercises each error variant and a success path.
fn main() {
    // ── 1. OpenError: file does not exist ────────────────────────────────────
    println!("Testing file operations...");
    match open_file("nonexistent.txt") {
        Ok(c) => println!("  Contents: {c}"),
        Err(e) => println!("  Error: {e}"),
    }

    // ── 2. Success: create a valid file and process it ────────────────────────
    println!("\nCreating empty file for testing...");
    let tmp = "/tmp/custom_errors_test.txt";
    fs::write(tmp, "VALID_HEADER\nsome data").expect("failed to write temp file");
    match process_file(tmp, "VALID_HEADER") {
        Ok(msg) => println!("  Success! File created and processed with result: {msg}"),
        Err(e) => println!("  Error: {e}"),
    }

    // ── 3. ContentError: file exists but header is wrong ─────────────────────
    println!("\nInvalid content test...");
    let bad_tmp = "/tmp/custom_errors_bad.txt";
    fs::write(bad_tmp, "WRONG_HEADER\ndata").expect("failed to write temp file");
    match process_file(bad_tmp, "VALID_HEADER") {
        Ok(msg) => println!("  Success: {msg}"),
        Err(e) => println!("  Error: {e}"),
    }

    // ── 4. PermissionError ────────────────────────────────────────────────────
    println!("\nPermission test...");
    match check_permission("restricted.txt") {
        Ok(()) => println!("  Access granted"),
        Err(e) => println!("  Error: {e}"),
    }
}