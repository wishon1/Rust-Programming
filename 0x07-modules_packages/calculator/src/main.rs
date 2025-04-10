// src/main.rs
use calculator;

fn main() {
    let operations = vec![
        "add 5 3",
        "subtract 10 4",
        "multiply 3 4",
        "divide 10 2",
        "power 2 3",
        "sqrt 16",
        "divide 5 0",  // Should handle error
    ];
    
    for op in operations {
        match calculator::calculate(op) {
            Ok(result) => println!("{} = {}", op, result),
            Err(e) => println!("{} error: {}", op, e),
        }
    }
}