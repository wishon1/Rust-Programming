/// Returns the first word of a string slice.
fn first_word(s: &str) -> &str {
    // Convert to byte slice for iteration
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // check if the byte is a space
        if item == b' ' {
            // Return the slice from the start to the space index
            return &s[0..i];
        }
    }
    // If no space was found, return the whole string slice
    &s[..]
}

/// Returns the nth word (0-indexed) of a string slice as an Option<&str>.
fn nth_word(s: &str, n: usize) -> Option<&str> {
    s.split_whitespace().nth(n)
}

/// Safely gets a slice from an array, returning a Result.
fn get_slice_result(arr: &[i32], range: std::ops::Range<usize>) -> Result<&[i32], &'static str> {
    arr.get(range).ok_or("index out of bounds") // Use get for safe access
}

fn main() {
    // String slices
    let my_string = String::from("Hello world from Rust");
    println!("String: \"{}\"", my_string);
    println!("First word: \"{}\"", first_word(&my_string));
    println!("Second word: \"{}\"", nth_word(&my_string, 1).unwrap_or("None"));
    println!("Third word: \"{}\"", nth_word(&my_string, 2).unwrap_or("None"));
    println!("Out of bounds word: {:?}", nth_word(&my_string, 10)); // Use {:?} for Option

    // Array slices
    let a = [1, 2, 3, 4, 5];
    println!("\nArray: {:?}", a);
    println!("First 3 elements: {:?}", &a[0..3]);
    println!("Last 2 elements: {:?}", &a[3..]);

    // Mutable slices
    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("\nMutable slice demonstration:");
    println!("Original array: {:?}", mutable_array);
    let mutable_slice = &mut mutable_array[1..3];
    mutable_slice[0] = 20;
    mutable_slice[1] = 30;
    println!("After modifying slice: {:?}", mutable_array);

    // Bounds checking
    println!("\nBounds checking:");
    println!("Valid slice: {:?}", mutable_array.get(0..3).unwrap()); // Using get
    match get_slice_result(&mutable_array, 6..10) { // Using custom function
         Ok(slice) => println!("Invalid slice (should not happen): {:?}", slice),
         Err(e) => println!("Invalid slice: {} (handled with Result)", e),
    }
}