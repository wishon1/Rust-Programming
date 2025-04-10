// src/main.rs
use calculator::shortcuts::{add, subtract, multiply};
fn main() {
    println!("5 + 3 = {}", add(5.0, 3.0));
    println!("10 - 4 = {}", subtract(10.0, 4.0));
    println!("3 * 4 = {}", multiply(3.0, 4.0));
}