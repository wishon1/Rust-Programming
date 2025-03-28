use colored::*; // Import the `colored` crate for colored terminal output.
use rand::Rng; // Import the `rand` crate for generating random numbers.
use std::cmp::Ordering; // Import `Ordering` for comparing numbers.
use std::io; // Import `io` for handling user input and output.

fn main() {
    loop {
        // Main game loop that allows replaying the game without restarting the program.

        // Display the welcome message and difficulty options.
        println!("{}", "Welcome to the Enhanced Guessing Game!".bold().green());
        println!();
        println!("{}", "Select difficulty:".bold().cyan());
        println!("1. Easy (1-50)");
        println!("2. Medium (1-100)");
        println!("3. Hard (1-200)");

        // Read the user's difficulty choice.
        let mut difficulty_choice = String::new();
        io::stdin()
            .read_line(&mut difficulty_choice)
            .expect("Failed to read line");

        // Determine the range and maximum attempts based on the difficulty choice.
        let (range, max_attempts) = match difficulty_choice.trim() {
            "1" => (1..=50, 10), // Easy: Range 1-50, 10 guesses.
            "2" => (1..=100, 10), // Medium: Range 1-100, 10 guesses.
            "3" => (1..=200, 10), // Hard: Range 1-200, 10 guesses.
            _ => {
                // Handle invalid input.
                println!("{}", "Invalid choice. Please select 1, 2, or 3.".red());
                continue; // Restart the loop for a valid input.
            }
        };

        // Generate a random secret number within the selected range.
        let secret_num = rand::thread_rng().gen_range(range.clone());
        let mut guessed_count = 0; // Initialize the guess counter.

        // Display the game instructions.
        println!(
            "{}",
            format!(
                "Guess the number! (Difficulty: {}-{})",
                range.start(),
                range.end()
            )
            .bold()
            .yellow()
        );
        println!(
            "{}",
            format!("You have {} guesses remaining.", max_attempts).bold().yellow()
        );

        // Loop for handling guesses until the maximum attempts are reached.
        while guessed_count < max_attempts {
            println!("{}", "Please input your guess:".cyan());

            // Read the user's guess.
            let mut guessed_num = String::new();
            io::stdin()
                .read_line(&mut guessed_num)
                .expect("Failed to read line");

            let trimmed_guess = guessed_num.trim();

            // Check if the user wants to exit the game.
            if trimmed_guess.eq_ignore_ascii_case("exit") {
                println!("{}", "Exiting the game...".bold().red());
                return; // Exit the program.
            }

            // Parse the user's guess into a number.
            let guessed_num: u32 = match trimmed_guess.parse() {
                Ok(num) => num, // Valid number.
                Err(_) => {
                    // Handle invalid input.
                    println!("{}", "Invalid input. Please enter a valid number.".red());
                    continue; // Restart the loop for a valid input.
                }
            };

            guessed_count += 1; // Increment the guess counter.
            let remaining_attempts = max_attempts - guessed_count; // Calculate remaining attempts.

            println!("{}", format!("Your guess: {}", guessed_num).bold().blue());

            // Compare the user's guess with the secret number.
            match guessed_num.cmp(&secret_num) {
                Ordering::Less => {
                    // Guess is too small.
                    println!("{}", "Too small!".red());
                    if remaining_attempts > 0 {
                        println!(
                            "{}",
                            format!("You have {} guesses remaining.", remaining_attempts)
                                .bold()
                                .yellow()
                        );
                    }
                }
                Ordering::Greater => {
                    // Guess is too large.
                    println!("{}", "Too large!".red());
                    if remaining_attempts > 0 {
                        println!(
                            "{}",
                            format!("You have {} guesses remaining.", remaining_attempts)
                                .bold()
                                .yellow()
                        );
                    }
                }
                Ordering::Equal => {
                    // Guess is correct.
                    println!(
                        "{}",
                        format!(
                            "You win! It took you {} guesses.",
                            guessed_count
                        )
                        .bold()
                        .green()
                    );
                    break; // Exit the guessing loop.
                }
            }

            // Check if the user has used all attempts.
            if guessed_count == max_attempts {
                println!(
                    "{}",
                    format!(
                        "Game over! You've used all {} guesses. The correct number was {}.",
                        max_attempts, secret_num
                    )
                    .bold()
                    .red()
                );
            }
        }

        // Ask the user if they want to play again.
        println!("{}", "Play again? (y/n):".cyan());
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        // Exit the game if the user chooses not to play again.
        if play_again.trim().eq_ignore_ascii_case("n") {
            println!("{}", "Thanks for playing! Goodbye!".bold().green());
            break; // Exit the main game loop.
        }
    }
}