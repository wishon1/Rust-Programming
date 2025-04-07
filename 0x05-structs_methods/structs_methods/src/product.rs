// 0x05. Rust - Structs and Methods
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub in_stock: bool,
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Product(name: {}, price: {}, in_stock: {})", 
               self.name, self.price, self.in_stock)
    }
}