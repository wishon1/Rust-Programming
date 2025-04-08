// 0x06. Rust - Enums and Pattern Matching
use config::{ServerConfig, apply_config};

fn main() {
    let configs = vec![
        ServerConfig::IP(String::from("127.0.0.1")),
        ServerConfig::Port(8080),
        ServerConfig::MaxConnections(100),
        ServerConfig::Port(9000),
    ];
    
    for config in configs {
        apply_config(config);
    }
}