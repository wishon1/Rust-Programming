// Function with no parameters
fn function_no_params() {
    println!("Function with no parameters called");
}

// function with parameters
fn function_with_params(x: i32, y: i32) {
    println!("Funtion with parameters called: x = {}, y = {}", x, y);
}

// function with return value
fn function_with_return(x: i32, y: i32) -> i32 {
    x + y
}

// function demonstrating expression syntax
fn expression_function(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum
}

// Higher-order function (takes a function as a parameter)
fn higher_order_function(x: i32, f: fn(i32) -> i32) -> i32 {
    f(x)
}

// Recursive function (calculates factoral)
fn factoral(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factoral(n - 1)
    }
}

fn main() {
    function_no_params();
    function_with_params(5, 10);
    let result = function_with_return(5, 10);
    println!("Function with return value: {}", result);
    
    let expr_result = expression_function(3, 4);
    println!("Expression result: {}", expr_result);
    
    let higher_order_result = higher_order_function(10, |x| x * 2);
    println!("Higher-order function result: {}", higher_order_result);
    
    let factoral_result = factoral(5);
    println!("Recursive factorial of 5: {}", factoral_result);
}