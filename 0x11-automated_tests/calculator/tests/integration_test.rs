// integration test for the 'calculator module' crate

use calculator::calculate;
use calculator::shortcuts::{
        add, subtract, multiply, divide, power, sqrt, logarithm, format_result,
};

///  approx_eq: its a funtion that accepts a function and a `f64` and returns a boolean
///  # Args:
///     * `a`<f64> - a function that returns a 64 bit floats 
///     * `b`<f64>: The args to compare with
///  # Return: returns a boolean
fn approx_eq(a: f64, b: f64) -> bool {
    let difference = a - b;
    let positive_num = difference.abs();
    positive_num < 1e-9
}

/// test_add_positive_numbers - test for positive numbers
#[test]
fn test_add_post_num() {
    assert!(approx_eq(add(5.0, 3.0), 8.0));
}

/// test_add_negative_numbers - function that test for negative numbers
#[test]
fn test_add_negt_num() {
    assert!(approx_eq(add(-4.0, -6.0), -10.0));
}

/// test_add_mixed_signed - test when both a positive numa and a negtive num are given
#[test]
fn test_add_mixed_num() {
    assert!(approx_eq(add(-3.0, 7.0), 4.0));
}

/// test_add_zero - this test should return same val even if `0` is added
#[test]
fn test_add_zero() {
    assert!(approx_eq(add(12.0, 0.0), 12.0));
}

/// test_subtract_post_int  - test the subtract funtion with two positive int.
#[test]
fn test_subtract_post_int() {
    assert!(approx_eq(subtract(12.0, 10.0), 2.0));
}

/// test_subtract_negt_int - returns a negative int if b > a
#[test]
fn test_subtract_negt_int() {
    assert!(approx_eq(subtract(3.0, 6.0), -3.0));
}

/// test_subtract_self_is_zero - test if both a and b are same return 0.0
#[test]
fn test_subtract_self_is_zero() {
    assert!(approx_eq(subtract(9.0, 9.0), 0.0));
}

/// test_multiply_positive - test the result of multiplication of two positive integer
#[test]
fn test_multiply_positive() {
    assert!(approx_eq(multiply(4.0, 2.0), 8.0));
}
/// test_multiply_neg - test for negative integers
#[test]
fn test_multiply_neg_num() {
    assert!(approx_eq(multiply(-3.0, -4.0), 12.0));
}

#[test]
fn test_multiply_mixed_num() {
    assert!(approx_eq(multiply(6.0, -3.0), -18.0));
}

#[test]
fn test_divide_positive_num() {
    assert!(approx_eq(divide(4.0, 2.0), Ok(2.0)));
}

#[test]
fn test_divide_fractions() {
    let result = divide(1.0, 3.0).unwrap();
    assert!(approx_eq(result, 1.0 / 3.0))
}

#[test]
fn test_divide_by_zero_reeturns_error() {
    let result = divide(7.0, 0.0);
    assert!(result.is_err());
    
    // The error message must contain "zero"
    assert!(result.unwrap_err().to_lower().contains("zero"));
}

#[test]
fn test_divide_negative_dividend() {
    assert_eq!(divide(-10.0, 2.0), ok(-5.0));
}


// ====================================================================
// SECTION 2 — advanced_ops via shortcuts re-exports
// ====================================================================
 
#[test]
fn test_power_integer_exponent() {
    assert!(approx_eq(power(2.0, 10.0), 1024.0));
}
 
#[test]
fn test_power_zero_exponent() {
    // Any base^0 == 1
    assert!(approx_eq(power(42.0, 0.0), 1.0));
}
 
#[test]
fn test_power_fractional_exponent() {
    // 8^(1/3) == 2
    assert!(approx_eq(power(8.0, 1.0 / 3.0), 2.0));
}
 
#[test]
fn test_power_negative_base() {
    // (-2)^3 == -8
    assert!(approx_eq(power(-2.0, 3.0), -8.0));
}
 
#[test]
fn test_sqrt_perfect_square() {
    assert_eq!(sqrt(144.0), Ok(12.0));
}
 
#[test]
fn test_sqrt_zero() {
    assert_eq!(sqrt(0.0), Ok(0.0));
}
 
#[test]
fn test_sqrt_non_perfect_square() {
    let result = sqrt(2.0).unwrap();
    assert!(approx_eq(result, std::f64::consts::SQRT_2));
}
 
#[test]
fn test_sqrt_negative_returns_err() {
    let result = sqrt(-9.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_lowercase().contains("negative"));
}
 
#[test]
fn test_logarithm_of_e_is_one() {
    let result = logarithm(std::f64::consts::E).unwrap();
    assert!(approx_eq(result, 1.0));
}
 
#[test]
fn test_logarithm_of_one_is_zero() {
    let result = logarithm(1.0).unwrap();
    assert!(approx_eq(result, 0.0));
}
 
#[test]
fn test_logarithm_of_zero_returns_err() {
    let result = logarithm(0.0);
    assert!(result.is_err());
}
 
#[test]
fn test_logarithm_of_negative_returns_err() {
    let result = logarithm(-5.0);
    assert!(result.is_err());
    // Error message must say "non-positive" or similar
    let msg = result.unwrap_err().to_lowercase();
    assert!(msg.contains("non-positive") || msg.contains("negative") || msg.contains("positive"));
}
 
// ====================================================================
// SECTION 3 — calculate() string dispatcher (happy paths)
// ====================================================================
 
#[test]
fn test_calculate_add() {
    assert!(approx_eq(calculate("add 100 200").unwrap(), 300.0));
}
 
#[test]
fn test_calculate_subtract() {
    assert!(approx_eq(calculate("subtract 50 18").unwrap(), 32.0));
}
 
#[test]
fn test_calculate_multiply() {
    assert!(approx_eq(calculate("multiply 6 7").unwrap(), 42.0));
}
 
#[test]
fn test_calculate_divide() {
    assert!(approx_eq(calculate("divide 22 7").unwrap(), 22.0 / 7.0));
}
 
#[test]
fn test_calculate_power() {
    assert!(approx_eq(calculate("power 3 4").unwrap(), 81.0));
}
 
#[test]
fn test_calculate_sqrt() {
    assert!(approx_eq(calculate("sqrt 256").unwrap(), 16.0));
}
 
#[test]
fn test_calculate_logarithm_keyword() {
    // Full "logarithm" keyword
    let result = calculate("logarithm 2.718281828").unwrap();
    assert!(approx_eq(result, 1.0_f64.ln() + (2.718281828_f64).ln()));
    // More direct: ln(e) ≈ 1
    let result2 = calculate("logarithm 2.718281828459045").unwrap();
    assert!(approx_eq(result2, 1.0));
}
 
#[test]
fn test_calculate_log_alias() {
    // "log" alias must resolve to the same function as "logarithm"
    let via_log = calculate("log 2.718281828459045").unwrap();
    let via_logarithm = calculate("logarithm 2.718281828459045").unwrap();
    assert!(approx_eq(via_log, via_logarithm));
}
 
// ====================================================================
// SECTION 4 — calculate() string dispatcher (error paths)
// ====================================================================
 
#[test]
fn test_calculate_divide_by_zero() {
    let result = calculate("divide 1 0");
    assert!(result.is_err());
}
 
#[test]
fn test_calculate_sqrt_negative() {
    let result = calculate("sqrt -4");
    assert!(result.is_err());
}
 
#[test]
fn test_calculate_log_negative() {
    let result = calculate("log -5");
    assert!(result.is_err());
}
 
#[test]
fn test_calculate_empty_input() {
    let result = calculate("");
    assert!(result.is_err());
}
 
#[test]
fn test_calculate_unknown_operation() {
    let result = calculate("modulo 10 3");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_lowercase().contains("unknown"));
}
 
#[test]
fn test_calculate_missing_operand_for_binary_op() {
    // "add" needs 2 operands — only 1 given
    let result = calculate("add 5");
    assert!(result.is_err());
}
 
#[test]
fn test_calculate_too_many_operands_for_unary_op() {
    // "sqrt" takes 1 operand — 2 given
    let result = calculate("sqrt 4 9");
    assert!(result.is_err());
}
 
#[test]
fn test_calculate_invalid_number_token() {
    let result = calculate("add five 3");
    assert!(result.is_err());
}
 
// ====================================================================
// SECTION 5 — format_result (via shortcuts)
// ====================================================================
 
#[test]
fn test_format_result_whole_number() {
    // Whole-number results should have no decimal point
    let s = format_result(42.0);
    assert_eq!(s, "42");
    assert!(!s.contains('.'));
}
 
#[test]
fn test_format_result_decimal() {
    // Fractional results should include a decimal
    let s = format_result(3.14);
    assert!(s.contains('.'));
}
 
#[test]
fn test_format_result_no_trailing_zeros() {
    // "0.500000" should be trimmed to "0.5"
    let s = format_result(0.5);
    assert!(!s.ends_with('0'), "trailing zero found in: {}", s);
}
 
#[test]
fn test_format_result_negative_whole() {
    let s = format_result(-7.0);
    assert_eq!(s, "-7");
}
 
// ====================================================================
// SECTION 6 — cross-module composition (the "integration" heart)
// ====================================================================
 
// These tests chain public APIs the way a real caller would,
// validating that the modules cooperate correctly end-to-end.
 
#[test]
fn test_power_then_sqrt_is_identity() {
    // sqrt(x^2) == x  for x >= 0
    let x = 7.0_f64;
    let squared = power(x, 2.0);
    let root = sqrt(squared).unwrap();
    assert!(approx_eq(root, x));
}
 
#[test]
fn test_log_of_power_is_exponent() {
    // ln(e^n) == n
    let n = 5.0_f64;
    let e_to_n = power(std::f64::consts::E, n);
    let log_result = logarithm(e_to_n).unwrap();
    assert!(approx_eq(log_result, n));
}
 
#[test]
fn test_add_then_divide_via_calculate() {
    // Compute (10 + 20) / 3 using two calculate() calls
    let sum = calculate("add 10 20").unwrap();         // 30.0
    let expr = format!("divide {} 3", sum);
    let result = calculate(&expr).unwrap();
    assert!(approx_eq(result, 10.0));
}
 
#[test]
fn test_format_result_on_calculate_output() {
    // Verify format_result and calculate() compose correctly
    let result = calculate("divide 22 7").unwrap();
    let formatted = format_result(result);
    // Should have decimals and no trailing zeros
    assert!(formatted.contains('.'));
    assert!(!formatted.ends_with('0'));
}
 
#[test]
fn test_multiply_distributes_over_add() {
    // a*(b+c) == a*b + a*c
    let a = 3.0_f64;
    let b = 4.0_f64;
    let c = 5.0_f64;
 
    let left  = multiply(a, add(b, c));
    let right = add(multiply(a, b), multiply(a, c));
    assert!(approx_eq(left, right));
}
