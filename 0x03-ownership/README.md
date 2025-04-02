# 0x03. Rust - Understanding Ownership
**Rust**
* Weight: 1
* Project duration: 3 days

## Concepts
*For this project, we expect you to look at these concepts:*
* Ownership
* Borrowing
* References
* Slices
* Memory management

## Resources
Read or watch:
* The Rust Programming Language - Chapter 4
* [Rust By Example - Ownership and Borrowing](https://doc.rust-lang.org/rust-by-example/scope/move.html)
* [Understanding Ownership in Rust](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch04-00-understanding-ownership.html)

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:
* What ownership is and why Rust has it
* How memory is managed in Rust
* What are references and borrowing
* How to use slices
* How to debug ownership issues

## Tasks

### 0. Ownership Basics
**mandatory**

Write a Rust program that demonstrates the basic concepts of ownership.
* Create a function that takes ownership of a string
* Create another function that takes a reference to a string
* Show how to use `&` and `&mut` references
* Demonstrate the difference between Copy and move types
* Include examples of ownership issues and how to fix them

```
$ cargo run
String before move: Hello
After move: Value moved

String before borrow: Hello
After immutable borrow: Hello
After mutable borrow: Hello, World!

Copy type example:
Original: 5
After function call: 5

Ownership issue example:
This would cause a compilation error if uncommented
Fixed version works correctly
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x03-ownership`
* File: `ownership_basics/src/main.rs`, `ownership_basics/Cargo.toml`

### 1. String Manipulation
**mandatory**

Create a Rust program that demonstrates string manipulation with ownership in mind.
* Implement a function that takes a string and returns its length
* Implement a function that takes a string and returns it reversed
* Build a function that concatenates two strings without taking ownership
* Show how to properly handle string ownership when storing strings in a vector
* Demonstrate memory efficiency by using string slices where appropriate

```
$ cargo run
Original string: Hello, Rust!
Length: 12
Reversed: !tsuR ,olleH
Concatenated: Hello, Rust! Welcome to ALX!

String storage in vector:
1. Hello
2. World
3. Rust

Slice manipulation:
First word: Hello
All words: ["Hello", "World", "Rust"]
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x03-ownership`
* File: `string_manipulation/src/main.rs`, `string_manipulation/Cargo.toml`

### 2. The Slice Type
**mandatory**

Write a Rust program focused on slices.
* Create a function that returns the first word of a string as a slice
* Implement a function to get the nth word of a string
* Show how to use slices with different data types (strings, arrays)
* Demonstrate how to create mutable slices
* Include examples of slice bounds checking

```
$ cargo run
String: "Hello world from Rust"
First word: "Hello"
Second word: "world"
Third word: "from"
Out of bounds word: None

Array: [1, 2, 3, 4, 5]
First 3 elements: [1, 2, 3]
Last 2 elements: [4, 5]

Mutable slice demonstration:
Original array: [1, 2, 3, 4, 5]
After modifying slice: [1, 20, 30, 4, 5]

Bounds checking:
Valid slice: [1, 20, 30]
Invalid slice: index out of bounds (handled with Result)
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x03-ownership`
* File: `slice_type/src/main.rs`, `slice_type/Cargo.toml`

### 3. Text Analyzer
**mandatory**

Create a text analyzer program that demonstrates ownership and borrowing.
* The program should count words, characters, and sentences in a text
* It should identify the most common words
* It should calculate the average word length
* Use references and slices appropriately
* Implement the program using proper Rust ownership rules

```
$ cargo run
Enter text: Rust is a systems programming language that is blazingly fast, prevents segfaults, and guarantees thread safety.

Analysis:
Characters: 98
Words: 16
Sentences: 1
Average word length: 5.1
Most common words:
1. is (2 times)
2. that (1 time)
3. a (1 time)
4. systems (1 time)
5. programming (1 time)
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x03-ownership`
* File: `text_analyzer/src/main.rs`, `text_analyzer/Cargo.toml`

### 4. Memory Usage Tracker
**advanced**

Create a program that demonstrates memory usage with different ownership patterns.
* Implement two versions of a data processing function:
  * One that copies data
  * One that uses references
* Measure and compare memory usage
* Show how to avoid unnecessary allocations
* Use the `std::mem` module to report memory usage

```
$ cargo run
Memory usage test:
Processing 100,000 integers:

Copy version:
- Peak memory usage: 1600 KB
- Time taken: 25 ms

Reference version:
- Peak memory usage: 800 KB
- Time taken: 12 ms

Using references reduced memory usage by 50% and improved performance by 52%
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x03-ownership`
* File: `memory_tracker/src/main.rs`, `memory_tracker/Cargo.toml`

### 5. Lifetime Annotations
**advanced**

Write a Rust program that demonstrates lifetime annotations.
* Create functions that use explicit lifetime annotations
* Show how to use lifetimes with structs
* Implement functions with multiple lifetime parameters
* Demonstrate the 'static lifetime
* Include examples of common lifetime issues and how to fix them

```
$ cargo run
Simple lifetime example:
Longest string: "long string"

Struct with lifetime:
Person with name: "Alice" and title: "Engineer"

Multiple lifetimes:
Combined string: "Hello, World!"

Static lifetime:
Static string: "I live for the entire program"

Lifetime issues:
Example 1: Fixed with explicit lifetime
Example 2: Fixed by extending scope
Example 3: Fixed by owned value
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x03-ownership`
* File: `lifetime_examples/src/main.rs`, `lifetime_examples/Cargo.toml`


### 6. Data Structure with Ownership
**Optional (Advanced)**
Create a data structure that demonstrates ownership and borrowing:

-   Implement a custom linked list or binary tree.
-   Show how to manage ownership of nodes.
-   Demonstrate borrowing for read-only operations.
-   Implement methods to add, remove, and traverse elements.

```rust
$ cargo run
Linked List: 1 -> 2 -> 3 -> Nil
Element at index 1: Some(2)
List after removing 2: 1 -> 3 -> Nil
$
```

**Repo:**

-   GitHub repository: `alx-rust_programming`
-   Directory: `0x03-ownership`
-   File: `data_structure/src/main.rs`, `data_structure/Cargo.toml`

### 7. Smart Pointers and Ownership
**Optional (Advanced)**
Create a program that demonstrates the use of smart pointers with ownership:

-   Use `Box<T>`, `Rc<T>`, and `RefCell<T>` to manage ownership.
-   Show how to use `Rc<T>` for shared ownership.
-   Demonstrate mutable borrowing with `RefCell<T>`.
-   Implement a scenario where smart pointers are necessary.

```rust
$ cargo run
Box example: Value: 10
Rc example: Count: 2, Value: 20
RefCell example: Modified value: 30
$
```

**Repo:**

-   GitHub repository: `alx-rust_programming`
-   Directory: `0x03-ownership`
-   File: `smart_pointers/src/main.rs`, `smart_pointers/Cargo.toml`

### 8. Ownership and Threads
**Optional (Advanced)**
Create a program that demonstrates ownership and borrowing in a multi-threaded context:

-   Use `std::thread` to create multiple threads.
-   Show how to move ownership to a thread.
-   Demonstrate how to use `Arc<T>` and `Mutex<T>` for shared mutable access.
-   Implement a scenario where threads need to share and modify data.

```rust
$ cargo run
Thread 1: Value: 10
Thread 2: Value: 20
Final value: 30
$
```

**Repo:**

-   GitHub repository: `alx-rust_programming`
-   Directory: `0x03-ownership`
-   File: `ownership_threads/src/main.rs`, `ownership_threads/Cargo.toml`

### 9. Custom Drop Implementation
**Optional (Advanced)**
Create a program that demonstrates custom `Drop` implementations:

-   Implement the `Drop` trait for a custom struct.
-   Show how `Drop` is called when an object goes out of scope.
-   Demonstrate resource cleanup in the `Drop` implementation.
-   Implement a scenario where custom `Drop` behavior is needed.

```rust
$ cargo run
Creating custom drop struct...
Custom drop struct goes out of scope. Cleaning up...
$
```

**Repo:**

-   GitHub repository: `alx-rust_programming`
-   Directory: `0x03-ownership`
-   File: `custom_drop/src/main.rs`, `custom_drop/Cargo.toml`

### 10. Memory Safety with Unsafe Rust
**Optional (Advanced)**
Create a program that demonstrates memory safety with `unsafe` Rust:

-   Use `unsafe` blocks to perform raw pointer operations.
-   Show how to create and dereference raw pointers.
-   Demonstrate how to use `unsafe` to call external C functions.
-   Implement a scenario where `unsafe` is necessary for performance or interoperability.
-   Write clear comments explaining the safety guarantees and invariants that must be upheld.

```rust
$ cargo run
Raw pointer value: 10
C function result: 20
$
```

**Repo:**

-   GitHub repository: `alx-rust_programming`
-   Directory: `0x03-ownership`
-   File: `unsafe_memory/src/main.rs`, `unsafe_memory/Cargo.toml`

