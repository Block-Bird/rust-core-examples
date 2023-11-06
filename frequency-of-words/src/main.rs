use std::collections::HashMap;

fn main() {
    let paragraph = "This is a sample paragraph. This paragraph contains sample words.";

    let mut word_frequencies: HashMap<String, u32> = HashMap::new();

    // Split the paragraph into words and iterate over them
    for word in paragraph.split_whitespace() {
        // Normalize the word to lowercase for case-insensitive counting
        let normalized_word = word.to_lowercase();

        // Update word frequencies in the HashMap
        let count = word_frequencies.entry(normalized_word).or_insert(0);
        *count += 1;
    }

    // Print the word frequencies
    for (word, count) in &word_frequencies {
        println!("Word: '{}', Frequency: {}", word, count);
    }
}
