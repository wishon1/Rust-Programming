// 0x07. Rust - Packages, Crates, and Modules

pub mod basic_ops;
pub mod advanced_ops;
pub mod utils;

// Calculates the result based on the input string
/// # Arguments
///     * `input` - A string containing operation and numbers (e.g., "add 5 3")
/// # Returns - * `Result<f64, String>` - The calculation result or an error message

pub fn calculate(input: &str) -> Result<f64, String> {
    // split the input into tokens
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // validate the input with the util module
    utils::validate_input(&tokens)?;

    //parse the operation and perform the calculation
    match tokens[0] {
        "add" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(basic_ops::add(a, b))
        },
    "subtract" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(basic_ops::subtract(a, b))
        },
        "multiply" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(basic_ops::multiply(a, b))
        },
        "divide" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            basic_ops::divide(a, b)
        },
        "power" => {
            let a = tokens[1].parse::<f64>().unwrap();
            let b = tokens[2].parse::<f64>().unwrap();
            Ok(advanced_ops::power(a, b))
        },
        "sqrt" => {
            let a = tokens[1].parse::<f64>().unwrap();
            advanced_ops::sqrt(a)
        },
        _ => Err(format!("Unknown operation: {}", tokens[0]))
    }
}