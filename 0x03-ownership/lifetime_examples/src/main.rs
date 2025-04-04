/// Returns the longest of two string slices.
/// The returned reference is tied to the shorter of the input lifetimes.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// A struct holding references, lifetime 'a ensures references don't outlive the data.
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

/// Combines string slices, returns an owned String (no output lifetime needed).
fn combine_parts<'a, 'b>(part1: &'a str, part2: &'b str) -> String {
    format!("{}, {}!", part1, part2)
}

// fn problematic_dangling_ref() -> &str { let s = String::from("temp"); &s } // Error!

/// Fix for dangling reference: Return an owned String.
fn fix_dangling_ref_owned() -> String {
    let s = String::from("temporary");
    s
}

/// Fix for dangling reference: Tie return lifetime to an input reference.
fn fix_dangling_ref_caller<'a>(data_source: &'a String) -> &'a str {
     data_source // Return reference tied to caller data
}

fn main() {
    println!("Simple lifetime example:");
    let string1 = String::from("abcd");
    let string2 = "xyz"; // &'static str
    println!("Longest string: \"{}\"", longest(string1.as_str(), string2));

    println!("\nStruct with lifetime:");
    let name = String::from("Alice");
    let person = Person { name: &name, title: "Engineer" };
    println!("Person with name: \"{}\" and title: \"{}\"", person.name, person.title);

    println!("\nMultiple lifetimes:");
    let part_a = "Hello"; let part_b = String::from("World");
    println!("Combined string: \"{}\"", combine_parts(part_a, &part_b));

    println!("\nStatic lifetime:");
    let static_string: &'static str = "I live for the entire program";
    println!("Static string: \"{}\"", static_string);

    println!("\nLifetime issues:");
    println!("Example 1: Fixed with owned value: \"{}\"", fix_dangling_ref_owned());
    let external_data = String::from("persistent data");
    println!("Example 2: Fixed by extending scope (passing reference): \"{}\"", fix_dangling_ref_caller(&external_data));
    println!("Example 3: Compiler prevents dangling references directly.");
    println!("Example 3: Fixed by returning owned value (as in Fix 1)");
}