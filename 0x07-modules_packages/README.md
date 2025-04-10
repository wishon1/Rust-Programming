# 0x07. Rust - Packages, Crates, and Modules

## Learning Objectives
At the end of this project, you are expected to be able to explain to anyone, without the help of Google:

### General
- The difference between packages, crates, and modules in Rust
- How to organize related code in modules
- How to control privacy with `pub` keyword
- How to use the module system to structure large projects
- How to use different paths for referring to items in the module tree
- How to use the `use` keyword to bring paths into scope
- How to create idiomatic `use` paths
- How to separate modules into different files
- When to use absolute vs relative paths

## Requirements
- Allowed editors: vim, emacs, Visual Studio Code
- All your files will be compiled on Ubuntu 20.04 LTS using rustc 1.68.0
- All your files should end with a new line
- A README.md file at the root of the folder of the project is mandatory
- Your code should use the rustfmt style
- All your Rust source files must pass `cargo clippy` without warnings
- The first line of all your files should be `// 0x07. Rust - Packages, Crates, and Modules`

## Tasks

### 0. Restaurant module
**Mandatory**
Create a module that models the components of a restaurant:

- Create a library crate called `restaurant`
- Create a module `front_of_house` with submodules `hosting` and `serving`
- In `hosting`, define functions `add_to_waitlist` and `seat_at_table`
- In `serving`, define functions `take_order`, `serve_order`, and `take_payment`
- Make the necessary items public using the `pub` keyword
- Create a `customer` function in the root module that demonstrates how to call these functions
- Test with the following main function:

```rust
// src/main.rs
use restaurant;

fn main() {
    restaurant::customer();
}
```

Expected output:
```
Please follow the host to your table
Seated at table
Taking order
Serving order: Pizza
Processing payment
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `src/lib.rs, src/main.rs, Cargo.toml`

### 1. Multiple files
**Mandatory**
Restructure the restaurant module to use multiple files:

- Move the `front_of_house` module to its own file
- Move the `hosting` and `serving` modules to their own files
- Maintain the same functionality as the previous task
- Test with the same main function

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `src/lib.rs, src/front_of_house.rs, src/front_of_house/hosting.rs, src/front_of_house/serving.rs, src/main.rs, Cargo.toml`

### 2. Calculator library
**Mandatory**
Create a calculator library with various operations:

- Create a library crate called `calculator`
- Create modules for `basic_ops`, `advanced_ops`, and `utils`
- In `basic_ops`, implement functions for `add`, `subtract`, `multiply`, and `divide`
- In `advanced_ops`, implement functions for `power`, `sqrt`, and `logarithm`
- In `utils`, implement functions for `validate_input` and `format_result`
- Create a `calculate` function in the root module that uses the appropriate operation based on input
- Test with the following main function:

```rust
// src/main.rs
use calculator;

fn main() {
    let operations = vec![
        "add 5 3",
        "subtract 10 4",
        "multiply 3 4",
        "divide 10 2",
        "power 2 3",
        "sqrt 16",
        "divide 5 0",  // Should handle error
    ];
    
    for op in operations {
        match calculator::calculate(op) {
            Ok(result) => println!("{} = {}", op, result),
            Err(e) => println!("{} error: {}", op, e),
        }
    }
}
```

Expected output:
```
add 5 3 = 8
subtract 10 4 = 6
multiply 3 4 = 12
divide 10 2 = 5
power 2 3 = 8
sqrt 16 = 4
divide 5 0 error: Cannot divide by zero
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `calculator/src/lib.rs, calculator/src/basic_ops.rs, calculator/src/advanced_ops.rs, calculator/src/utils.rs, calculator/src/main.rs, calculator/Cargo.toml`

### 3. Use keyword
**Mandatory**
Modify the calculator library to use the `use` keyword effectively:

- Restructure your code to use `use` for bringing items into scope
- Demonstrate idiomatic use paths with proper grouping
- Create a `shortcuts` module that re-exports commonly used functions
- Test with the following main function:

```rust
// src/main.rs
use calculator::shortcuts::{add, subtract, multiply};

fn main() {
    println!("5 + 3 = {}", add(5, 3));
    println!("10 - 4 = {}", subtract(10, 4));
    println!("3 * 4 = {}", multiply(3, 4));
}
```

Expected output:
```
5 + 3 = 8
10 - 4 = 6
3 * 4 = 12
```

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `calculator/src/lib.rs, calculator/src/shortcuts.rs, calculator/src/main.rs, calculator/Cargo.toml`

### 4. Privacy and scoping
**Mandatory**
Create a module that demonstrates Rust's privacy rules:

- Create a module `privacy_demo` with nested modules
- Include functions with different visibility levels (public, private)
- Include a struct with both public and private fields
- Include a public function that manipulates private data
- Add comments explaining the privacy rules
- Test with a main function that demonstrates which items are accessible

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `src/privacy_demo.rs, src/main.rs, Cargo.toml`

### 5. Custom path imports
**Mandatory**
Create a module that demonstrates using the `as` keyword for custom imports:

- Create modules with functions that have potentially conflicting names
- Import these functions with custom names using `as`
- Demonstrate when to use `self`, `super`, and `crate` in paths
- Test with a main function that shows clear usage of renamed imports

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `src/custom_imports.rs, src/main.rs, Cargo.toml`

### 6. Nested modules
**Mandatory**
Create a deeply nested module structure:

- Create a module tree with at least 4 levels of nesting
- Demonstrate both absolute and relative paths
- Include examples of `super` to access parent modules
- Include examples of `crate` to access from the crate root
- Test with a main function that accesses functions at different levels

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `src/nested_modules.rs, src/main.rs, Cargo.toml`

### 7. Re-exporting
**Mandatory**
Create a module that demonstrates re-exporting:

- Create several internal modules with functions
- Create a public API module that re-exports selected functions
- Use nested `pub use` statements to flatten the module hierarchy
- Test with a main function that uses the re-exported functions

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages`
- File: `src/api.rs, src/internal_modules.rs, src/main.rs, Cargo.toml`

### 8. Workspace
**Advanced**
Create a Cargo workspace with multiple crates:

- Create a workspace with at least 3 crates: `common_utils`, `front_end`, and `back_end`
- Make `front_end` and `back_end` depend on `common_utils`
- Create some shared functionality in `common_utils`
- Implement specific functionality in the other crates
- Create a binary crate that uses all three libraries
- Test with a main function that demonstrates the workspace integration

**Repo:**
- GitHub repository: `alx-rust_programming`
- Directory: `0x07-modules_packages/workspace`
- File: `Cargo.toml, common_utils/src/lib.rs, front_end/src/lib.rs, back_end/src/lib.rs, app/src/main.rs`

***Provide directory should be structured in this manner:
```
alx-rust_programming/
├── 0x07-modules_packages/
│   ├── src/
│   │   ├── lib.rs                  # Task 0 & Task 1 library crate
│   │   ├── main.rs                 # Main file for various tasks
│   │   ├── front_of_house.rs       # Task 1 front_of_house module
│   │   ├── front_of_house/
│   │   │   ├── hosting.rs          # Task 1 hosting module
│   │   │   └── serving.rs          # Task 1 serving module
│   │   ├── privacy_demo.rs         # Task 4 privacy demo
│   │   ├── custom_imports.rs       # Task 5 custom imports
│   │   ├── nested_modules.rs       # Task 6 nested modules
│   │   ├── api.rs                  # Task 7 re-exporting
│   │   └── internal_modules.rs     # Task 7 internal modules
│   ├── Cargo.toml                  # Project manifest
│   │
│   ├── calculator/                 # Task 2 & Task 3 calculator crate
│   │   ├── src/
│   │   │   ├── lib.rs              # Calculator library
│   │   │   ├── main.rs             # Calculator main
│   │   │   ├── basic_ops.rs        # Basic operations
│   │   │   ├── advanced_ops.rs     # Advanced operations
│   │   │   ├── utils.rs            # Utility functions
│   │   │   └── shortcuts.rs        # Task 3 shortcuts
│   │   └── Cargo.toml              # Calculator manifest
│   │
│   └── workspace/                  # Task 8 workspace
│       ├── Cargo.toml              # Workspace manifest
│       ├── common_utils/
│       │   ├── src/
│       │   │   └── lib.rs          # Common utilities library
│       │   └── Cargo.toml          # Common utilities manifest
│       ├── front_end/
│       │   ├── src/
│       │   │   └── lib.rs          # Front-end library
│       │   └── Cargo.toml          # Front-end manifest
│       ├── back_end/
│       │   ├── src/
│       │   │   └── lib.rs          # Back-end library
│       │   └── Cargo.toml          # Back-end manifest
│       └── app/
│           ├── src/
│           │   └── main.rs         # Application main
│           └── Cargo.toml          # Application manifest
```
heres instruction to do it: 
```
