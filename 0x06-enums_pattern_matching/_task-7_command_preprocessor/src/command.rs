// 0x06. Rust - Enums and Pattern Matching

/// Represents different types of commands for a simple calculator
#[derive(Debug)]
pub enum Command {
    /// Addition operation with two operands
    Add(i32, i32),
    /// Subtraction operation with two operands
    Subtract(i32, i32),
    /// Multiplication operation with two operands
    Multiply(i32, i32),
    /// Division operation with two operands
    Divide(i32, i32),
    /// Exit the program
    Exit,
}

/// Processes a command and returns the updated result
/// # Arguments
///     * `command` - The Command to process
///     * `current_result` - The current result value
/// # Returns
/// * `i32` - The updated result after processing the command
pub fn process_command(command: Command, current_result: i32) -> i32 {
    // Match on the command to perform the appropriate operation
    match command {
        Command::Add(a, b) => {
            let result = a + b;
            result
        }
        Command::Subtract(a, b) => {
            let result = a - b;
            result
        }
        Command::Multiply(a, b) => {
            let result = a * b;
            result
        }
        Command::Divide(a, b) => {
            if b == 0 {
                println!("Cannot divide by zero");
                current_result
            } else {
                let result = a / b;
                result
            }
        }
        Command::Exit => {
            println!("Exiting...");
            current_result
        }
    }
}

/// Parses a string into a Command
/// # Arguments
///     * `cmd_str` - The string to parse
/// # Returns
///     * `Command` - The parsed command
pub fn parse_command(cmd_str: &str) -> Command {
    // Split the string by spaces
    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    
    // Match on the first part to determine the command type
    if parts.is_empty() {
        return Command::Exit;
    }
    
    // Parse based on the command type
    match parts[0] {
        "add" => {
            if parts.len() >= 3 {
                // Parse the two operands
                let a = parts[1].parse::<i32>().unwrap_or(0);
                let b = parts[2].parse::<i32>().unwrap_or(0);
                Command::Add(a, b)
            } else {
                // Not enough operands
                Command::Exit
            }
        }
        "subtract" => {
            if parts.len() >= 3 {
                let a = parts[1].parse::<i32>().unwrap_or(0);
                let b = parts[2].parse::<i32>().unwrap_or(0);
                Command::Subtract(a, b)
            } else {
                Command::Exit
            }
        }
        "multiply" => {
            if parts.len() >= 3 {
                let a = parts[1].parse::<i32>().unwrap_or(0);
                let b = parts[2].parse::<i32>().unwrap_or(0);
                Command::Multiply(a, b)
            } else {
                Command::Exit
            }
        }
        "divide" => {
            if parts.len() >= 3 {
                let a = parts[1].parse::<i32>().unwrap_or(0);
                let b = parts[2].parse::<i32>().unwrap_or(0);
                Command::Divide(a, b)
            } else {
                Command::Exit
            }
        }
        "exit" => Command::Exit,
        _ => Command::Exit,
    }
}