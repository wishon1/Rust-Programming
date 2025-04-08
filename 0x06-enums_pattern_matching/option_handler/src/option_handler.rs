// 0x06. Rust - Enums and Pattern Matching

/// Finds the first occurrence of a value in a vector and returns its index
///
/// # Arguments: 
//  * `numbers` - A slice of integers to search through
/// * `value` - The value to find in the vector
///
/// # Returns
///
/// * `Option<usize>` - The index of the first occurrence of the value, or None if not found
pub fn find_value(numbers: &[i32], value: i32) -> Option<usize> {
    // Iterate through the vector with enumeration to get index and value
    for (index, &number) in numbers.iter().enumerate() {
        if number == value {
            return Some(index);
        }
    }
    // Return None if value is not found
    None
}

// Displays the result of a search operation
///
/// # Arguments
///
/// * `result` - An Option containing the index if found, or None
/// * `value` - The value that was searched for
pub fn display_result(result: Option<usize>, value: i32) {
    // usematch to handle both Some and None cases
    match result {
        Some(index) => println!("Found value {} at index {}", value, index),
        None => println!("Value {} not found in the vector", value),
    }
}