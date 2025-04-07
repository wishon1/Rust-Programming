// 0x05. Rust - Structs and Methods
mod counter;
mod point;
mod product;
mod point_generic;
mod string_holder;
mod rectangle;

use rectangle::Rectangle;
use counter::{Tick, process_ticks};
use point::Point3d;
use product::Product;
use point_generic::Point as GenericPoint;
use string_holder::StringHolder;

fn main() {
    // Task 0 & 1: Rectangle and area
    let rect = Rectangle::new(30, 50);
    println!("rect is {:?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());
    
    // Task 2: Rectangle validation
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let invalid_rect = Rectangle::new(0, 10);
    println!("Invalid rectangle defaults to: {:?}", invalid_rect);
    
    // Task 3: Display for Rectangle
    println!("{}", rect);
    
    // Task 4: Square from Rectangle
    let square = Rectangle::square(25);
    println!("{}", square);
    println!("Area: {}", square.area());
    
    // Task 5: Tuple structs
    let point = Point3d(3.0, 4.0, 5.0);
    println!("Point: {:?}", point);
    println!("Distance from origin: {:.2}", point.distance_from_origin());
    
    // Task 6: Mutable Rectangle
    let mut rect = Rectangle::new(30, 50);
    println!("Before resize: {}", rect);
    rect.resize(40, 60);
    println!("After resize: {}", rect);
    rect.resize(0, 10);
    println!("After invalid resize: {}", rect);
    
    // Task 7: Unit-like Structs
    let ticks = vec![Tick, Tick, Tick, Tick, Tick];
    let count = process_ticks(&ticks);
    println!("Processed {} ticks.", count);
    
    // Task 8: Structs with Different Data Types
    let product = Product {
        name: String::from("Laptop"),
        price: 1200.50,
        in_stock: true,
    };
    println!("{:?}", product);
    println!("{}", product);
    
    // Task 9: Structs with Generic Types
    let point_float = GenericPoint { x: 3.0, y: 4.0 };
    println!("Point (float): {:?}", point_float);
    println!("Distance (float): {:.2}", point_float.distance_from_origin());
    let point_int = GenericPoint { x: 3, y: 4 };
    println!("Point (int): {:?}", point_int);
    println!("Distance (int): {:.2}", point_int.distance_from_origin());
    
    // Task 10: Structs with Lifetime Annotations
    let text = String::from("Hello, Rust!");
    let holder = StringHolder { text: &text };
    println!("Held string: {}", holder.get_string());
}