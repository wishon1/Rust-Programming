// 0x06. Rust - Enums and Pattern Matching
mod temperature;
use temperature::{Temperature, describe_temperature};

fn main() {
    let temperatures = vec![
        Temperature::Celsius(-10.0),
        Temperature::Celsius(15.0),
        Temperature::Celsius(40.0),
        Temperature::Fahrenheit(32.0),
        Temperature::Fahrenheit(68.0),
        Temperature::Fahrenheit(212.0),
    ];
    
    for temp in temperatures {
        describe_temperature(temp);
    }
}