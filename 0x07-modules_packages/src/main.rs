// src/main.rs
mod privacy_demo;
use restaurant;

fn main() {
    restaurant::customer();

    // Task 4
    println!();
    println!("TASK 4:");
    println!();
    // Accessing public functions
    println!("1. {}", privacy_demo::privacy_demo::public_function());
    println!("2. {}", privacy_demo::privacy_demo::public_interface());
     
    // Accessing nested public functions
    println!("3. {}", privacy_demo::privacy_demo::nested::nested_public());
    println!("4. {}", privacy_demo::privacy_demo::nested::call_parent());
     
    // Working with a struct that has private fields
    let mut person = privacy_demo::privacy_demo::Person::new("Alice".to_string(), 30);
     
    // We can access public fields directly
    println!("5. Person's name: {}", person.name);
     
    // We cannot access private fields directly
    // Uncommenting the line below would cause a compilation error:
    // println!("Person's age: {}", person.age);
     
    // But we can use public methods to access and manipulate private data
    println!("6. Person's age: {}", person.get_age());
    person.have_birthday();
    println!("7. Person's age after birthday: {}", person.get_age());
}