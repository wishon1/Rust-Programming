//! # result_demo
//!
//! Demonstrates `Result<T, E>` — Rust's mechanism for **recoverable errors**.
//!
//! ## Background (for Python / C developers)
//!
//! | Rust concept           | Python equivalent               | C equivalent              |
//! |------------------------|---------------------------------|---------------------------|
//! | `Result<T, E>`         | return value **or** raise       | return code + `errno`     |
//! | `Ok(value)`            | normal return                   | return 0 (success)        |
//! | `Err(e)`               | `raise SomeException()`         | return -1 / set errno     |
//! | `?` operator           | implicit re-raise in a function | `if (ret < 0) return ret` |
//! | `unwrap()` / `expect()`| bare attribute access that crashes | dereference without check |
//!
//! The key insight: in Rust, errors are **values** encoded in the return type.
//! The compiler *forces* you to handle them; you cannot silently ignore a
//! `Result` (you get a `#[must_use]` warning at minimum).

use std::fs;
use std::num::ParseIntError;

// ---------------------------------------------------------------------------
// Custom error type for this demo
// ---------------------------------------------------------------------------

/// Application-level errors for `result_demo`.
///
/// Mirroring a Python custom exception hierarchy:
/// ```python
/// class AppError(Exception): pass
/// class NegativeNumberError(AppError): pass
/// class ParseError(AppError): pass
/// ```
#[derive(Debug)]
enum AppError {
    /// The supplied number is negative; only non-negative values are accepted.
    NegativeNumber(i32),
    /// A string could not be parsed as an integer.
    ParseError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NegativeNumber(n) => {
                write!(f, "Cannot process negative number: {n}")
            }
            AppError::ParseError(msg) => write!(f, "ParseError: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

// Allows `?` to convert `ParseIntError` → `AppError` automatically.
// Python analogy: an implicit `except ParseIntError as e: raise AppError(e)`.
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// 1. Validate user input
// ---------------------------------------------------------------------------

/// Returns `Ok(n)` when `n >= 0`, otherwise `Err(AppError::NegativeNumber)`.
///
/// This is the idiomatic Rust alternative to raising an exception: the error
/// is encoded in the return type, so callers *must* deal with it.
///
/// # Arguments
///
/// * `n` – The integer to validate.
///
/// # Errors
///
/// Returns [`AppError::NegativeNumber`] when `n < 0`.
fn validate_positive(n: i32) -> Result<i32, AppError> {
    if n < 0 {
        Err(AppError::NegativeNumber(n))
    } else {
        Ok(n)
    }
}

// ---------------------------------------------------------------------------
// 2. File reading
// ---------------------------------------------------------------------------

/// Reads the entire contents of a file, returning an `io::Error` on failure.
///
/// In Python you would write:
/// ```python
/// with open(path) as fh:
///     return fh.read()
/// ```
/// and let `FileNotFoundError` propagate.  Here the error is explicit in the
/// return type: the caller *knows* this can fail.
///
/// # Arguments
///
/// * `path` – Filesystem path to read.
///
/// # Errors
///
/// Propagates any [`std::io::Error`] produced by [`fs::read_to_string`].
fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

// ---------------------------------------------------------------------------
// 3. String → integer parsing
// ---------------------------------------------------------------------------

/// Parses a decimal string into an `i32`.
///
/// The `?` operator is equivalent to:
/// ```rust,ignore
/// match s.trim().parse::<i32>() {
///     Ok(n)  => n,
///     Err(e) => return Err(AppError::from(e)),
/// }
/// ```
/// The `From<ParseIntError> for AppError` impl above makes the conversion
/// automatic, keeping call sites clean.
///
/// # Arguments
///
/// * `s` – The string slice to parse.
///
/// # Errors
///
/// Returns [`AppError::ParseError`] when `s` is not a valid integer.
fn parse_number(s: &str) -> Result<i32, AppError> {
    // `?` propagates the error, converting via `From<ParseIntError>`.
    let n: i32 = s.trim().parse()?;
    Ok(n)
}

// ---------------------------------------------------------------------------
// 4. Chained fallible operations
// ---------------------------------------------------------------------------

/// Reads a file, parses its content as a number, and validates it.
///
/// Each `?` short-circuits the function on failure — similar to a chain of
/// `try/except` blocks but without the visual nesting.
///
/// # Arguments
///
/// * `path` – Path to a file whose sole content is a decimal integer.
///
/// # Errors
///
/// - [`std::io::Error`] wrapped in a `Box<dyn std::error::Error>` if the file
///   cannot be opened.
/// - [`AppError`] (as a boxed trait object) if the content cannot be parsed
///   or is negative.
fn process_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // Step 1: read the file — propagate io::Error on failure.
    let contents = read_file(path)?;
    // Step 2: parse the string — propagate AppError on failure.
    let number = parse_number(&contents)?;
    // Step 3: validate the number — propagate AppError on failure.
    let validated = validate_positive(number)?;
    Ok(validated)
}

// ---------------------------------------------------------------------------
// Helper: demonstrate `match`, `unwrap`, `expect`, and `?`
// ---------------------------------------------------------------------------

/// Shows the four most common ways to *handle* a `Result`.
fn demonstrate_result_handling() {
    println!("\nBasic Result handling:");

    // ── match expression (most explicit; analogous to Python if/else) ────────
    match validate_positive(42) {
        Ok(n) => println!("  Success value: {n}"),
        Err(e) => println!("  Error: {e}"),
    }
    match validate_positive(-5) {
        Ok(n) => println!("  Success value: {n}"),
        Err(e) => println!("  Error message: {e}"),
    }

    // ── unwrap / expect ──────────────────────────────────────────────────────
    // Use ONLY when you can *prove* the Result is Ok, e.g. in tests or when
    // the input is a compile-time constant.  In production code, prefer
    // match or `?`.
    let _safe = validate_positive(100).expect("100 is always positive");

    // ── if-let (convenient when you only care about the Ok branch) ───────────
    if let Ok(n) = validate_positive(7) {
        println!("  if-let Ok branch: {n}");
    }
}

/// Demonstrates file reading with `?` and with `match`.
fn demonstrate_file_reading() {
    println!("\nUsing ? operator:");

    // Success path — create a temp file first.
    let tmp = "/tmp/result_demo_test.txt";
    fs::write(tmp, "hello from result_demo").ok();

    match read_file(tmp) {
        Ok(contents) => {
            println!("  Success case: File contents read successfully! ({} bytes)", contents.len())
        }
        Err(e) => println!("  Error: {e}"),
    }

    // Failure path — non-existent file.
    match read_file("/nonexistent/path/file.txt") {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error case: Failed to read file: {e}"),
    }
}

/// Demonstrates `From` trait-based error conversion.
fn demonstrate_error_conversion() {
    println!("\nError conversion:");

    let parse_result: Result<i32, AppError> = parse_number("not_a_number");
    match parse_result {
        Ok(n) => println!("  Parsed: {n}"),
        Err(e) => println!("  Converted to application error: {e}"),
    }
}

/// Demonstrates chaining multiple fallible steps.
fn demonstrate_chained_operations() {
    println!("\nChained operations:");

    // Write a valid file.
    let valid_path = "/tmp/result_demo_valid.txt";
    fs::write(valid_path, "42").ok();

    match process_file(valid_path) {
        Ok(n) => println!("  Success: {n}"),
        Err(e) => println!("  Failure: {e}"),
    }

    // Non-existent file.
    match process_file("/tmp/does_not_exist.txt") {
        Ok(n) => println!("  Success: {n}"),
        Err(e) => println!("  Failure: Chain failed — {e}"),
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Runs all `Result` demos.
fn main() {
    demonstrate_result_handling();
    demonstrate_file_reading();
    demonstrate_error_conversion();
    demonstrate_chained_operations();
}