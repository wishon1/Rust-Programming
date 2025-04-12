# 0x02. Rust - Common Programming Concepts
Rust

Weight: 1
Project duration: 3 days

## Concepts
*For this project, we expect you to look at these concepts:*
* Variables and mutability
* Data types in Rust
* Functions and control flow
* Ownership and borrowing basics

## Resources
Read or watch:
* The Rust Programming Language - Chapter 3
* [Rust By Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
* [Rust By Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:
* How variables work in Rust
* The differences between mutable and immutable variables
* Rust's basic data types
* How to write functions in Rust
* How to use control flow constructs
* The significance of ownership in Rust

## Tasks

### 0. Variables and Mutability
**mandatory**

Write a Rust program that demonstrates variable declaration, initialization, and mutation.
* Create immutable and mutable variables of different data types
* Try to modify an immutable variable and handle the error
* Use shadowing to change the value and type of a variable
* Demonstrate the use of constants
* Display all variables and their values

```
$ cargo run
Immutable variable: 5
Mutable variable before change: 10
Mutable variable after change: 15
Shadowed variable (first): Hello
Shadowed variable (second): 3
Constant value: 60
Error demonstration: cannot assign twice to immutable variable `x`
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: `variables/src/main.rs`, `variables/Cargo.toml`

### 1. Data Types
**mandatory**

Create a Rust program that demonstrates all the basic data types.
* Include examples of: integers, floating-point numbers, booleans, characters, and tuples
* Show overflow handling for integers
* Demonstrate tuple indexing and destructuring
* Show array declaration and access
* Include examples of type conversions

```
$ cargo run
Integer types:
i8: -128 to 127
u8: 0 to 255
Current value (i32): 42

Floating point:
f32 value: 3.14
f32 precision example: 0.1 + 0.2 = 0.30000000000000004

Boolean:
bool value: true
Logical AND: false
Logical OR: true

Characters:
Character: A
Unicode character: ☺

Tuples:
Tuple: (42, 3.14, true)
First value: 42
Destructured: a=42, b=3.14, c=true

Arrays:
Array: [1, 2, 3, 4, 5]
Second element: 2
Length: 5

Type conversion:
i32 to f64:

```
i32 to f64: 42 -> 42.0
f64 to i32: 3.14 -> 3 (truncated)
Char to u8: 'A' -> 65
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: `data_types/src/main.rs`, `data_types/Cargo.toml`

### 2. Functions
**mandatory**

Write a Rust program that demonstrates different aspects of functions.
* Create a main function that calls several other functions
* Include functions with parameters and return values
* Use both statement and expression syntax in function bodies
* Demonstrate function overloading (not supported in Rust!)
* Show how to pass functions as parameters
* Implement a recursive function

```
$ cargo run
Function with no parameters called
Function with parameters called: x = 5, y = 10
Function with return value: 15
Expression result: 7
Higher-order function result: 20
Recursive factorial of 5: 120
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: `functions/src/main.rs`, `functions/Cargo.toml`

### 3. Control Flow
**mandatory**

Create a Rust program that demonstrates various control flow constructs.
* Use if/else expressions
* Show how to use if in a let statement
* Implement different types of loops: loop, while, for
* Use break and continue in appropriate contexts
* Demonstrate loop labels
* Use match expressions for pattern matching

```
$ cargo run
If-else result: Number is positive
If in let: 10
Loop with break: Counted to 5
While loop: Counted down from 5 to 1
For loop through range: 1 2 3 4 5
For loop through array: 10 20 30 40 50
Nested loops with labels: Completed
Match expression: Number is between 1 and 5
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: `control_flow/src/main.rs`, `control_flow/Cargo.toml`

### 4. Temperature Converter
**mandatory**

Write a Rust program that converts between Fahrenheit and Celsius temperatures.
* The program should accept a temperature value and a unit (F or C)
* It should convert the temperature to the other unit
* The formula for conversion is: F = C * 9/5 + 32
* Use proper function organization and error handling
* Allow for command-line arguments or user input

```
$ cargo run
Temperature Converter

Enter temperature: 32
Enter unit (F/C): F
32°F is 0°C

Convert another? (y/n): y
Enter temperature: 100
Enter unit (F/C): C
100°C is 212°F

Convert another? (y/n): n
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: `temp_converter/src/main.rs`, `temp_converter/Cargo.toml`

### 5. Fibonacci Generator
**advanced**

Create a Rust program that generates Fibonacci numbers.
* Implement the Fibonacci sequence using iteration and recursion
* Compare the performance of both approaches
* Use proper error handling for invalid inputs
* Format the output nicely
* Allow for generating a specific Fibonacci number or a sequence

```
$ cargo run
Fibonacci Generator

Enter a command:
1. Generate nth Fibonacci number
2. Generate Fibonacci sequence
3. Compare performance
4. Exit

Choice: 1
Enter n: 10
The 10th Fibonacci number is: 55

Choice: 2
How many numbers to generate? 8
Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13

Choice: 3
Calculating Fibonacci(30):
Recursive: 832040 ns
Iterative: 850 ns
Iterative is 979 times faster!

Choice: 4
Goodbye!
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: `fibonacci/src/main.rs`, `fibonacci/Cargo.toml`

### 6. FizzBuzz Rust
**advanced**

Implement the classic FizzBuzz problem in Rust.
* For numbers 1 to n, print:
  * "Fizz" if the number is divisible by 3
  * "Buzz" if the number is divisible by 5
  * "FizzBuzz" if the number is divisible by both 3 and 5
  * The number itself otherwise
* Use proper Rust conventions and error handling
* Implement at least two different approaches (e.g., using if/else vs. match)
* Allow for command-line arguments to specify the range

```
$ cargo run -- 15
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
$
```

**Repo:**
* GitHub repository: `alx-rust_programming`
* Directory: `0x02-common_concepts`
* File: fizzbuzz/src/main.rs, fizzbuzz/Cargo.toml
