pub mod basic_ops;
pub mod advanced_ops;
pub mod utils;
pub mod shortcuts;

use crate::{
    basic_ops::{add, subtract, multiply, divide},
    advanced_ops::{power, sqrt, logarithm},
    utils::validate_input,
};

/// Calculates the result based on the input string
/// # Arguments - * `input` - A string containing operation and numbers (e.g., "add 5 3")
/// # Returns - * `Result<f64, String>` - The calculation result or an error message
pub fn calculate(input: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    validate_input(&tokens)?;

    match tokens[0] {
        "add" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(add(a, b))
        },
        "subtract" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(subtract(a, b))
        },
        // Fixed: multiply was missing from calculate()
        "multiply" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(multiply(a, b))
        },
        "divide" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            divide(a, b)
        },
        "power" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(power(a, b))
        },
        "sqrt" => {
            let a = tokens[1].parse::<f64>().unwrap();
            sqrt(a)
        },
        // Fixed: logarithm was missing from calculate()
        "logarithm" | "log" => {
            let a = tokens[1].parse::<f64>().unwrap();
            logarithm(a)
        },
        _ => Err(format!("Unknown operation: {}", tokens[0]))
    }
}