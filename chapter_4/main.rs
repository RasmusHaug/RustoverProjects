use std::{io, usize};

fn main() {
    println!("Type characters to iterate until word is found:\n");
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    let fw = first_word(&word);
    println!("First word found: {}", fw);

    let ranges = second_word(&word);
    for (start, end) in ranges {
        println!("Second word found: {}", &word[start..end]);
    }
    example();

    // `str` uses up more memory than `String` but this is offset because `str` can be used better
    // as a slice.
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

fn first_word(s: &String) -> &str {
    // Convert the string to bytes.
    let bytes = s.as_bytes();
    // Using {:?} in order to print the bytes in DEBUG format.
    // println!(
    //     "String as bytes: {:?}. Space Byte value: {}, 't' in bytes: {}",
    //     bytes, b' ', b't'
    // );

    // iterate over the bytes of text.
    for (i, &item) in bytes.iter().enumerate() {
        // println!("Iteration: {}, Item: {}", i, item);
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> Vec<(usize, usize)> {
    // ONLY prints out the second word!
    // Different way to complete the task. `first_word` is by far simpler and easier to understand
    // what is happening.
    let bytes: &[u8] = s.as_bytes();
    let mut ranges = Vec::new();
    let mut start_index: Option<usize> = None;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // If we find a space and whe have a start index, we conclude the previous word.
            if let Some(start) = start_index {
                // Push range for the word.
                ranges.push((start, i));
                // Reset start index for the next word.
                start_index = None;
            } else {
                // If we find a non-space character and we don't have a start index, set it.
                if start_index.is_none() {
                    start_index = Some(i);
                }
            }
        }
        // handle the last word if there is no trailing space.
    }
    if let Some(start) = start_index {
        // Push range for the last word.
        ranges.push((start, bytes.len()));
    }

    ranges
}

fn example() {
    let bytes = b"Hello World"; // A byte string
    println!("\nRunning example code for bytes using String 'Hello World'.\nIterating through the bytes using `iter().enumerate()` that returns `u8` bytes.\nByte represantation: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        println!("Iteration: {}, Item: {}", i, item); // item is a u8 here
    }
}
