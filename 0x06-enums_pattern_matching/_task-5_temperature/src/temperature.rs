// 0x06. Rust - Enums and Pattern Matching

/// Represents a temperature in either Celsius or Fahrenheit
#[derive(Debug)]
pub enum Temperature {
    /// Temperature in Celsius
    Celsius(f64),
    /// Temperature in Fahrenheit
    Fahrenheit(f64),
}

/// Describes a temperature based on its value using match guards
///
/// # Arguments
///
/// * `temp` - The Temperature to describe
pub fn describe_temperature(temp: Temperature) {
    // Match on the temperature with guards to categorize it
    match temp {
        // Celsius cases with guards for different temperature ranges
        Temperature::Celsius(c) if c <= 0.0 => {
            println!("Celsius({}) is freezing", c);
        }
        Temperature::Celsius(c) if c < 10.0 => {
            println!("Celsius({}) is cold", c);
        }
        Temperature::Celsius(c) if c < 25.0 => {
            println!("Celsius({}) is moderate", c);
        }
        Temperature::Celsius(c) if c < 35.0 => {
            println!("Celsius({}) is warm", c);
        }
        Temperature::Celsius(c) if c >= 100.0 => {
            println!("Celsius({}) is boiling", c);
        }
        Temperature::Celsius(c) => {
            println!("Celsius({}) is hot", c);
        }
        
        // Fahrenheit cases with guards for different temperature ranges
        Temperature::Fahrenheit(f) if f <= 32.0 => {
            println!("Fahrenheit({}) is freezing", f);
        }
        Temperature::Fahrenheit(f) if f < 50.0 => {
            println!("Fahrenheit({}) is cold", f);
        }
        Temperature::Fahrenheit(f) if f < 77.0 => {
            println!("Fahrenheit({}) is moderate", f);
        }
        Temperature::Fahrenheit(f) if f < 95.0 => {
            println!("Fahrenheit({}) is warm", f);
        }
        Temperature::Fahrenheit(f) if f >= 212.0 => {
            println!("Fahrenheit({}) is boiling", f);
        }
        Temperature::Fahrenheit(f) => {
            println!("Fahrenheit({}) is hot", f);
        }
    }
}