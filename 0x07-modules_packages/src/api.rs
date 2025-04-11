// 0x07. Rust - Packages, Crates, and Modules

/// API module that re-exports selected internal functions
pub mod api {
    // Re-export validation functions
    pub use crate::internal_modules::validation::validate_email;
    
    // Re-export user management functions
    pub use crate::internal_modules::user_management::{
        create_user,
        delete_user,
    };
    
    // Re-export authentication functions
    pub use crate::internal_modules::authentication::{
        login,
        logout,
    };
    
    /// User management API submodule that flattens the hierarchy
    pub mod user {
        // Re-export all user-related functionality under one module
        pub use crate::internal_modules::validation::*;
        pub use crate::internal_modules::user_management::*;
        pub use crate::internal_modules::authentication::*;
    }
}