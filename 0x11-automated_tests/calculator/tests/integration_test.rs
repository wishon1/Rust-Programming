//! # Integration Tests — `calculator` crate
//!
//! This file lives in `tests/` which means Rust compiles it as a completely
//! separate program that imports `calculator` like any external consumer would.
//!
//! In C terms: think of this as a separate `test_main.c` that links against
//! your compiled library via its public header.
//! In Python terms: think of this as `test_calculator.py` that does
//! `import calculator` at the top.
//!
//! ## What is tested
//!
//! | Section | Coverage |
//! |---------|----------|
//! | 1 | `basic_ops` — add, subtract, multiply, divide |
//! | 2 | `advanced_ops` — power, sqrt, logarithm |
//! | 3 | `calculate()` string dispatcher — happy paths |
//! | 4 | `calculate()` string dispatcher — error paths |
//! | 5 | `format_result` output formatting |
//! | 6 | Cross-module composition (true integration) |
//!
//! ## Running the tests
//!
//! ```bash
//! cargo test                        # run all tests
//! cargo test test_add               # run only tests whose name contains "test_add"
//! cargo test -- --nocapture         # show println! output during tests
//! ```

use calculator::calculate;
use calculator::shortcuts::{
    add, subtract, multiply, divide, power, sqrt, logarithm, format_result,
};

// -----------------------------------------------------------------------------
// Helper
// -----------------------------------------------------------------------------

/// Checks whether two `f64` values are close enough to be considered equal.
///
/// Floating point numbers are never exactly equal after arithmetic — the same
/// reason you use `fabs(a - b) < epsilon` in C instead of `a == b`.
///
/// # Arguments
/// * `a` — the actual value produced by the function under test
/// * `b` — the expected value you are comparing against
///
/// # Returns
/// `true` if the absolute difference between `a` and `b` is less than
/// `0.000_000_001` (i.e. `1e-9`), otherwise `false`.
fn approx_eq(a: f64, b: f64) -> bool {
    let difference = a - b;
    let positive_num = difference.abs(); // same as fabs() in C
    positive_num < 0.000_000_001        // no semicolon = implicit return in Rust
}

// ====================================================================
// SECTION 1 — basic_ops via shortcuts re-exports
// ====================================================================

/// Verifies that adding two positive numbers produces the correct sum.
#[test]
fn test_add_post_num() {
    assert!(approx_eq(add(5.0, 3.0), 8.0));
}

/// Verifies that adding two negative numbers produces a negative sum.
#[test]
fn test_add_negt_num() {
    assert!(approx_eq(add(-4.0, -6.0), -10.0));
}

/// Verifies that adding a negative and a positive number produces the correct result.
#[test]
fn test_add_mixed_num() {
    assert!(approx_eq(add(-3.0, 7.0), 4.0));
}

/// Verifies the additive identity: any number plus zero equals itself.
#[test]
fn test_add_zero() {
    assert!(approx_eq(add(12.0, 0.0), 12.0));
}

/// Verifies that subtracting a smaller number from a larger one gives a positive result.
#[test]
fn test_subtract_post_int() {
    assert!(approx_eq(subtract(12.0, 10.0), 2.0));
}

/// Verifies that subtracting a larger number from a smaller one gives a negative result.
#[test]
fn test_subtract_negt_int() {
    assert!(approx_eq(subtract(3.0, 6.0), -3.0));
}

/// Verifies the reflexive property: any number minus itself equals zero.
#[test]
fn test_subtract_self_is_zero() {
    assert!(approx_eq(subtract(9.0, 9.0), 0.0));
}

/// Verifies that multiplying two positive numbers gives a positive product.
#[test]
fn test_multiply_positive() {
    assert!(approx_eq(multiply(4.0, 2.0), 8.0));
}

/// Verifies that multiplying two negative numbers gives a positive product.
///
/// This mirrors the mathematical rule: negative × negative = positive.
#[test]
fn test_multiply_neg_num() {
    assert!(approx_eq(multiply(-3.0, -4.0), 12.0));
}

/// Verifies that multiplying a positive and a negative number gives a negative product.
#[test]
fn test_multiply_mixed_num() {
    assert!(approx_eq(multiply(6.0, -3.0), -18.0));
}

/// Verifies that dividing two positive numbers that produce an exact whole result works.
///
/// Uses `assert_eq!` (not `approx_eq`) because `4.0 / 2.0 = 2.0` is exact —
/// no floating point imprecision involved.
/// `divide()` returns `Result<f64, String>` so we compare against `Ok(2.0)` directly.
#[test]
fn test_divide_positive_num() {
    assert_eq!(divide(4.0, 2.0), Ok(2.0));
}

/// Verifies that dividing numbers that produce a repeating decimal works correctly.
///
/// `1.0 / 3.0` is `0.333...` repeating — it cannot be hardcoded exactly.
/// Instead we compare `divide()`'s output against Rust's own `/` operator,
/// which has the same floating point representation. `.unwrap()` extracts the
/// `f64` out of the `Ok()` wrapper so `approx_eq` can receive a plain `f64`.
#[test]
fn test_divide_fractions() {
    let result = divide(1.0, 3.0).unwrap();
    assert!(approx_eq(result, 1.0 / 3.0));
}

/// Verifies that dividing by zero returns an `Err` containing the word "zero".
///
/// Tests both that the error variant is returned AND that the message is meaningful —
/// a silent or misleading error message would be just as bad as a wrong result.
#[test]
fn test_divide_by_zero_returns_error() {
    let result = divide(7.0, 0.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_lowercase().contains("zero"));
}

/// Verifies that dividing a negative dividend by a positive divisor gives a negative result.
#[test]
fn test_divide_negative_dividend() {
    assert_eq!(divide(-10.0, 2.0), Ok(-5.0));
}

// ====================================================================
// SECTION 2 — advanced_ops via shortcuts re-exports
// ====================================================================

/// Verifies that raising 2 to the power of 10 gives 1024.
#[test]
fn test_power_integer_exponent() {
    assert!(approx_eq(power(2.0, 10.0), 1024.0));
}

/// Verifies the zero exponent rule: any base raised to the power of 0 equals 1.
#[test]
fn test_power_zero_exponent() {
    assert!(approx_eq(power(42.0, 0.0), 1.0));
}

/// Verifies that fractional exponents work correctly.
///
/// `8^(1/3)` is the cube root of 8, which equals 2.
/// This also confirms that `power()` handles non-integer exponents.
#[test]
fn test_power_fractional_exponent() {
    assert!(approx_eq(power(8.0, 1.0 / 3.0), 2.0));
}

/// Verifies that a negative base raised to an odd integer exponent gives a negative result.
#[test]
fn test_power_negative_base() {
    assert!(approx_eq(power(-2.0, 3.0), -8.0));
}

/// Verifies that the square root of a perfect square returns the exact integer root.
///
/// Uses `assert_eq!` because `sqrt(144.0)` is exactly `12.0` with no imprecision.
#[test]
fn test_sqrt_perfect_square() {
    assert_eq!(sqrt(144.0), Ok(12.0));
}

/// Verifies that the square root of zero is zero.
#[test]
fn test_sqrt_zero() {
    assert_eq!(sqrt(0.0), Ok(0.0));
}

/// Verifies that the square root of a non-perfect square matches Rust's built-in constant.
///
/// `SQRT_2` is the standard library's precomputed value for √2, used as the
/// reference instead of a hardcoded decimal to avoid imprecision mismatches.
#[test]
fn test_sqrt_non_perfect_square() {
    let result = sqrt(2.0).unwrap();
    assert!(approx_eq(result, std::f64::consts::SQRT_2));
}

/// Verifies that taking the square root of a negative number returns an `Err`.
///
/// Square roots of negative numbers are not real numbers, so the function
/// must reject them with a descriptive error message containing "negative".
#[test]
fn test_sqrt_negative_returns_err() {
    let result = sqrt(-9.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_lowercase().contains("negative"));
}

/// Verifies that the natural logarithm of e equals 1.
///
/// By definition, ln(e) = 1. `std::f64::consts::E` is Rust's built-in
/// value for e (Euler's number ≈ 2.71828...).
#[test]
fn test_logarithm_of_e_is_one() {
    let result = logarithm(std::f64::consts::E).unwrap();
    assert!(approx_eq(result, 1.0));
}

/// Verifies that the natural logarithm of 1 equals 0.
///
/// By definition, ln(1) = 0 because e^0 = 1.
#[test]
fn test_logarithm_of_one_is_zero() {
    let result = logarithm(1.0).unwrap();
    assert!(approx_eq(result, 0.0));
}

/// Verifies that taking the logarithm of zero returns an `Err`.
///
/// ln(0) is negative infinity — undefined for practical purposes.
#[test]
fn test_logarithm_of_zero_returns_err() {
    let result = logarithm(0.0);
    assert!(result.is_err());
}

/// Verifies that taking the logarithm of a negative number returns an `Err`.
///
/// Logarithms of negative numbers are not real numbers.
/// The error message must contain "non-positive", "negative", or "positive"
/// to confirm it is descriptive and not a generic fallback.
#[test]
fn test_logarithm_of_negative_returns_err() {
    let result = logarithm(-5.0);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_lowercase();
    assert!(msg.contains("non-positive") || msg.contains("negative") || msg.contains("positive"));
}

// ====================================================================
// SECTION 3 — calculate() string dispatcher (happy paths)
// ====================================================================

/// Verifies that the string dispatcher correctly routes "add" to the add function.
#[test]
fn test_calculate_add() {
    assert!(approx_eq(calculate("add 100 200").unwrap(), 300.0));
}

/// Verifies that the string dispatcher correctly routes "subtract" to the subtract function.
#[test]
fn test_calculate_subtract() {
    assert!(approx_eq(calculate("subtract 50 18").unwrap(), 32.0));
}

/// Verifies that the string dispatcher correctly routes "multiply" to the multiply function.
#[test]
fn test_calculate_multiply() {
    assert!(approx_eq(calculate("multiply 6 7").unwrap(), 42.0));
}

/// Verifies that the string dispatcher correctly routes "divide" to the divide function.
///
/// Uses `22.0 / 7.0` as the expected value rather than a hardcoded decimal
/// because the result is a repeating fraction.
#[test]
fn test_calculate_divide() {
    assert!(approx_eq(calculate("divide 22 7").unwrap(), 22.0 / 7.0));
}

/// Verifies that the string dispatcher correctly routes "power" to the power function.
#[test]
fn test_calculate_power() {
    assert!(approx_eq(calculate("power 3 4").unwrap(), 81.0));
}

/// Verifies that the string dispatcher correctly routes "sqrt" to the sqrt function.
#[test]
fn test_calculate_sqrt() {
    assert!(approx_eq(calculate("sqrt 256").unwrap(), 16.0));
}

/// Verifies that both "logarithm" and "log" keywords route to the same function.
///
/// The dispatcher supports both spellings as aliases. This test confirms
/// both produce identical output for the same input.
#[test]
fn test_calculate_log_alias() {
    let via_log = calculate("log 2.718281828459045").unwrap();
    let via_logarithm = calculate("logarithm 2.718281828459045").unwrap();
    assert!(approx_eq(via_log, via_logarithm));
}

// ====================================================================
// SECTION 4 — calculate() string dispatcher (error paths)
// ====================================================================

/// Verifies that passing "divide X 0" through the dispatcher returns an `Err`.
#[test]
fn test_calculate_divide_by_zero() {
    let result = calculate("divide 1 0");
    assert!(result.is_err());
}

/// Verifies that passing "sqrt" with a negative number through the dispatcher returns an `Err`.
#[test]
fn test_calculate_sqrt_negative() {
    let result = calculate("sqrt -4");
    assert!(result.is_err());
}

/// Verifies that passing "log" with a negative number through the dispatcher returns an `Err`.
#[test]
fn test_calculate_log_negative() {
    let result = calculate("log -5");
    assert!(result.is_err());
}

/// Verifies that an empty string input returns an `Err` rather than panicking.
#[test]
fn test_calculate_empty_input() {
    let result = calculate("");
    assert!(result.is_err());
}

/// Verifies that an unrecognised operation name returns an `Err` containing "unknown".
#[test]
fn test_calculate_unknown_operation() {
    let result = calculate("modulo 10 3");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_lowercase().contains("unknown"));
}

/// Verifies that a binary operation given only one operand returns an `Err`.
///
/// "add" requires exactly 2 operands. Passing only 1 must be rejected by
/// `validate_input` before any arithmetic is attempted.
#[test]
fn test_calculate_missing_operand_for_binary_op() {
    let result = calculate("add 5");
    assert!(result.is_err());
}

/// Verifies that a unary operation given two operands returns an `Err`.
///
/// "sqrt" requires exactly 1 operand. Passing 2 must be rejected by
/// `validate_input`.
#[test]
fn test_calculate_too_many_operands_for_unary_op() {
    let result = calculate("sqrt 4 9");
    assert!(result.is_err());
}

/// Verifies that a non-numeric token in the operand position returns an `Err`.
///
/// "five" cannot be parsed as an `f64`, so `validate_input` must catch this
/// and return a descriptive error rather than panicking.
#[test]
fn test_calculate_invalid_number_token() {
    let result = calculate("add five 3");
    assert!(result.is_err());
}

// ====================================================================
// SECTION 5 — format_result (via shortcuts)
// ====================================================================

/// Verifies that a whole number result is formatted without a decimal point.
///
/// `42.0` should display as `"42"`, not `"42.000000"`.
#[test]
fn test_format_result_whole_number() {
    let s = format_result(42.0);
    assert_eq!(s, "42");
    assert!(!s.contains('.'));
}

/// Verifies that a fractional result is formatted with a decimal point.
#[test]
fn test_format_result_decimal() {
    let s = format_result(3.14);
    assert!(s.contains('.'));
}

/// Verifies that trailing zeros are stripped from fractional results.
///
/// `0.5` stored as `f64` would naively format as `"0.500000"`.
/// The formatter must strip those trailing zeros to produce `"0.5"`.
#[test]
fn test_format_result_no_trailing_zeros() {
    let s = format_result(0.5);
    assert!(!s.ends_with('0'), "trailing zero found in: {}", s);
}

/// Verifies that a negative whole number is formatted correctly without decimals.
#[test]
fn test_format_result_negative_whole() {
    let s = format_result(-7.0);
    assert_eq!(s, "-7");
}

// ====================================================================
// SECTION 6 — cross-module composition (the "integration" heart)
// ====================================================================

/// Verifies that `sqrt(power(x, 2))` returns the original value.
///
/// This chains `power` and `sqrt` together — the kind of real-world usage
/// that unit tests inside `src/` cannot simulate cleanly because they test
/// each function in isolation.
#[test]
fn test_power_then_sqrt_is_identity() {
    let x = 7.0_f64;
    let squared = power(x, 2.0);
    let root = sqrt(squared).unwrap();
    assert!(approx_eq(root, x));
}

/// Verifies that `logarithm(power(e, n))` returns `n`.
///
/// By definition, ln(e^n) = n. This test chains `power` and `logarithm`
/// to confirm the two modules agree mathematically.
#[test]
fn test_log_of_power_is_exponent() {
    let n = 5.0_f64;
    let e_to_n = power(std::f64::consts::E, n);
    let log_result = logarithm(e_to_n).unwrap();
    assert!(approx_eq(log_result, n));
}

/// Verifies that two chained `calculate()` calls produce the correct result.
///
/// Computes `(10 + 20) / 3` using two separate string dispatcher calls,
/// feeding the output of the first as input to the second.
/// This mirrors how a real application might pipeline operations.
#[test]
fn test_add_then_divide_via_calculate() {
    let sum = calculate("add 10 20").unwrap();      // produces 30.0
    let expr = format!("divide {} 3", sum);         // builds "divide 30 3"
    let result = calculate(&expr).unwrap();
    assert!(approx_eq(result, 10.0));
}

/// Verifies that `format_result` and `calculate()` compose correctly end-to-end.
///
/// `22 / 7` is a repeating decimal. The formatted output must contain a decimal
/// point and must not end with a trailing zero.
#[test]
fn test_format_result_on_calculate_output() {
    let result = calculate("divide 22 7").unwrap();
    let formatted = format_result(result);
    assert!(formatted.contains('.'));
    assert!(!formatted.ends_with('0'));
}

/// Verifies the distributive law holds across `multiply` and `add`.
///
/// Mathematically: `a * (b + c) == (a * b) + (a * c)`.
/// This confirms that `multiply` and `add` interact correctly when composed.
#[test]
fn test_multiply_distributes_over_add() {
    let a = 3.0_f64;
    let b = 4.0_f64;
    let c = 5.0_f64;

    let left  = multiply(a, add(b, c));
    let right = add(multiply(a, b), multiply(a, c));
    assert!(approx_eq(left, right));
}