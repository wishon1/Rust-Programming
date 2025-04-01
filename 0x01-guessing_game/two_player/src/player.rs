use std::io;

/// Gets a secret number from Player 1. The input is hidden after entry.
/// Returns a valid number between 1 and 100, or defaults to 42 if invalid.
pub fn get_secret_number() -> u32 {
    let mut input = String::new();

    println!("Player 1, enter a secret number (it will be hidden): ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input and validate the range (1 to 100), else default to 42
    match input.trim().parse::<u32>() {
        Ok(num) if num > 0 && num <= 100 => {

            // Clear the terminal to hide the secret number (ANSI escape codes)
            print!("\x1B[2J");
            print!("\x1B[1;1H");

            println!("Player 1, enter a secret number (it will be hidden): ****");
            
            num
        }
        _ => {
            println!("Invalid input. Using 42 as the secret number.");
            42
        }
    }
}

/// Prompts Player 2 to enter a guess. Ensures valid input between 1 and 100.
/// Recursively asks for input until a valid number is entered.
pub fn get_guess() -> u32 {
    let mut input = String::new();
    
    print!("Enter your guess: ");
    
    // Ensure the prompt is displayed before user input
    io::Write::flush(&mut io::stdout()).expect("Failed to flush");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Parse input and validate range (1 to 100)
    match input.trim().parse::<u32>() {
        Ok(num) if num > 0 && num <= 100 => {
            num
        }
        _ => {
            println!("Please enter a valid number between 1 and 100!");

             // Recursively ask for valid input
            get_guess()
        }
    }
}