// 0x05. Rust -struct and methods
use std::fmt::Debug;

#[derive(Debug)]
pub struct Point3d(pub f64, pub f64, pub f64);

impl Point3d {
    pub fn distance_from_origin(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2).sqrt()
    }
}