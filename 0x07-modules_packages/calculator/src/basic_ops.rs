// 0x07. Rust - Packages, Crates, and Modules

// Adds two numbers
/// # Arguments
///     * `a` - First number
///     * `b` - Second number
/// # Returns - * `f64` - The sum of a and b
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

// Subtracts the second number from the first
/// # Arguments
///     * `a` - First number
///     * `b` - Second number
/// # Returns - * `f64` - The result of a - b
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers
/// # Arguments
///     * `a` - First number
///     * `b` - Second number
/// # Returns - * `f64` - The product of a and b
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides the first number by the second
/// # Arguments
///     * `a` - First number
///     * `b` - Second number
/// # Returns - * `Result<f64, String>` - The result of a / b or an error if b is zero
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}