// 0x05. Rust - Structs and Methods
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {

        /// function that creats a new rectangle struct
        pub fn new(width: u32, height: u32) -> Rectangle {
            if width == 0 {
                return Rectangle { width: 1, height};
            }
            if height == 0 {
                return Rectangle { width, height: 1};
            }
            Rectangle { width, height }
        }

        /// calculate the area of the retcangle
        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        /// `can_hold` that takes another `Rectangle` as a parameter and returns a boolean
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Rectangle(width: {}, height: {})", self.width, self.height)
    }
}