fn main() {
    // if-else expression
    let num = 5;
    if num > 0 {
        println!("if-else result: {} is positive", num);
    } else {
        println!("if-else result: {} is non-positive", num);
    }

    // if in let statement
    let x = if num > 0 { 10 } else { -10 };
    println!("if in let: {}", x);

    // loop with break
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            println!("Loop with break: Counted to 5");
            break;
        }
    }

    // While loop
    let mut n = 5;
    while n > 0 {
        println!("While loop: Counted down from {} to 1", n);
        n -= 1;
    }

    // For loop through range
    println!("For loop through range: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // For loop through array
    let arr = [ 10, 20, 30, 40, 50];
    print!("For loop through array: ");
    for &item in &arr {
        print!("{} ", item);
    }
    println!();

    // Nested loops with labels
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                break 'outer;
            }
        }
    }
    println!("Nested loops with labels: Completed");

    // match expression
    let number = 3;
    match number {
        1..=5 => println!("Match expression: Number is between 1 and 5"),
        _=> println!("Match expression: Number is out of range"),
    }
}
