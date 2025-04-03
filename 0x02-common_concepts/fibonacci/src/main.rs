//! # Fibonacci Generator
//! 
//! This program generates Fibonacci numbers using both recursive and iterative approaches.
//! It provides a command-line interface for generating specific Fibonacci numbers,
//! sequences, and comparing performance between the two approaches.

use std::io::{self, Write};  // Fixed `std:io` to `std::io`
use std::time::Instant;      // Fixed `instant` to `Instant`

/// main function that displays a menu and handles user interaction
fn main() {
    println!("Fibonacci Generator\n");

    loop {
        print_menu();  // Fixed `print_menue()` to `print_menu()`
        let choice = get_user_choice();

        match choice {
            1 => generate_nth_fibonacci(),  // Fixed `generate_nth_fibbonacci()`
            2 => generate_fibonacci_sequence(),
            3 => compare_performance(),
            4 => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please try again"),  // Fixed spacing
        }
        println!();
    }
}

fn print_menu() {  // Fixed `print_menue` to `print_menu`
    println!("Enter a command:");
    println!("1. Generate nth Fibonacci number");
    println!("2. Generate Fibonacci sequence");
    println!("3. Compare performance");
    println!("4. Exit");
    print!("\nChoice: ");

    // Flush to ensure the prompt appears before waiting for input
    io::stdout().flush().unwrap();
}

/// Gets user choice from standard input
/// 
/// Returns a u32 representing the menu choice, or 0 if input is invalid
fn get_user_choice() -> u32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input from the user");

    if let Ok(num) = input.trim().parse::<u32>() {  // Fixed `input.parse()` to `input.trim().parse::<u32>()`
        return num;  // Fixed `return 0` inside if block
    }
    0  // Added missing return value for invalid input
}

/// Gets a positive integer input from the user
/// 
/// # Arguments
/// * `prompt` - The text to display as a prompt
/// 
/// # Returns
/// * `Result<u32, String>` - The parsed number or an error message
fn get_user_input(prompt: &str) -> Result<u32, String> {
    print!("{}", prompt);

    io::stdout()
        .flush()
        .unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|_| "Failed to read input".to_string())?;
    
    input.trim().parse::<u32>()
    .map_err(|_| "Please enter a valid positive number".to_string())
}

/// Calculates the nth Fibonacci number using recursion
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence (0-indexed)
/// 
/// # Returns
/// * The nth Fibonacci number
fn fibonacci_recursive(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)  // Removed unnecessary return
}

/// Calculates the nth Fibonacci number using iteration 
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence (0-indexed)
/// 
/// # Returns
/// * The nth Fibonacci number
fn fibonacci_iterative(n: u32) -> u64 {  // Fixed `fibonacci iterative` to `fibonacci_iterative`
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut temp: u64;

    for _ in 2..=n {
        temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// Handles the generation of a specific Fibonacci number
fn generate_nth_fibonacci() {  // Fixed `generate_nth_fibbonacci()` to `generate_nth_fibonacci()`
    match get_user_input("Enter n: ") {
        Ok(n) => {
            let result = fibonacci_iterative(n);  // Fixed comment typo: "arroach" → "approach"
            println!("The {}th Fibonacci number is: {}", n, result);
        }
        Err(e) => println!("Error: {}", e),
    }
}

/// Generates and displays a Fibonacci sequence up to the specified count
fn generate_fibonacci_sequence() {
    match get_user_input("How many numbers to generate? ") {
        Ok(count) => {  // Fixed `ok(count)` to `Ok(count)`
            if count == 0 {
                println!("Fibonacci sequence: ");
                return;
            }

            print!("Fibonacci sequence: ");
            for i in 0..count {
                print!("{}", fibonacci_iterative(i));
                if i < count - 1 {  // Fixed spacing in condition
                    print!(", ");
                }
            }
            println!();
        },
        Err(e) => println!("Error: {}", e),
    }
}

/// Compares the performance of recursive and iterative Fibonacci implementations
fn compare_performance() {
    let n = 30;
    println!("Calculating fibonacci({}):", n);

    // Measure recursive approach performance
    let start = Instant::now();  // Fixed `instant::now()` to `Instant::now()`
    let _ = fibonacci_recursive(n);
    let recursive_duration = start.elapsed().as_nanos();

    // Measure iterative approach performance
    let start = Instant::now();
    let _ = fibonacci_iterative(n);
    let iterative_duration = start.elapsed().as_nanos();
    
    println!("Recursive: {} ns", recursive_duration);
    println!("Iterative: {} ns", iterative_duration);

    if iterative_duration > 0 {
        let speedup = recursive_duration / iterative_duration;
        println!("Iterative is {} times faster!", speedup);
    } else {
        println!("Iterative is significantly faster (too fast to measure precisely)!");
    }
}   
