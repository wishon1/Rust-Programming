use calculator::shortcuts::{add, subtract, multiply, divide, power, sqrt, logarithm, format_result};
use calculator::calculate;

fn main() {
    println!("=== Basic Operations ===");

    // add
    let result = add(5.0, 3.0);
    println!("add:      5 + 3        = {}", format_result(result));

    // subtract
    let result = subtract(10.0, 4.0);
    println!("subtract: 10 - 4       = {}", format_result(result));

    // multiply
    let result = multiply(3.0, 4.0);
    println!("multiply: 3 * 4        = {}", format_result(result));

    // divide (returns Result)
    match divide(15.0, 3.0) {
        Ok(result) => println!("divide:   15 / 3       = {}", format_result(result)),
        Err(e)     => println!("divide error: {}", e),
    }

    // divide by zero — shows error path
    match divide(7.0, 0.0) {
        Ok(result) => println!("divide:   7 / 0        = {}", format_result(result)),
        Err(e)     => println!("divide:   7 / 0        => Error: {}", e),
    }

    println!("\n=== Advanced Operations ===");

    // power
    let result = power(2.0, 10.0);
    println!("power:    2 ^ 10       = {}", format_result(result));

    // sqrt (returns Result)
    match sqrt(144.0) {
        Ok(result) => println!("sqrt:     √144         = {}", format_result(result)),
        Err(e)     => println!("sqrt error: {}", e),
    }

    // sqrt of negative — shows error path
    match sqrt(-9.0) {
        Ok(result) => println!("sqrt:     √-9          = {}", format_result(result)),
        Err(e)     => println!("sqrt:     √-9          => Error: {}", e),
    }

    // logarithm (returns Result)
    match logarithm(std::f64::consts::E) {
        Ok(result) => println!("log:      ln(e)        = {}", format_result(result)),
        Err(e)     => println!("log error: {}", e),
    }

    // logarithm of non-positive — shows error path
    match logarithm(-1.0) {
        Ok(result) => println!("log:      ln(-1)       = {}", format_result(result)),
        Err(e)     => println!("log:      ln(-1)       => Error: {}", e),
    }

    println!("\n=== Via calculate() string API ===");

    let expressions = [
        "add 100 200",
        "subtract 50 18",
        "multiply 6 7",
        "divide 22 7",
        "power 3 4",
        "sqrt 256",
        "log 2.718281828",
        "divide 1 0",       // triggers divide-by-zero error
        "sqrt -4",          // triggers negative sqrt error
        "log -5",           // triggers non-positive log error
    ];

    for expr in &expressions {
        match calculate(expr) {
            Ok(result) => println!("  {:20} => {}", expr, format_result(result)),
            Err(e)     => println!("  {:20} => Error: {}", expr, e),
        }
    }
}