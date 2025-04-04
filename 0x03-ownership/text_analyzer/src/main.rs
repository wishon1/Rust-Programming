use std::io::{self, Write};
use std::collections::HashMap;

// for explicit comparison result
use std::cmp::Ordering;

/// Counts Unicode characters in a text slice.
// or use; text.chars().count()
fn count_characters(text: &str) -> usize {
    let mut count = 0;
    for _char in text.chars() {
        count += 1;
    }
    count
}

/// Counts words by looping over slit_whitespace result
// or use; text.split_whitespace().count()
fn count_words(text: &str) -> usize {
    let mut count = 0;
    for _word in text.split_whitespace() {
        count += 1
    }
    count
}

/// /// Counts sentences by looping through chars and checking terminators.
fn count_sentences(text: &str) -> usize {
    let mut count = 0;
    for char in text.chars() {
        if char == '.' || char == '!' || char == '?' {
            count += 1;
        }
    }
    count
}

/// Calculates the average word length in a text slice.
fn average_word_length(text: &str) -> f64 {
    if text == " " {
        return 0.0;
    }

    let mut words = Vec::new();
    for word in text.split_whitespace() {
        words.push(word);
    }

    if words.is_empty() {
        return 0.0
    }

    let mut total_len = 0;
    for word in words.iter() {
        total_len += word.chars().count();
    }
    total_len as f64 / words.len() as f64
}

/// Finds the `count` most common words using simplified sorting and selection.
fn most_common_words(text: &str, count: usize) -> Vec<(String, usize)> {
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        // Keep this cleaning part as manual alternative is complex/less robust
        let cleaned_word = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if !cleaned_word.is_empty() {
            let entry = word_counts.entry(cleaned_word).or_insert(0);
            *entry += 1;
        }
    }

    let mut sorted_words: Vec<(String, usize)> = Vec::new();
    for (word, freq) in word_counts {
        sorted_words.push((word, freq)); // Collect map entries into a Vec
    }


    // Sort using an expanded closure for clarity
    sorted_words.sort_by(|a, b| {
        // Compare counts descending
        let count_comparison = b.1.cmp(&a.1);
        // If counts are equal, compare words alphabetically ascending
        if count_comparison == Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            count_comparison
        }
    });

    // Take the top `count` elements using slicing
    let final_count = std::cmp::min(count, sorted_words.len());
    // Clone the slice to return an owned Vec
    sorted_words[0..final_count].to_vec()
}


fn main() {
    print!("Enter text: ");
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let trimmed_text = input_text.trim();

    if trimmed_text.is_empty() { println!("\nNo text entered."); return; }

    println!("\nAnalysis:");
    println!("Characters: {}", count_characters(trimmed_text));
    println!("Words: {}", count_words(trimmed_text));
    let mut sentence_count = count_sentences(trimmed_text);
     if sentence_count == 0 && !trimmed_text.is_empty() { sentence_count = 1; } // Handle no terminators
    println!("Sentences: {}", sentence_count);
    println!("Average word length: {:.1}", average_word_length(trimmed_text));
    println!("Most common words:");
    let common_words = most_common_words(trimmed_text, 5);
    if common_words.is_empty() { println!("  (No words found)"); }
    else {
        for (i, (word, count)) in common_words.iter().enumerate() {
            println!("{}. {} ({} times)", i + 1, word, count);
        }
    }
}