// 0x06. Rust - Enums and Pattern Matching
mod option_handler;

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    
    for value in [30, 60] {
        let result = option_handler::find_value(&numbers, value);
        option_handler::display_result(result, value);
    }
}