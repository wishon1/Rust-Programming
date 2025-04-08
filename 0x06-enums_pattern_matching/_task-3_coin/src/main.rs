// 0x06. Rust - Enums and Pattern Matching
mod coin;
use coin::{Coin, State, print_state_quarters, value_in_cents};

fn main() {
    let coins = vec![
        Coin::Penny,
        Coin::Quarter(State::Alabama),
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(State::Alaska),
        Coin::Quarter(State::Arizona),
    ];
    
    println!("Value of Alabama Quarter: {} cents", value_in_cents(Coin::Quarter(State::Alabama)));
    print_state_quarters(&coins);
}