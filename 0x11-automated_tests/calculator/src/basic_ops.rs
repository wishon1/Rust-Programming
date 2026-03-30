// Module containing basic arithmetic operations

/// Adds two numbers together
/// # Arguments
///     * `a` - The first operand
///     * `b` - The second operand
/// # Returns - * `f64` - The sum of a and b
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts b from a
/// # Arguments
///     * `a` - The first operand
///     * `b` - The second operand
/// # Returns - * `f64` - The difference of a and b
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers together
/// # Arguments
///     * `a` - The first operand
///     * `b` - The second operand
/// # Returns - * `f64` - The product of a and b
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides a by b
/// # Arguments
///     * `a` - The dividend
///     * `b` - The divisor
/// # Returns - * `Result<f64, String>` - The quotient, or an error if b is zero
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}