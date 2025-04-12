// 0x07. Rust - Packages, Crates, and Modules

// Import all three libraries from the workspace
use common_utils::logging;
use common_utils::config;
use front_end::{self, ui, forms};
use back_end::{self, database, api};

fn main() {
    println!("Workspace Integration Demo\n");
    
    // Get application version from config
    let version = config::get_config("version");
    logging::info(&format!("Application version: {}", version));
    
    // Initialize front-end and back-end
    front_end::initialize();
    if back_end::initialize() {
        logging::info("All components initialized successfully");
    } else {
        logging::error("Failed to initialize all components");
        return;
    }
    
    // Use front-end components
    ui::display_message("Hello from the app!");
    let form_result = forms::process_form("user123", "user@example.com");
    logging::info(&format!("Form processing result: {}", form_result));
    
    // Use back-end components
    let query_results = database::query("SELECT * FROM users");
    logging::info(&format!("Query returned {} results", query_results.len()));
    
    let api_response = api::process_request("/users", "id=123");
    logging::info(&format!("API response: {}", api_response));
    
    logging::info("Application shutting down");
}