use std::time::{Instant};
use crate::player;

/// Plays a round of the guessing game.
/// Player 1 enters a secret number, and Player 2 tries to guess it.
pub fn play_round() {
    // Get secret number from Player 1
    let secret_number = player::get_secret_number();

    println!("Player 2, try to guess the number between 1 and 100");

    // Start timer for scoring
    let start_time = Instant::now();

    // Player 2 guessing loop
    let mut guessed_count = 0;
    loop {
        guessed_count += 1;

        // Get Player 2's guess
        let guess = player::get_guess();

        // Check if correct
        if guess == secret_number {
            println!("You win! The secret number was {}", secret_number);
            break;
        }

        // Give hint based on how close the guess is
        let hint = get_hint(guess, secret_number);
        println!("{}", hint);
    }

    // Calculate and display score
    let elapsed = start_time.elapsed().as_secs();
    let penalty = (guessed_count * 30) as u32 + (elapsed as u32 * 10);
    let score = 1000u32.saturating_sub(penalty);
    println!("Score: {} points", score);
}

/// Determines the hint based on how close the guess is to the secret number.
/// Returns a string indicating the level of proximity.
fn get_hint(guess: u32, secret: u32) -> &'static str {
    let difference = if guess > secret {
        guess - secret
    } else {
        secret - guess
    };
    
    if difference <= 2 {
        "Very hot!"
    } else if difference <= 5 {
        "Hot!"
    } else if difference <= 15 {
        "Warm!"
    } else if difference <= 30 {
        "Cold."
    } else {
        "Very cold."
    }
}