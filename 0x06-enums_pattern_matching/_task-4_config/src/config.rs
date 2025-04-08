// 0x06. Rust - Enums and Pattern Matching

/// Represents different types of server configuration options
#[derive(Debug)]
pub enum ServerConfig {
    /// IP address configuration
    IP(String),
    /// Port number configuration
    Port(u16),
    /// Maximum connections configuration
    MaxConnections(u32),
}

/// Applies a server configuration using if let expressions
/// # Arguments
/// * `config` - The ServerConfig to apply
pub fn apply_config(config: ServerConfig) {
    // Use if let to handle each variant separately
    // This is more concise than match when we only care about one variant at a time
    if let ServerConfig::IP(ip_address) = config {
        println!("Setting server IP to {}", ip_address);
    } else if let ServerConfig::Port(port_number) = config {
        println!("Setting server port to {}", port_number);
    } else if let ServerConfig::MaxConnections(max_conn) = config {
        println!("Setting maximum connections to {}", max_conn);
    }
    
    // Alternative implementation using match:
    // match config {
    //     ServerConfig::IP(ip_address) => println!("Setting server IP to {}", ip_address),
    //     ServerConfig::Port(port_number) => println!("Setting server port to {}", port_number),
    //     ServerConfig::MaxConnections(max_conn) => println!("Setting maximum connections to {}", max_conn),
    // }
}