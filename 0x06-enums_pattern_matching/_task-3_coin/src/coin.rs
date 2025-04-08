// 0x06. Rust - Enums and Pattern Matching

/// Represents different US states for quarters
#[derive(Debug, Clone)]
pub enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

/// Represents different types of US coins
#[derive(Debug, Clone)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

/// Returns the value of a coin in cents
/// # Arguments
///     * `coin` - The coin to get the value of
/// # Returns
/// * `u32` - The value of the coin in cents
pub fn value_in_cents(coin: Coin) -> u32 {
    // Match on the coin to return its value
    // Quarter value is the same regardless of state
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

/// Prints information about state quarters in a collection
/// # Arguments
///     * `coins` - A slice of Coin enums
pub fn print_state_quarters(coins: &[Coin]) {
    println!("State quarters found:");
    
    // Filter for quarters and print their state
    for coin in coins {
        // If it's a quarter, print the state information
        if let Coin::Quarter(state) = coin {
            match state {
                State::Alabama => println!("Alabama quarter"),
                State::Alaska => println!("Alaska quarter"),
                State::Arizona => println!("Arizona quarter"),
                State::Arkansas => println!("Arkansas quarter"),
                State::California => println!("California quarter"),
            }
        }
    }
}