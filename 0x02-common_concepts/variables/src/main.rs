fn main() {
    // Immutable variable declaration
    let x = 5;
    println!("Immutable variable: {}", x);
    
    // Mutable variable declaration and mutation
    let mut y = 10;
    println!("Mutable variable before change: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("Mutable variable after change: {}", y);
    
    // Shadowing - reusing variable name
    let var = "Hello";
    println!("Shadowed variable (first): {}", var);
    
    // Shadowing allows changing type
    let var = 3; // This is a different variable that shadows the previous one
    println!("Shadowed variable (second): {}", var);
    
    // Constant declaration - note the naming convention and required type annotation
    const MAX_POINTS: u32 = 60;
    println!("Constant value: {}", MAX_POINTS);
    
    // Demonstration of error when trying to modify immutable variable
    // Uncomment the line below to see the error:
    // x = 6; // This would cause a compilation error
    println!("Error demonstration: cannot assign twice to immutable variable `x`");
    
    // Uncommenting the line above would produce an error like:
    // error[E0384]: cannot assign twice to immutable variable `x`
}