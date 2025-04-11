// 0x07. Rust - Packages, Crates, and Modules

/// Internal module for validation functions
pub mod validation {
    /// Validates an email format
    pub fn validate_email(email: &str) -> bool {
        // Simple validation for demonstration
        email.contains('@')
    }
    
    /// Validates a password strength
    pub fn validate_password(password: &str) -> bool {
        // Simple validation for demonstration
        password.len() >= 8
    }
}

/// Internal module for user management
pub mod user_management {
    /// Creates a new user
    pub fn create_user(username: &str, email: &str) -> String {
        format!("User created: {} ({})", username, email)
    }
    
    /// Deletes a user
    pub fn delete_user(username: &str) -> String {
        format!("User deleted: {}", username)
    }
}

/// Internal module for authentication
pub mod authentication {
    /// Logs in a user
    pub fn login(username: &str, password: &str) -> String {
        format!("User logged in: {}", username)
    }
    
    /// Logs out a user
    pub fn logout(username: &str) -> String {
        format!("User logged out: {}", username)
    }
}