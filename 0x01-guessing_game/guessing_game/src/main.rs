/// A function for a number guessing game
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=10);
    let mut guessed_count = 0;

    loop {
        println!("Please input your guess (0-10) or type 'exit' to quit:");

        let mut guessed_num = String::new();
        io::stdin()
            .read_line(&mut guessed_num)
            .expect("Failed to read line");

        let trimmed_guess = guessed_num.trim();

        // Check if the user wants to exit
        if trimmed_guess == "exit" || trimmed_guess == "Exit" {
            println!("Exiting the game...");
            break;
        }

        let guessed_num: u32 = match trimmed_guess.parse() {
            Ok(num) => num,
            // Skip invalid input
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 10.");
                continue;
            }
        };

        println!("You guessed: {}", guessed_num);
        guessed_count += 1;

        match guessed_num.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took you {} attempts.", guessed_count);
                break;
            }
        }
    }
}
