// 0x07. Rust - Packages, Crates, and Modules

/// Module demonstrating deeply nested module structure
pub mod nested_modules {
    /// Level 1 module
    pub mod level1 {
        /// Function in level 1
        pub fn function1() -> String {
            "Function in level 1".to_string()
        }
        
        /// Level 2 module
        pub mod level2 {
            /// Function in level 2
            pub fn function2() -> String {
                "Function in level 2".to_string()
            }
            
            /// Calls parent function using super
            pub fn call_parent() -> String {
                format!("Level 2 calling parent: {}", super::function1())
            }
            
            /// Level 3 module
            pub mod level3 {
                /// Function in level 3
                pub fn function3() -> String {
                    "Function in level 3".to_string()
                }
                
                /// Calls grandparent function using super::super
                pub fn call_grandparent() -> String {
                    format!("Level 3 calling grandparent: {}", super::super::function1())
                }
                
                /// Level 4 module
                pub mod level4 {
                    /// Function in level 4
                    pub fn function4() -> String {
                        "Function in level 4".to_string()
                    }
                    
                    /// Calls function from level 1 using absolute path with crate
                    pub fn call_level1_absolute() -> String {
                        format!("Level 4 calling level 1 (absolute): {}", 
                                crate::nested_modules::nested_modules::level1::function1())
                    }
                    
                    /// Calls function from level 2 using relative path with super::super
                    pub fn call_level2_relative() -> String {
                        format!("Level 4 calling level 2 (relative): {}", 
                                super::super::function2())
                    }
                    
                    /// Calls function from level 3 using super
                    pub fn call_level3_relative() -> String {
                        format!("Level 4 calling level 3 (relative): {}", 
                                super::function3())
                    }
                }
            }
        }
    }
    
    /// Utility function that demonstrates accessing deeply nested modules
    pub fn access_all_levels() {
        // Using absolute paths
        println!("Absolute paths:");
        println!("- Level 1: {}", level1::function1());
        println!("- Level 2: {}", level1::level2::function2());
        println!("- Level 3: {}", level1::level2::level3::function3());
        println!("- Level 4: {}", level1::level2::level3::level4::function4());
        
        // Demonstrating super and relative paths
        println!("\nRelative paths and super:");
        println!("- Level 2 -> Level 1: {}", level1::level2::call_parent());
        println!("- Level 3 -> Level 1: {}", level1::level2::level3::call_grandparent());
        println!("- Level 4 -> Level 1 (absolute): {}", 
                 level1::level2::level3::level4::call_level1_absolute());
        println!("- Level 4 -> Level 2 (relative): {}", 
                 level1::level2::level3::level4::call_level2_relative());
        println!("- Level 4 -> Level 3 (relative): {}", 
                 level1::level2::level3::level4::call_level3_relative());
    }
}