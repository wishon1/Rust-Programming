// 0x06. Rust - Enums and Pattern Matching

/// The Message enum represents different types of messages that can be sent in a system
#[derive(Debug)]
pub enum Message {
  /// Simple quit message with no data
  Quit,
  /// Move message with x and y coordinates
  Move { x: i32, y: i32 },
  /// Write message containing a string
  Write(String),
  /// ChangeColor message with RGB values
  ChangeColor(i32, i32, i32),
}

impl Message {
    /// Prints a different message based on the variant
    pub fn call(&self) {
        // Match on self to handle each variant differently
        match self {
            Message::Quit => println!("Quit message recieved"),
            Message::Move { x, y } =>  println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}