// 0x07. Rust - Packages, Crates, and Modules

/// Module demonstrating Rust's privacy rules
pub mod privacy_demo {
    // This is a private function in the privacy_demo module
    // It can only be accessed within this module and its child modules
    fn internal_function() -> String {
        "This is an internal function".to_string()
    }

    // This is a public function that can be accessed from outside the module
    pub fn public_function() -> String {
        "This is a public function".to_string()
    }

    // This public function accesses a private function
    // Even though internal_function is private, public_interface can use it
    // and expose its functionality in a controlled way
    pub fn public_interface() -> String {
        format!("Public interface calling: {}", internal_function())
    }

    // A public struct with mixed visibility fields
    pub struct Person {
        // This field is public and can be accessed directly
        pub name: String,
        
        // This field is private and cannot be accessed directly from outside
        age: u32,
    }

    impl Person {
        // Public constructor
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }

        // Public method that provides access to private data
        pub fn get_age(&self) -> u32 {
            // We can access private fields within the implementation
            self.age
        }

        // Public method that manipulates private data
        pub fn have_birthday(&mut self) {
            // We can modify private fields within the implementation
            self.age += 1;
        }
    }

    // Nested module with its own privacy rules
    pub mod nested {
        // This function is public within nested, but since nested is public,
        // it can be accessed as privacy_demo::nested::nested_public
        pub fn nested_public() -> String {
            "This is a public function in a nested module".to_string()
        }

        // This is private to the nested module
        fn nested_private() -> String {
            "This is private to the nested module".to_string()
        }

        // Public function that uses super to access parent module items
        pub fn call_parent() -> String {
            // super refers to the parent module (privacy_demo)
            format!("Nested module calling: {}", super::internal_function())
        }
    }
}