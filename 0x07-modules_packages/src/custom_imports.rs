// 0x07. Rust - Packages, Crates, and Modules

/// Module demonstrating custom imports with the `as` keyword
pub mod custom_imports {
    pub mod math {
        /// Add two numbers
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        /// Calculate the area of a rectangle
        pub fn area(width: i32, height: i32) -> i32 {
            width * height
        }
    }
    
    pub mod geometry {
        /// Calculate the area of a rectangle
        pub fn area(width: f64, height: f64) -> f64 {
            width * height
        }
        
        /// Calculate the area of a circle
        pub fn circle_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }
    
    pub mod utils {
        /// Format a number with two decimal places
        pub fn format(value: f64) -> String {
            format!("{:.2}", value)
        }
        
        pub mod helper {
            /// Helper function to check if a number is positive
            pub fn is_positive(value: i32) -> bool {
                value > 0
            }
            
            /// Helper function to get absolute value
            pub fn absolute(value: i32) -> i32 {
                if value < 0 {
                    -value
                } else {
                    value
                }
            }
        }
    }
    
    // Module using different path techniques
    pub mod demo {
        // Change these lines:
        use super::math::area as math_area;
        use super::geometry::area as geom_area;
        
        // Using self to reference the current module
        use self::internal::helper as self_helper;
        
        // Using super to reference the parent module
        use super::utils::helper::is_positive;
        
        // Change this line:
        use super::utils::format;
        
        mod internal {
            pub mod helper {
                pub fn say_hello() -> String {
                    "Hello from internal helper".to_string()
                }
            }
        }
        
        /// Demonstrates various import techniques
        pub fn demonstrate() {
            // Using renamed imports to avoid conflicts
            println!("Math area: {}", math_area(5, 10));
            println!("Geometry area: {}", geom_area(5.0, 10.0));
            
            // Using self reference
            println!("{}", self_helper::say_hello());
            
            // Using super reference
            println!("Is 42 positive? {}", is_positive(42));
            
            // Using absolute path with crate
            println!("Formatted value: {}", format(3.14159));
        }
    }
}