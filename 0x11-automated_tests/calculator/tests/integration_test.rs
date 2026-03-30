// integration test for the 'calculator module' crate

use calculator::calculator;
use calculator::shortcuts::{
        add, subtract
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
    assert!(approx_eq(add(5.0, 3.0)), 8.0);
}

/// test_add_negative_numbers - function that test for negative numbers
#[test]
fn test_add_negt_num() {
    assert!(approx_eq(add(-4.0, -0.6), -10.0));
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
    assert!(approx_eq(subtract(3.0, 6.0)), -3.0);
}

/// test_subtract_self_is_zero - test if both a and b are same return 0.0
#[test]
fn test_subtract_self_is_zero() {
    assert!(approx_eq(subtract(9.0, 9.0), 0.0));
}