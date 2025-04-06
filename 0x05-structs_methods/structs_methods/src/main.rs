mod rectangle;
use rectangle::Rectangle;

/// main function to display the rectangle
fn main() {
    let rect = Rectangle::new(30, 50);
    println!("rect is {:?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());

    // task 2
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let invalid_rect = Rectangle::new(0, 10);
    println!("Invalid rectangle defaults to: {:?}", invalid_rect);
    
}