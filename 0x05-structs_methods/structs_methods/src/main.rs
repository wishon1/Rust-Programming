mod rectangle;
mod point;

use point::Point3d;
use rectangle::Rectangle;

/// main function to display the rectangle
fn main() {
    let mut rect = Rectangle::new(30, 50);
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

    // task 4
    let square = Rectangle::square(25);
    println!("{}", square);
    println!("Area: {}", square.area());
    
    // task 5
    let point = Point3d(3.0, 4.0, 5.0);
    println!("Point: {:?}", point);
    println!("Distance from origin: {:.2}", point.distance_from_origin());

    // task 6
    println!("Before resize: {}", rect);
    rect.resize(40, 60);
    println!("After resize: {}", rect);
    rect.resize(0, 10);
    println!("After invalid resize: {}", rect);
}
