# 0x05. Rust - Structs and Methods

## Resources
Read or watch:
- [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
- [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [The Rust Programming Language - Chapter 5](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust By Example - Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:

### General
- Why Rust programming is awesome
- What are structs and how to define them
- How to create an instance of a struct
- How to access fields of a struct
- What are methods and how to implement them
- When to use the `self` keyword
- How to use tuple structs
- What is the unit-like struct and its purpose
- How to use derived traits like Debug
- How to implement the display trait
- How to create methods that modify instances (mutable self)
- How to create associated functions (like constructors)

## Requirements
- Allowed editors: vim, emacs, Visual Studio Code
- All your files will be compiled on Ubuntu 20.04 LTS using rustc 1.68.0
- All your files should end with a new line
- A README.md file at the root of the folder of the project is mandatory
- Your code should use the rustfmt style (install using `rustup component add rustfmt`)
- All your Rust source files must pass `cargo clippy` without warnings
- The first line of all your files should be `// 0x05. Rust - Structs and Methods`
- All your projects should have proper documentation
- Documentation is more than a simple comment; it's a real sentence explaining the purpose of the function, struct, or method

## Tasks

### 0. My first rectangle
**Mandatory**
Create a file `src/rectangle.rs` that defines a struct `Rectangle` with width and height fields:

- The struct should have two fields: `width` and `height`, both of type `u32`
- You must implement the `Debug` trait for the struct
- Create a `new` associated function that creates a new rectangle
- Test your implementation with the following main function:

```rust
// src/main.rs
use rectangle::Rectangle;

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("rect is {:?}", rect);
}
```

Expected output:
```
rect is Rectangle { width: 30, height: 50 }
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/rectangle.rs, src/main.rs, Cargo.toml`

### 1. Rectangle with area
**Mandatory**
Building upon the previous task, add an `area` method to the `Rectangle` struct:

- Implement an `area` method that returns the area of the rectangle
- The method should be implemented for the `Rectangle` struct
- Use `self` as the first parameter of the method
- Test your implementation with the following main function:

```rust
// src/main.rs
use rectangle::Rectangle;

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("The area of the rectangle is {} square pixels.", rect.area());
}
```

Expected output:
```
The area of the rectangle is 1500 square pixels.
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/rectangle.rs, src/main.rs, Cargo.toml`

### 2. Rectangle validation
**Mandatory**
Modify your `Rectangle` struct to validate inputs:

- Modify the `new` associated function to ensure width and height are greater than 0
- If either dimension is 0, return a default rectangle with width and height of 1
- Implement a method `can_hold` that takes another `Rectangle` as a parameter and returns a boolean
- The `can_hold` method should return `true` if the rectangle can contain the other rectangle
- Test your implementation with the following main function:

```rust
// src/main.rs
use rectangle::Rectangle;

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let invalid_rect = Rectangle::new(0, 10);
    println!("Invalid rectangle defaults to: {:?}", invalid_rect);
}
```

Expected output:
```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
Invalid rectangle defaults to: Rectangle { width: 1, height: 10 }
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/rectangle.rs, src/main.rs, Cargo.toml`

### 3. Display for Rectangle
**Mandatory**
Implement the `Display` trait for the `Rectangle` struct:

- Import the `fmt` module from the standard library
- Implement the `Display` trait for `Rectangle`
- The display format should be: `Rectangle(width: {}, height: {})`
- Test your implementation with the following main function:

```rust
// src/main.rs
use rectangle::Rectangle;

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("{}", rect);
}
```

Expected output:
```
Rectangle(width: 30, height: 50)
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/rectangle.rs, src/main.rs, Cargo.toml`

### 4. Square from Rectangle
**Mandatory**
Add a method to create a square from your `Rectangle` struct:

- Add a `square` associated function that returns a `Rectangle` with equal width and height
- The function should take one parameter, `size`, which is used for both dimensions
- Test your implementation with the following main function:

```rust
// src/main.rs
use rectangle::Rectangle;

fn main() {
    let square = Rectangle::square(25);
    println!("{}", square);
    println!("Area: {}", square.area());
}
```

Expected output:
```
Rectangle(width: 25, height: 25)
Area: 625
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/rectangle.rs, src/main.rs, Cargo.toml`

### 5. Tuple structs
**Advanced**
Create a file `src/point.rs` that defines a tuple struct for 3D coordinates:

- Define a tuple struct `Point3D` with three fields of type `f64`
- Implement the `Debug` trait for your tuple struct
- Implement a `distance_from_origin` method that returns the Euclidean distance from the origin (0, 0, 0)
- Test your implementation with the following main function:

```rust
// src/main.rs
mod point;
use point::Point3D;

fn main() {
    let point = Point3D(3.0, 4.0, 5.0);
    println!("Point: {:?}", point);
    println!("Distance from origin: {:.2}", point.distance_from_origin());
}
```

Expected output:
```
Point: Point3D(3.0, 4.0, 5.0)
Distance from origin: 7.07
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/point.rs, src/main.rs, Cargo.toml`


### 6. Mutable Rectangle
**Mandatory**
Modify the Rectangle struct to allow for resizing:

- Implement a method `resize` that takes two parameters, `new_width` and `new_height`, both of type `u32`.
- The `resize` method should modify the `width` and `height` fields of the Rectangle struct.
- Ensure that the inputs for `new_width` and `new_height` are greater than 0. If not, don't change the rectangle.
- Test your implementation with the following `main` function:

```rust
// src/main.rs
use rectangle::Rectangle;

fn main() {
    let mut rect = Rectangle::new(30, 50);
    println!("Before resize: {}", rect);
    rect.resize(40, 60);
    println!("After resize: {}", rect);
    rect.resize(0, 10);
    println!("After invalid resize: {}", rect);
}
```

Expected output:

```
Before resize: Rectangle(width: 30, height: 50)
After resize: Rectangle(width: 40, height: 60)
After invalid resize: Rectangle(width: 40, height: 60)
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/rectangle.rs`, `src/main.rs`, `Cargo.toml`

### 7. Unit-like Structs: Counter
**Mandatory**
Create a unit-like struct and implement a method that uses it:

- Create a file `src/counter.rs`
- Define a unit-like struct named `Tick`.
- Implement a function `process_ticks` that takes a slice of `Tick` and returns the number of ticks processed.
- Test your implementation with the following `main` function:

```rust
// src/main.rs
mod counter;
use counter::{Tick, process_ticks};

fn main() {
    let ticks = vec![Tick, Tick, Tick, Tick, Tick];
    let count = process_ticks(&ticks);
    println!("Processed {} ticks.", count);
}
```

Expected output:

```
Processed 5 ticks.
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/counter.rs`, `src/main.rs`, `Cargo.toml`

### 8. Structs with Different Data Types
**Mandatory**
Create a struct with fields of different data types:

- Create a file `src/product.rs`
- Define a struct `Product` with the following fields:
    - `name`: `String`
    - `price`: `f64`
    - `in_stock`: `bool`
- Implement the `Debug` and `Display` traits for the `Product` struct.
- The `Display` trait should format the product as: `Product(name: <name>, price: <price>, in_stock: <in_stock>)`
- Test your implementation with the following `main` function:

```rust
// src/main.rs
mod product;
use product::Product;

fn main() {
    let product = Product {
        name: String::from("Laptop"),
        price: 1200.50,
        in_stock: true,
    };
    println!("{:?}", product);
    println!("{}", product);
}
```

Expected output:

```
Product { name: "Laptop", price: 1200.5, in_stock: true }
Product(name: Laptop, price: 1200.5, in_stock: true)
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/product.rs`, `src/main.rs`, `Cargo.toml`

### 9. Structs with Generic Types
**Advanced**
Create a struct with generic types:

- Create a file `src/point_generic.rs`
- Define a generic struct `Point<T>` with `x` and `y` fields of type `T`.
- Implement a method `distance_from_origin` that calculates the distance from the origin (0, 0) for points where `T` can be converted to `f64`.
- Use trait bounds to ensure `T` supports conversion to `f64`.
- Implement the `Debug` trait for the generic struct.
- Test your implementation with the following `main` function:

```rust
// src/main.rs
mod point_generic;
use point_generic::Point;

fn main() {
    let point_float = Point { x: 3.0, y: 4.0 };
    println!("Point (float): {:?}", point_float);
    println!("Distance (float): {:.2}", point_float.distance_from_origin());

    let point_int = Point { x: 3, y: 4 };
    println!("Point (int): {:?}", point_int);
    println!("Distance (int): {:.2}", point_int.distance_from_origin());
}
```

Expected output:

```
Point (float): Point { x: 3.0, y: 4.0 }
Distance (float): 5.00
Point (int): Point { x: 3, y: 4 }
Distance (int): 5.00
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/point_generic.rs`, `src/main.rs`, `Cargo.toml`

### 10. Structs with Lifetime Annotations
**Advanced**
Create a struct with lifetime annotations:

- Create a file `src/string_holder.rs`
- Define a struct `StringHolder<'a>` that holds a reference to a string slice.
- Implement a method `get_string` that returns the held string slice.
- Ensure that the lifetime of the string slice is correctly annotated.
- Test your implementation with the following `main` function:

```rust
// src/main.rs
mod string_holder;
use string_holder::StringHolder;

fn main() {
    let text = String::from("Hello, Rust!");
    let holder = StringHolder { text: &text };
    println!("Held string: {}", holder.get_string());
}
```

Expected output:

```
Held string: Hello, Rust!
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x05-structs_methods`
- File: `src/string_holder.rs`, `src/main.rs`, `Cargo.toml`
