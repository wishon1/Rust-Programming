// 0x07. Rust - Packages, Crates, and Modules

/// Validates the input tokens for the calculator
/// # Arguments
/// * `tokens` - Vector of string tokens from the input
/// # Returns - * `Result<(), String>` - Ok if valid, Err with a message if invalid
pub fn validate_input(tokens: &Vec<&str>) -> Result<(), String> {
    if tokens.is_empty() {
        return Err("Empty input".to_string());
    }

    match tokens[0] {
        "add" | "subtract" | "multiply" | "divide" | "power" => {
            if tokens.len() != 3 {
                return Err(format!("{} operation requires 2 operands", tokens[0]));
            }
            // Try to parse the numbers
            tokens[1].parse::<f64>().map_err(|_| format!("Invalid first operand: {}", tokens[1]))?;
            tokens[2].parse::<f64>().map_err(|_| format!("Invalid second operand: {}", tokens[2]))?;
        },
        "sqrt" => {
            if tokens.len() != 2 {
                return Err("sqrt operation requires 1 operand".to_string());
            }
            // Try to parse the number
            tokens[1].parse::<f64>().map_err(|_| format!("Invalid operand: {}", tokens[1]))?;
        },
        _ => return Err(format!("Unknown operation: {}", tokens[0])),
    }

    Ok(())
}

/// Formats the calculation result
/// # Arguments
///     * `result` - The result to format
/// # Returns - * `String` - The formatted result
pub fn format_result(result: f64) -> String {
    result.to_string()
}