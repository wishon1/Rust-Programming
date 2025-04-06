
# 0x08. Rust - Common Collections

## Resources
Read or watch:
- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust by Example - Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)
- [Rust by Example - Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
- [Rust by Example - HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:

### General
- How to store lists of values with vectors
- How to create and manipulate vectors
- When to use vectors vs arrays
- How to store UTF-8 encoded text with strings
- The differences between `String` and string slices (`&str`)
- How to store keys with associated values in hash maps
- How to create and update hash maps
- How to decide which collection to use for different scenarios
- How to iterate over collections
- How to handle common operations on collections

## Requirements
- Allowed editors: vim, emacs, Visual Studio Code
- All your files will be compiled on Ubuntu 20.04 LTS using rustc 1.68.0
- All your files should end with a new line
- A README.md file at the root of the folder of the project is mandatory
- Your code should use the rustfmt style
- All your Rust source files must pass `cargo clippy` without warnings
- The first line of all your files should be `// 0x08. Rust - Common Collections`

## Tasks

### 0. Vector basics
**Mandatory**
Create a file that demonstrates the basics of vectors:

- Create functions for creating, populating, and accessing vectors
- Implement a function `create_vector` that returns a vector with specified elements
- Implement a function `get_element` that safely retrieves an element at an index
- Implement a function `modify_vector` that modifies vector elements
- Test with the following main function:

```rust
// src/main.rs
mod vectors;

fn main() {
    // Create a vector with values 1-5
    let mut vec = vectors::create_vector(5);
    println!("Vector: {:?}", vec);
    
    // Access elements
    println!("Element at index 2: {:?}", vectors::get_element(&vec, 2));
    println!("Element at index 10: {:?}", vectors::get_element(&vec, 10));
    
    // Modify vector
    vectors::modify_vector(&mut vec, 0, 10);
    println!("Modified vector: {:?}", vec);
}
```

Expected output:
```
Vector: [1, 2, 3, 4, 5]
Element at index 2: Some(3)
Element at index 10: None
Modified vector: [10, 2, 3, 4, 5]
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections`
- File: `src/vectors.rs, src/main.rs, Cargo.toml`

### 1. Vector operations
**Mandatory**
Create functions that perform various operations on vectors:

- Implement a function `push_pop_demo` that demonstrates push and pop operations
- Implement a function `iterate_vector` that uses different loops to iterate a vector
- Implement a function `vector_slice` that works with slices of vectors
- Implement a function `vector_of_different_types` that demonstrates using enums to store different types
- Test with a main function that shows all these operations

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections`
- File: `src/vector_ops.rs, src/main.rs, Cargo.toml`

### 2. String basics
**Mandatory**
Create functions that demonstrate string operations:

- Implement a function `create_string` that demonstrates different ways to create strings
- Implement a function `string_concatenation` that shows how to concatenate strings
- Implement a function `string_slicing` that demonstrates string slices
- Implement a function `compare_strings` that compares strings and string slices
- Test with a main function showing all these operations

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections`
- File: `src/strings.rs, src/main.rs, Cargo.toml`

### 3. String manipulation
**Mandatory**
Create functions for complex string manipulations:

- Implement a function `string_indexing` that safely gets characters by index
- Implement a function `string_iteration` that iterates through characters and bytes
- Implement a function `string_words` that splits a string into words
- Implement a function `string_transformation` that converts case and trims whitespace
- Test with a main function showing all these operations

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections`
- File: `src/string_manip.rs, src/main.rs, Cargo.toml`

### 4. HashMap basics
**Mandatory**
Create functions that demonstrate HashMap basics:

- Implement a function `create_hashmap` that creates and populates a HashMap
- Implement a function `access_hashmap` that safely gets values by key
- Implement a function `update_hashmap` that updates or inserts values
- Implement a function `remove_from_hashmap` that removes key-value pairs
- Test with a main function showing all these operations

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections`
- File: `src/hashmaps.rs, src/main.rs, Cargo.toml`

### 5. HashMap operations
**Mandatory**
Create functions for complex HashMap operations:

- Implement a function `merge_hashmaps` that combines two hashmaps
- Implement a function `hashmap_iteration` that iterates keys and values
- Implement a function `count_words` that counts word occurrences in a text
- Implement a function `group_by` that groups values by a key function
- Test with a main function showing all these operations

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections`
- File: `src/hashmap_ops.rs, src/main.rs, Cargo.toml`

### 6. Student management system
**Mandatory**
Create a simple student management system using collections:

- Define a `Student` struct with name, ID, and grades (vector)
- Implement methods to add grades and calculate average
- Create a `School` struct that stores students in a HashMap
- Implement methods to add students, record grades, and get statistics
- Test with a main function demonstrating the system

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x08-common_collections