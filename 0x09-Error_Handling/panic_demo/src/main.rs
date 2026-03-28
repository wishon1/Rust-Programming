//! # panic_demo
//!
//! Demonstrates the `panic!` macro and Rust's approach to **unrecoverable errors**.
//!
//! ## Background (for Python / C developers)
//!
//! | Concept            | Python equivalent          | C equivalent                       |
//! |--------------------|----------------------------|------------------------------------|
//! | `panic!`           | `raise RuntimeError(...)`  | `abort()` / undefined behaviour    |
//! | `RUST_BACKTRACE=1` | `traceback.print_exc()`    | running under a debugger           |
//! | `panic = "abort"`  | N/A (always unwinds)       | compiling without `-funwind-tables` |
//!
//! ## When to use `panic!`
//!
//! - A **programming contract** (invariant) has been violated and continuing
//!   would lead to undefined or nonsensical state.
//! - You are writing a **prototype or test** and want to fail loudly.
//! - An **impossible branch** is reached (e.g., `unreachable!()`).
//!
//! Never use `panic!` for expected, user-facing failures — use `Result<T, E>`
//! for those (see `result_demo`).
//!
//! ## Backtrace
//!
//! ```bash
//! RUST_BACKTRACE=1   cargo run   # condensed trace
//! RUST_BACKTRACE=full cargo run  # includes inlined frames
//! ```

// ---------------------------------------------------------------------------
// Function 1 – Basic explicit panic
// ---------------------------------------------------------------------------

/// Triggers a deliberate `panic!` with a human-readable message.
///
/// Use `panic!` when an **invariant** that must always hold has been broken
/// and the only sane response is to abort the current thread.
///
/// # Panics
///
/// Always panics — intentional for demonstration.
fn basic_panic() {
    println!("Function 1: Demonstrating basic panic!");
    // Python equivalent: raise RuntimeError("This is a deliberate panic")
    // C equivalent:      abort();
    panic!("This is a deliberate panic");
}

// ---------------------------------------------------------------------------
// Function 2 – Out-of-bounds index
// ---------------------------------------------------------------------------

/// Demonstrates how Rust panics on an out-of-bounds array index.
///
/// In **C**, an out-of-bounds index is undefined behaviour — silent memory
/// corruption or a segfault.  In **Python** you get an `IndexError`.
/// Rust's bounds checking turns it into a controlled, debuggable `panic!`.
///
/// # Panics
///
/// Always panics: `numbers[10]` on a 4-element array.
fn out_of_bounds_panic() {
    println!("Function 2: Demonstrating out-of-bounds index panic!");
    let numbers = [1, 2, 3, 4]; // fixed-size array, length = 4
    println!("Attempting to access index 10 of a 4-element array...");
    // Use a runtime variable so the bounds check happens at runtime
    // (a compile-time constant index is caught by the compiler as a
    // hard error; a runtime index produces the familiar panic message).
    let idx: usize = std::hint::black_box(10);
    let _value = numbers[idx];
}

// ---------------------------------------------------------------------------
// Function 3 – Safe divide with explicit panic guard
// ---------------------------------------------------------------------------

/// Performs integer division, panicking if the divisor is zero.
///
/// `panic!` is appropriate here because a zero divisor is a **caller bug**
/// (a violated precondition), not an expected runtime condition.  For
/// user-supplied denominators, prefer returning `Result<i32, MyError>`.
///
/// # Arguments
///
/// * `dividend` – The number to be divided.
/// * `divisor`  – Must **not** be zero.
///
/// # Returns
///
/// Integer quotient `dividend / divisor`.
///
/// # Panics
///
/// Panics with `"Division by zero is undefined!"` when `divisor == 0`.
fn safe_divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("Division by zero is undefined!");
    }
    dividend / divisor
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Runs the panic demos in sequence.
///
/// Because `panic!` terminates the thread, only the *first* demo that panics
/// will run.  Comment out earlier demos to reach the later ones, or run with
/// `RUST_BACKTRACE=1` to inspect the call stack.
fn main() {
    // ── Demo 1: basic explicit panic ────────────────────────────────────────
    // Uncomment the line you want to run; comment out the others.
    basic_panic();

    // ── Demo 2: out-of-bounds (unreachable if Demo 1 fires) ─────────────────
    out_of_bounds_panic();

    // ── Demo 3: divide-by-zero guard ────────────────────────────────────────
    println!("Function 3: Demonstrating safe division with panic on zero!");
    let result = safe_divide(10, 2);
    println!("10 / 2 = {result}");

    // Uncomment to trigger the divide-by-zero panic:
    // let _boom = safe_divide(10, 0);
}