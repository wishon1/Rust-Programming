// 0x07. Rust - Packages, Crates, and Modules

/// Raises a number to a power
/// # Arguments
//      * `base` - The base number
///     * `exponent` - The exponent
/// # Returns - * `f64` - The result of base^exponent
pub fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

// Calculates the square root of a number
/// # Arguments
///     * `a` - The number
/// # Returns - * `Result<f64, String>` - The square root of a or an error if a is negative
pub fn sqrt(a: f64) -> Result<f64, String> {
    if a < 0.0 {
        Err("Cannot calculate square root of a negative number".to_string())
    } else {
        Ok(a.sqrt())
    }
}

/// Calculates the natural logarithm of a number
/// # Arguments
///  * `a` - The number
/// # Returns - * `Result<f64, String>` - The natural log of a or an error if a is negative or zero
pub fn logarithm(a: f64) -> Result<f64, String> {
    if a <= 0.0 {
        Err("Cannot calculate logarithm of a non-positive number".to_string())
    } else {
        Ok(a.ln())
    }
}