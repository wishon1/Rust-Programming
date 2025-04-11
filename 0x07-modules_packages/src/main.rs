// src/main.rs
mod privacy_demo;
mod custom_imports;
mod nested_modules;
mod internal_modules;
mod api;

// Import functions with custom names using `as`
use custom_imports::custom_imports::math::add;
use custom_imports::custom_imports::geometry::area as geometry_area;
use custom_imports::custom_imports::utils::format as format_number;
use custom_imports::custom_imports::utils::helper::absolute;

// Using absolute path to import access_all_levels
use nested_modules::nested_modules::access_all_levels;

// Using absolute path with specific imports
use nested_modules::nested_modules::level1;
use nested_modules::nested_modules::level1::level2::level3::level4;

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

    println!();
    println!("Custom imports example:");
    
    // Using imported functions with their original or renamed identifiers
    println!("Addition: {} + {} = {}", 5, 10, add(5, 10));
    println!("Geometry area: {} x {} = {}", 7.5, 3.2, geometry_area(7.5, 3.2));
    println!("Formatted PI: {}", format_number(std::f64::consts::PI));
    println!("Absolute value of -42: {}", absolute(-42));
    
    // Using the demonstration function
    println!("\nDemonstration of various import techniques:");
    custom_imports::custom_imports::demo::demonstrate();


    println!("Nested Modules Example\n");

    // Demonstrate accessing all levels from the utility function
    access_all_levels();
    
    println!("\nDirect access from main:");
    
    // Access modules directly from main with specific imports
    println!("- Level 1 direct: {}", level1::function1());
    println!("- Level 4 direct: {}", level4::function4());
    
    // Access deeply nested function with full path
    println!("- Full path: {}", 
             nested_modules::nested_modules::level1::level2::level3::function3());

     // Task 7: Testing re-exported functions
     println!();
     println!("TASK 7:");
     println!();
     
     // Using re-exported functions
     println!("Email validation: {}", api::api::validate_email("user@example.com"));
     println!("{}", api::api::create_user("johndoe", "john@example.com"));
     println!("{}", api::api::login("johndoe", "password123"));
     println!("{}", api::api::logout("johndoe"));
     
     // Using the flattened API
     println!("\nUsing flattened API:");
     println!("Password validation: {}", api::api::user::validate_password("password123"));
     println!("{}", api::api::user::create_user("janedoe", "jane@example.com"));
     println!("{}", api::api::user::delete_user("janedoe"));
}