// 0x06. Rust - Enums and Pattern Matching
mod command;
use command::{process_command, parse_command};

fn main() {
    let commands = vec![
        "add 5 3",
        "multiply 2 4",
        "subtract 10 7",
        "divide 15 3",
        "divide 10 0",
        "exit",
    ];
    
    let mut result = 0;
    for cmd_str in commands {
        println!("Command: {}", cmd_str);
        let cmd = parse_command(cmd_str);
        result = process_command(cmd, result);
        println!("Result: {}", result);
    }
}