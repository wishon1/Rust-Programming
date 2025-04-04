/// Calculates the length of a borrowed String.
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Returns a new String that is the reverse of the borrowed String.
fn reverse_string(s: &String) -> String {
    s.chars().rev().collect::<String>()
}

/// Concatenates two string slices into a new String.
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from(s1);
    result.push_str(s2);
    result
}

/// Returns a slice containing the first word of the given string slice.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let main_string = String::from("Hello, Rust!");
    println!("Original string: {}", main_string);

    let len = calculate_length(&main_string);
    println!("Length: {}", len);

    let reversed = reverse_string(&main_string);
    println!("Reversed: {}", reversed);

    let s1 = "Hello, Rust!";
    let s2 = " Welcome to ALX!";
    let concatenated = concatenate_strings(s1, s2);
    println!("Concatenated: {}", concatenated);

    // Storing Strings in a Vector
    println!("\nString storage in vector:");
    let mut string_vec: Vec<String> = Vec::new();
    string_vec.push(String::from("Hello"));
    string_vec.push(String::from("World"));
    string_vec.push(String::from("Rust"));

    for (i, s) in string_vec.iter().enumerate() {
        println!("{}. {}", i + 1, s);
    }

    // Using string slices
    println!("\nSlice manipulation:");
    let sentence = String::from("Hello World Rust");
    let first = first_word(&sentence);
    println!("First word: {}", first);

    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("All words: {:?}", words);
}