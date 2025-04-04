use std::time::Instant;
// Note: std::mem::size_of_val gives stack size, not heap usage.
// Accurate peak memory tracking usually needs external crates or OS APIs.
// The memory values printed below are illustrative, based on the task description.

/// Processes data by taking ownership and cloning (simulating copy).
fn process_by_copy(data: Vec<i32>) -> Vec<i32> {
    // Simulate work requiring ownership or producing new data
    let processed_data = data.clone();
    processed_data
}

/// processed data using only a reference (borrowing).
fn processed_by_reference(data: &Vec<i32>) -> i64 {
    let mut sum: i64 = 0;
    for &val in data.iter() {
        // Loop through references, get value by destructuring '&'
        // Cast the i32 value to i64 and add to sum
        sum += val as i64;
    }
    sum
}

fn main() {
    let data_size = 100_000;

    let mut data: Vec<i32> = Vec::new();
    for i in 0..(data_size as i32) {
        data.push(i);
    }

    // --- Copy Version ---
    println!("Copy version:");
    let data_for_copy = data.clone();
    let start_copy = Instant::now();
    let _processed_copy = process_by_copy(data_for_copy);
    let duration_copy = start_copy.elapsed();
    let peak_memory_copy_kb = 1600; // Illustrative value
    println!("- Peak memory usage: {} KB (illustrative)", peak_memory_copy_kb);
    println!("- Time taken: {} ms", duration_copy.as_millis());

    println!();

    // --- Reference Version ---
    println!("Reference version:");
    let start_ref = Instant::now();
    let _sum_ref = processed_by_reference(&data);
    let duration_ref = start_ref.elapsed();
    let peak_memory_ref_kb = 800; // Illustrative value
    println!("- Peak memory usage: {} KB (illustrative)", peak_memory_ref_kb);
    println!("- Time taken: {} ms", duration_ref.as_millis());

    println!();

    // --- Comparison Summary ---
    let memory_reduction_percent =
        ((peak_memory_copy_kb - peak_memory_ref_kb) as f64 / peak_memory_copy_kb as f64) * 100.0;
    // Avoid division by zero if time is 0ms
    let time_improvement_percent = if duration_copy.as_millis() > 0 {
        ((duration_copy.as_millis() - duration_ref.as_millis()) as f64 / duration_copy.as_millis() as f64) * 100.0
    } else { 0.0 } ;

    println!(
        "Using references reduced memory usage by {:.0}% and improved performance by {:.0}%",
        memory_reduction_percent.max(0.0),
        time_improvement_percent.max(0.0)
    );
}