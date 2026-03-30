// Module containing calculator utilities

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
                // Fixed: was incorrectly using tokens[1] for the operation name
                return Err(format!("{} operation requires 2 operands", tokens[0]));
            }
            tokens[1].parse::<f64>()
                .map_err(|_| format!("Invalid first operand: {}", tokens[1]))?;
            tokens[2].parse::<f64>()
                .map_err(|_| format!("Invalid second operand: {}", tokens[2]))?;
        }

        // sqrt and logarithm both take exactly 1 operand
        "sqrt" | "logarithm" | "log" => {
            if tokens.len() != 2 {
                return Err(format!("{} operation requires 1 operand", tokens[0]));
            }
            tokens[1].parse::<f64>()
                .map_err(|_| format!("Invalid operand: {}", tokens[1]))?;
        }

        _ => return Err(format!("Unknown operation: {}", tokens[0])),
    }
    Ok(())
}

/// Formats the calculation result for display
/// # Arguments
///     * `result` - The result to format
/// # Returns - * `String` - The formatted result
pub fn format_result(result: f64) -> String {
    // Show as integer if the result is a whole number, otherwise show decimal
    if result.fract() == 0.0 {
        format!("{}", result as i64)
    } else {
        format!("{:.6}", result).trim_end_matches('0').to_string()
    }
}