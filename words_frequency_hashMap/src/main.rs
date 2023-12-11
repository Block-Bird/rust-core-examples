use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} dummy.txt", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // Read the file line by line and tokenize words
    let mut word_freq_map: HashMap<String, usize> = HashMap::new();

    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                process_line(&line, &mut word_freq_map);
            }
        }

        // Display the top N words and their frequencies
        display_top_words(&word_freq_map, 10); // Change 10 to your desired N
    } else {
        eprintln!("Error reading the file: {}", filename);
        std::process::exit(1);
    }
}

fn process_line(line: &str, word_freq_map: &mut HashMap<String, usize>) {
    let words: Vec<&str> = line.split_whitespace().collect();

    for word in words {
        // Remove common punctuation and convert to lowercase
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();

        // Update the frequency in the hash map
        let counter = word_freq_map.entry(cleaned_word).or_insert(0);
        *counter += 1;
    }
}

fn display_top_words(word_freq_map: &HashMap<String, usize>, n: usize) {
    // Sort the map by frequency in descending order
    let mut sorted_words: Vec<_> = word_freq_map.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1));

    // Display the top N words and their frequencies
    println!("Top {} words and their frequencies:", n);
    for (i, (word, freq)) in sorted_words.iter().take(n).enumerate() {
        println!("{}. {}: {}", i + 1, word, freq);
    }
}
