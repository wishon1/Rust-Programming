use std::io;

///main function that runs the temperature converter program.
fn main() {
    println!("Temperature Converter");

    loop{
        let temp = get_input("Enter temperature: ");

        // Get unit (F or C) from user
        let unit = get_unit();

        // perform conversion based on unit
        if unit == 'F' {
            let celsuis = (temp - 32) * 5 / 9;
            println!("{}F is {}C", temp, celsuis);
        } else {
            let fahrenheit = temp * 9 / 5 + 32;
            println!("{}C is {}F", temp, fahrenheit);
        }

        // Ask user if they want to convert another temperature
        let again = get_input("Convert another? (1 = yes, 0 = no): ");
        if again == 0 {
            break;
        }
    }
}

/// Reads a numerical input from the user.
/// 
/// # Arguments: * `prompt` - A string message displayed to the user before input.
/// # Returns: * `i32` - The parsed integer value from user input.
fn get_input(prompt: &str) -> i32 {
    let mut input = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim()
        .parse()
        .expect("Ivalid number")
}

/// Reads and validates the unit of temperature from user input.
/// 
/// # Returns: * `char` - 'F' for Fahrenheit or 'C' for Celsius.
fn get_unit() -> char {
    loop {
        let mut input = String::new();

        println!("Enter unit (F/C): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input user");
        
        let unit = input.trim().to_uppercase();

        if unit == "F" || unit == "C" {
            return unit.chars().next().unwrap();
        }
        println!("Invalid unit. Please enter F or C.");
    }
}