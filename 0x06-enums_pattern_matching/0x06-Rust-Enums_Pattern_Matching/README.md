I'll continue with Rust chapters 6-12, each with 8 tasks in the ALX format. Here's the next set of exercises:

# 0x06. Rust - Enums and Pattern Matching

## Resources
Read or watch:
- [Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The match Control Flow Operator](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Concise Control Flow with if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [Rust By Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Rust By Example - Pattern Matching](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:

### General
- Why Rust's enums are more powerful than enums in other languages
- How to define and use enums
- How to use enums with data
- How to use the `Option<T>` enum
- How to use pattern matching with `match` expressions
- How to use `if let` syntax for concise pattern matching
- How to use pattern matching with `match` guards
- How to handle exhaustiveness in pattern matching
- How to use enums to represent variants of data

## Requirements
- Allowed editors: vim, emacs, Visual Studio Code
- All your files will be compiled on Ubuntu 20.04 LTS using rustc 1.68.0
- All your files should end with a new line
- A README.md file at the root of the folder of the project is mandatory
- Your code should use the rustfmt style
- All your Rust source files must pass `cargo clippy` without warnings
- The first line of all your files should be `// 0x06. Rust - Enums and Pattern Matching`

## Tasks

### 0. Message enum
**Mandatory**
Create a file `src/message.rs` that defines an enum `Message` with various variants:

- Define an enum `Message` with the following variants:
  - `Quit` (no associated data)
  - `Move { x: i32, y: i32 }` (named fields)
  - `Write(String)` (tuple struct)
  - `ChangeColor(i32, i32, i32)` (tuple struct with RGB values)
- Implement a `call` method for `Message` that prints different messages based on the variant
- Test with the following main function:

```rust
// src/main.rs
use message::Message;

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in &messages {
        message.call();
    }
}
```

Expected output:
```
Quit message received
Move to x: 10, y: 20
Text message: Hello, Rust!
Change color to RGB(255, 0, 0)
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/message.rs, src/main.rs, Cargo.toml`

### 1. Option handling
**Mandatory**
Create a file `src/option_handler.rs` that works with Rust's `Option` type:

- Implement a function `find_value` that takes a vector of integers and a value to find
- The function should return `Option<usize>` - the index of the first occurrence of the value, or None if not found
- Implement a function `display_result` that takes an `Option<usize>` and prints an appropriate message
- Test with the following main function:

```rust
// src/main.rs
mod option_handler;

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    
    for value in [30, 60] {
        let result = option_handler::find_value(&numbers, value);
        option_handler::display_result(result, value);
    }
}
```

Expected output:
```
Found value 30 at index 2
Value 60 not found in the vector
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/option_handler.rs, src/main.rs, Cargo.toml`

### 2. Coin counter
**Mandatory**
Create a file `src/coin.rs` that defines an enum for different types of coins:

- Define an enum `Coin` with variants `Penny`, `Nickel`, `Dime`, and `Quarter`
- Implement a function `value_in_cents` that returns the value of each coin in cents
- Implement a function `count_coins` that takes a vector of coins and returns the total value
- Test with the following main function:

```rust
// src/main.rs
use coin::{Coin, count_coins, value_in_cents};

fn main() {
    let coins = vec![
        Coin::Penny,
        Coin::Quarter,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter,
    ];
    
    println!("Value of a Quarter: {} cents", value_in_cents(Coin::Quarter));
    println!("Total value of coins: {} cents", count_coins(&coins));
}
```

Expected output:
```
Value of a Quarter: 25 cents
Total value of coins: 66 cents
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/coin.rs, src/main.rs, Cargo.toml`

### 3. State quarter
**Mandatory**
Modify the `Coin` enum from the previous task to include additional data:

- Add a `State` enum with variants for at least 5 US states
- Modify the `Quarter` variant to include an associated `State` value
- Update the `value_in_cents` function to handle the modified `Quarter` variant
- Implement a function `print_state_quarters` that identifies and prints only state quarters
- Test with the following main function:

```rust
// src/main.rs
use coin::{Coin, State, print_state_quarters, value_in_cents};

fn main() {
    let coins = vec![
        Coin::Penny,
        Coin::Quarter(State::Alabama),
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(State::Alaska),
        Coin::Quarter(State::Arizona),
    ];
    
    println!("Value of Alabama Quarter: {} cents", value_in_cents(Coin::Quarter(State::Alabama)));
    print_state_quarters(&coins);
}
```

Expected output:
```
Value of Alabama Quarter: 25 cents
State quarters found:
Alabama quarter
Alaska quarter
Arizona quarter
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/coin.rs, src/main.rs, Cargo.toml`

### 4. if let expressions
**Mandatory**
Create a file `src/config.rs` that demonstrates the use of `if let` expressions:

- Define an enum `ServerConfig` with variants like `IP(String)`, `Port(u16)`, and `MaxConnections(u32)`
- Implement a function `apply_config` that takes a `ServerConfig` and applies the configuration using `if let`
- Test with the following main function:

```rust
// src/main.rs
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
```

Expected output:
```
Setting server IP to 127.0.0.1
Setting server port to 8080
Setting maximum connections to 100
Setting server port to 9000
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/config.rs, src/main.rs, Cargo.toml`

### 5. Match with guards
**Mandatory**
Create a file `src/temperature.rs` that uses match guards:

- Define an enum `Temperature` with variants `Celsius(f64)` and `Fahrenheit(f64)`
- Implement a function `describe_temperature` that categorizes temperatures using match guards
- Categories should include: freezing, cold, moderate, warm, hot, and boiling
- Test with the following main function:

```rust
// src/main.rs
use temperature::{Temperature, describe_temperature};

fn main() {
    let temperatures = vec![
        Temperature::Celsius(-10.0),
        Temperature::Celsius(15.0),
        Temperature::Celsius(40.0),
        Temperature::Fahrenheit(32.0),
        Temperature::Fahrenheit(68.0),
        Temperature::Fahrenheit(212.0),
    ];
    
    for temp in temperatures {
        describe_temperature(temp);
    }
}
```

Expected output:
```
Celsius(-10.0) is freezing
Celsius(15.0) is moderate
Celsius(40.0) is hot
Fahrenheit(32.0) is freezing
Fahrenheit(68.0) is moderate
Fahrenheit(212.0) is boiling
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/temperature.rs, src/main.rs, Cargo.toml`

### 6. Binary tree
**Mandatory**
Create a file `src/binary_tree.rs` that implements a simple binary tree using enums:

- Define an enum `BinaryTree` with a variant `Node` containing a value and left/right subtrees
- Implement a method `insert` to add values to the tree
- Implement a method `contains` to check if a value exists in the tree
- Implement a method `print_in_order` to display the tree in-order
- Test with the following main function:

```rust
// src/main.rs
use binary_tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::Empty;
    
    for value in [5, 3, 7, 2, 4, 6, 8] {
        tree.insert(value);
    }
    
    println!("Tree contains 4: {}", tree.contains(4));
    println!("Tree contains 9: {}", tree.contains(9));
    
    println!("In-order traversal:");
    tree.print_in_order();
}
```

Expected output:
```
Tree contains 4: true
Tree contains 9: false
In-order traversal:
2
3
4
5
6
7
8
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/binary_tree.rs, src/main.rs, Cargo.toml`

### 7. Command processor
**Mandatory**
Create a file `src/command.rs` that processes commands using enums:

- Define an enum `Command` with variants like `Add`, `Subtract`, `Multiply`, `Divide`, and `Exit`
- Implement a function `process_command` that takes a `Command` and performs the operation
- Implement a function `parse_command` that converts a string to a `Command`
- Test with the following main function:

```rust
// src/main.rs
use command::{Command, process_command, parse_command};

fn main() {
    let commands = vec![
        "add 5 3",
        "multiply 2 4",
        "subtract 10 7",
        "divide 15 3",
        "divide 10 0",
        "exit",
    ];
    
    let mut result = 0;
    for cmd_str in commands {
        println!("Command: {}", cmd_str);
        let cmd = parse_command(cmd_str);
        result = process_command(cmd, result);
        println!("Result: {}", result);
    }
}
```

Expected output:
```
Command: add 5 3
Result: 8
Command: multiply 2 4
Result: 8
Command: subtract 10 7
Result: 3
Command: divide 15 3
Result: 5
Command: divide 10 0
Cannot divide by zero
Result: 5
Command: exit
Exiting...
Result: 5
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/command.rs, src/main.rs, Cargo.toml`

### 8. Result handling
**Advanced**
Create a file `src/file_operation.rs` that handles file operations using the `Result` enum:

- Define an enum `FileError` with variants for different errors (NotFound, PermissionDenied, etc.)
- Implement a function `read_file` that returns `Result<String, FileError>`
- Implement a function `write_file` that returns `Result<(), FileError>`
- Implement a function `handle_file_operation` that handles both operations with pattern matching
- Test with the following main function:

```rust
// src/main.rs
use file_operation::{read_file, write_file, handle_file_operation};

fn main() {
    let operations = vec![
        ("read", "existing.txt"),
        ("read", "nonexistent.txt"),
        ("write", "new_file.txt"),
        ("write", "/root/forbidden.txt"),
        ("invalid", "something.txt"),
    ];
    
    for (operation, filename) in operations {
        println!("Operation: {} on file: {}", operation, filename);
        handle_file_operation(operation, filename);
    }
}
```

Expected output:
```
Operation: read on file: existing.txt
Success: Read 'Contents of existing.txt'
Operation: read on file: nonexistent.txt
Error: File not found
Operation: write on file: new_file.txt
Success: Wrote to new_file.txt
Operation: write on file: /root/forbidden.txt
Error: Permission denied
Operation: invalid on file: something.txt
Error: Invalid operation
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/file_operation.rs, src/main.rs, Cargo.toml`



Understood. Here are the 5 additional optional tasks for 0x06. Rust - Enums and Pattern Matching, with strict adherence to the requested style, including expected output.


### 9. Complex State Machine
**Optional (Advanced)**
Create a file `src/state_machine.rs` that implements a complex state machine using enums:

- Define an enum `State` representing the states of a device (e.g., `Idle`, `Connecting`, `Connected`, `Error`).
- Define an enum `Event` representing the events that trigger state transitions (e.g., `Start`, `ConnectSuccess`, `ConnectFail`, `Disconnect`).
- Implement a function `transition` that takes a `State` and an `Event` and returns the new `State` based on a predefined state transition table.
- Implement a function `process_events` that takes a vector of `Event` and an initial `State`, and returns the final `State` after processing all events.
- Test with a sequence of events and verify the final state.

```rust
// src/main.rs
mod state_machine;
use state_machine::{State, Event, process_events};

fn main() {
    let events = vec![
        Event::Start,
        Event::ConnectSuccess,
        Event::Disconnect,
    ];
    let initial_state = State::Idle;
    let final_state = process_events(events, initial_state);
    println!("Final state: {:?}", final_state);
}
```

Expected output:

```
Final state: Idle
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/state_machine.rs`, `src/main.rs`, `Cargo.toml`

### 10. Abstract Syntax Tree (AST)
**Optional (Advanced)**
Create a file `src/ast.rs` that defines an Abstract Syntax Tree (AST) using enums:

- Define an enum `Expr` to represent expressions in a simple language (e.g., `Number(i32)`, `Add(Box<Expr>, Box<Expr>)`, `Multiply(Box<Expr>, Box<Expr>)`).
- Implement a function `evaluate` that takes an `Expr` and returns the result of evaluating the expression.
- Implement a function `parse` that takes a string representation of an expression and returns an `Expr`.
- Test with various expressions and verify the results.

```rust
// src/main.rs
mod ast;
use ast::{Expr, evaluate, parse};

fn main() {
    let expr = parse("2 + 3 * 4");
    println!("Expression: {:?}", expr);
    println!("Result: {}", evaluate(expr));
}
```

Expected output:

```
Expression: Add(Number(2), Multiply(Number(3), Number(4)))
Result: 14
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/ast.rs`, `src/main.rs`, `Cargo.toml`

### 11. Variant Data Aggregation
**Optional (Advanced)**
Create a file `src/data_aggregation.rs` that aggregates data from different enum variants:

- Define an enum `Data` with variants like `Number(i32)`, `Text(String)`, and `Boolean(bool)`.
- Implement a function `aggregate_data` that takes a vector of `Data` and returns a summary of the data (e.g., counts of each variant, sum of numbers, concatenated text).
- Test with a mixed vector of `Data` and verify the summary.

```rust
// src/main.rs
mod data_aggregation;
use data_aggregation::{Data, aggregate_data};

fn main() {
    let data = vec![
        Data::Number(10),
        Data::Text(String::from("Hello")),
        Data::Boolean(true),
        Data::Number(20),
        Data::Text(String::from(" Rust")),
        Data::Boolean(false),
    ];
    let summary = aggregate_data(&data);
    println!("Data summary: {:?}", summary);
}
```

Expected output:

```
Data summary: Summary { number_count: 2, text_count: 2, boolean_count: 2, number_sum: 30, text_concat: "Hello Rust" }
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/data_aggregation.rs`, `src/main.rs`, `Cargo.toml`

### 12. Recursive Enum for Linked List
**Optional (Advanced)**
Create a file `src/linked_list.rs` that implements a singly linked list using a recursive enum:

- Define an enum `List<T>` with variants `Node(T, Box<List<T>>)` and `Nil`.
- Implement methods `push`, `pop`, `peek`, and `print` for the linked list.
- Test with various operations and verify the list's behavior.

```rust
// src/main.rs
mod linked_list;
use linked_list::List;

fn main() {
    let mut list = List::Nil;
    list = list.push(1);
    list = list.push(2);
    list = list.push(3);
    list.print();
    println!("Peek: {:?}", list.peek());
    println!("Pop: {:?}", list.pop());
    println!("Peek after pop: {:?}", list.peek());
}
```

Expected output:

```
3 -> 2 -> 1 -> Nil
Peek: Some(3)
Pop: Some(3)
Peek after pop: Some(2)
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/linked_list.rs`, `src/main.rs`, `Cargo.toml`

### 13. Dynamic Dispatch with Enums
**Optional (Advanced)**
Create a file `src/dynamic_dispatch.rs` that demonstrates dynamic dispatch using enums:

- Define an enum `Shape` with variants `Circle(f64)` and `Rectangle(f64, f64)`.
- Implement methods `area` and `perimeter` for each variant of `Shape`.
- Implement a function `process_shapes` that takes a vector of `Shape` and calculates the total area and perimeter.
- Test with a mixed vector of shapes and verify the results.

```rust
// src/main.rs
mod dynamic_dispatch;
use dynamic_dispatch::Shape;

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
    ];
    let (total_area, total_perimeter) = dynamic_dispatch::process_shapes(&shapes);
    println!("Total area: {}, Total perimeter: {}", total_area, total_perimeter);
}
```

Expected output:

```
Total area: 102.53981633974483, Total perimeter: 51.41592653589793
```

**Repo:**

- GitHub repository: `alx-rust_programming`
- Directory: `0x06-enums_pattern_matching`
- File: `src/dynamic_dispatch.rs`, `src/main.rs`, `Cargo.toml`

