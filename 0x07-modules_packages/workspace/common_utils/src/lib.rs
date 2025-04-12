// 0x07. Rust - Packages, Crates, and Modules

/// Common utilities shared between front_end and back_end

/// Logging module for application-wide logging
pub mod logging {
    /// Log an info message
    pub fn info(message: &str) {
        println!("[INFO] {}", message);
    }

    /// Log an error message
    pub fn error(message: &str) {
        println!("[ERROR] {}", message);
    }
}

/// Data validation module
pub mod validation {
    /// Validate a user input
    pub fn validate_input(input: &str) -> bool {
        !input.is_empty()
    }

    /// Sanitize user input
    pub fn sanitize(input: &str) -> String {
        // Simple sanitization for demonstration
        input.trim().to_string()
    }
}

/// Configuration module
pub mod config {
    /// Get a configuration value by key
    pub fn get_config(key: &str) -> String {
        // Simple mock implementation
        match key {
            "version" => "1.0.0".to_string(),
            "api_url" => "https://api.example.com".to_string(),
            _ => "unknown".to_string(),
        }
    }
}