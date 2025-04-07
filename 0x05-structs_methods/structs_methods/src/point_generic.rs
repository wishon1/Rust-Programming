// 0x05. Rust - Structs and Methods
use std::fmt::Debug;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Into<f64> + Copy,
{
    pub fn distance_from_origin(&self) -> f64 {
        let x_f64: f64 = self.x.into();
        let y_f64: f64 = self.y.into();
        (x_f64.powi(2) + y_f64.powi(2)).sqrt()
    }
}