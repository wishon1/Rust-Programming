mod rectangle;
use rectangle::Rectangle;

/// main function to display the rectangle
fn main() {
    let rect = Rectangle::new(30, 50);
    println!("rect is {:?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("The area of the rectangle is {} square pixels.", rect.area());
}