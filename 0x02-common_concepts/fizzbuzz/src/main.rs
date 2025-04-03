//! FizzBuzz Program
//! This program prints numbers from 1 to n, with "Fizz" for multiples of 3,
//! "Buzz" for multiples of 5, and "FizzBuzz" for multiples of both.

use std::env;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the right number of arguments
    if args.len() != 2 {
        println!("Usage: {} <number>", args[0]);
        return;
    }
    
    // Try to parse the argument as a number
    let n = match args[1].parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Please provide a valid positive number");
            return;
        }
    };
    
    // First approach: Using if/else statements
    fizzbuzz_if_else(n);
    
    // Uncomment to use the second approach
    // fizzbuzz_match(n);
}

/// Implements FizzBuzz using if/else statements
/// 
/// # Arguments
/// * `n` - The upper limit of numbers to process
fn fizzbuzz_if_else(n: u32) {
    // Loop through numbers 1 to n
    for i in 1..=n {
        // Check divisibility and print appropriate output
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

/// Implements FizzBuzz using match expressions
/// 
/// # Arguments
/// * `n` - The upper limit of numbers to process
fn fizzbuzz_match(n: u32) {
    // Loop through numbers 1 to n
    for i in 1..=n {
        // Create a tuple of booleans that represent divisibility by 3 and 5
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", i),
        }
    }
}