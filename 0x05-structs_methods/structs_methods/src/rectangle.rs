// 0x05. Rust - Structs and Methods
use std::fmt::Debug;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
        /// function that creats a new rectangle struct
        pub fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }

        /// calculate the area of the retcangle
        pub fn area(&self) -> u32 {
            self.width * self.height
        }
}