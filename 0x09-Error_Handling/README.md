# 0x09. Rust — Error Handling

A progressive series of Rust projects covering every major error-handling
pattern, written from the perspective of a developer coming from

## Repository layout

```
0x09-error_handling/
├── panic_demo/          Task 0 – unrecoverable errors with panic!
├── result_demo/         Task 1 – recoverable errors with Result<T,E>
├── custom_errors/       Task 2 – custom error types
├── error_guidelines/    Task 3 – thiserror + anyhow, unit tests
├── cli_error_handling/  Task 4 – CLI app with clap, exit codes, --verbose
└── result_collection/   Task 5 – collect / partition / aggregate Results
```

## Quick start

```bash
# Task 0
cd panic_demo && cargo run

# Task 1
cd result_demo && cargo run

# Task 2
cd custom_errors && cargo run

# Task 3 – tests + demo
cd error_guidelines && cargo test && cargo run --example demo

# Task 4
cd cli_error_handling
cargo run -- --help
cargo run -- process sample.txt
cargo run -- process --verbose restricted.txt

# Task 5
cd result_collection && cargo run
```

## Concepts at a glance

| Concept | Project | Key types / macros |
|---|---|---|
| Unrecoverable errors | `panic_demo` | `panic!`, `RUST_BACKTRACE` |
| Recoverable errors | `result_demo` | `Result<T,E>`, `?`, `match` |
| Custom error types | `custom_errors` | `enum`, `Display`, `std::error::Error`, `From` |
| Ergonomic error libs | `error_guidelines` | `thiserror`, `anyhow` |
| CLI error UX | `cli_error_handling` | `clap`, exit codes, `--verbose` |
| Batch error handling | `result_collection` | `collect`, `partition`, `fold`, `map_err` |

## Compiler requirement

All crates target **Rust 2021 edition** and compile on the Rust version
shipped with Ubuntu 20.04 / 24.04 LTS (`rustc 1.75+`).