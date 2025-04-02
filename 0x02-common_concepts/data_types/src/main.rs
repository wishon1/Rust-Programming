fn main() {
    // Integer types
    println!("Integer types:");
    println!("i8: {} to {}", i8::MIN, i8::MAX);
    println!("u8: {} to {}", u8::MIN, u8::MAX);
    
    let integer_value: i32 = 42;
    println!("Current value (i32): {}", integer_value);
    println!();
    
    // Floating point numbers
    println!("Floating point:");
    let float_value: f32 = 3.14;
    println!("f32 value: {}", float_value);
    
    // Show precision issues (common in floating point)
    let result = 0.1_f32 + 0.2_f32;
    println!("f32 precision example: 0.1 + 0.2 = {}", result);
    println!();
    
    // Boolean type
    println!("Boolean:");
    let bool_value = true;
    println!("bool value: {}", bool_value);
    
    // Logical operations
    let logical_and = true && false;
    let logical_or = true || false;
    println!("Logical AND: {}", logical_and);
    println!("Logical OR: {}", logical_or);
    println!();
    
    // Character type
    println!("Characters:");
    let char_value = 'A';
    println!("Character: {}", char_value);
    
    // Unicode character
    let unicode_char = '☺';
    println!("Unicode character: {}", unicode_char);
    println!();
    
    // Tuple type
    println!("Tuples:");
    let tuple: (i32, f64, bool) = (42, 3.14, true);
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    println!("First value: {}", tuple.0);
    
    // Tuple destructuring
    let (a, b, c) = tuple;
    println!("Destructured: a={}, b={}, c={}", a, b, c);
    println!();
    
    // Array type
    println!("Arrays:");
    let array = [1, 2, 3, 4, 5];
    println!("Array: [{}, {}, {}, {}, {}]", array[0], array[1], array[2], array[3], array[4]);
    println!("Second element: {}", array[1]);
    println!("Length: {}", array.len());
    println!();
    
    // Type conversion (casting)
    println!("Type conversion:");
    
    // Integer to float
    let i = 42;
    let f = i as f64;
    println!("i32 to f64: {} -> {}", i, f);
    
    // Float to integer (truncation)
    let f = 3.14;
    let i = f as i32;
    println!("f64 to i32: {} -> {} (truncated)", f, i);
    
    // Char to integer (ASCII value)
    let c = 'A';
    let i = c as u8;
    println!("Char to u8: '{}' -> {}", c, i);
}