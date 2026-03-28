// Module containing advanced mathematical operations

/// Calculates the exponent of a number
/// # Arguments
///     * `base` - The base number to work on
///     * `exponent` - The power to raise the base to
/// # Returns - * `f64` - The result of base raised to exponent
pub fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Calculates the square root of a number
/// # Arguments
///     * `a` - The number to take the square root of
/// # Returns - * `Result<f64, String>` - The square root, or an error if a is negative
pub fn sqrt(a: f64) -> Result<f64, String> {
    if a < 0.0 {
        Err("Cannot calculate square root of a negative number".to_string())
    } else {
        Ok(a.sqrt())
    }
}

/// Calculates the natural logarithm of a number
/// # Arguments
///     * `a` - The number to take the logarithm of (must be positive)
/// # Returns - * `Result<f64, String>` - The natural log, or an error if a is non-positive
pub fn logarithm(a: f64) -> Result<f64, String> {
    if a <= 0.0 {
        Err("Cannot calculate logarithm of a non-positive number".to_string())
    } else {
        Ok(a.ln())
    }
}