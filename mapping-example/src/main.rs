use std::collections::HashMap;

fn main() {
    // Create a new empty HashMap with keys of type String and values of type i32.
    let mut scores = HashMap::new();

    // Insert key-value pairs into the HashMap.
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 90);
    scores.insert(String::from("Charlie"), 85);

    // Access values using keys.
    let alice_score = scores.get(&String::from("Alice"));

    // Check if a key exists in the HashMap.
    if let Some(score) = alice_score {
        println!("Alice's score: {}", score);
    } else {
        println!("Alice's score not found.");
    }

    // Iterate over the HashMap.
    for (name, score) in &scores {
        println!("{} has a score of {}", name, score);
    }

    // Update a value in the HashMap.
    let new_score = scores.entry(String::from("Alice")).or_insert(95);
    println!("Updated Alice's score: {}", new_score);

    // Remove a key-value pair from the HashMap.
    scores.remove(&String::from("Charlie"));

    // Check if a key exists after removal.
    if scores.contains_key(&String::from("Charlie")) {
        println!("Charlie's score found.");
    } else {
        println!("Charlie's score not found.");
    }
}
