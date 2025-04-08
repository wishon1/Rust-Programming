// src/main.rs
mod message;
use message::Message;

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in &messages {
        message.call();
    }
}