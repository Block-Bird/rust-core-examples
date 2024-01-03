use std::collections::HashMap;

fn main() {
    // Create a new HashMap with keys and values of type String
    let mut my_map: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs into the HashMap
    my_map.insert(String::from("Alice"), 25);
    my_map.insert(String::from("Bob"), 30);
    my_map.insert(String::from("Charlie"), 22);

    // Access values using the keys
    println!("Bob's age: {:?}", my_map.get("Bob"));

    // Update a value
    my_map.insert(String::from("Alice"), 26);

    // Iterate over the HashMap
    for (key, value) in &my_map {
        println!("{} is {} years old", key, value);
    }

    // Check if a key exists
    let key_to_check = String::from("Dave");
    if my_map.contains_key(&key_to_check) {
        println!("{} is in the HashMap", key_to_check);
    } else {
        println!("{} is not in the HashMap", key_to_check);
    }

    // Remove a key-value pair
    let removed_value = my_map.remove("Charlie");
    match removed_value {
        Some(age) => println!("Removed Charlie, who is {} years old", age),
        None => println!("Charlie is not in the HashMap"),
    }

    // Clear the HashMap
    my_map.clear();
    println!("HashMap after clearing: {:?}", my_map);

    // Insert more key-value pairs
    my_map.insert(String::from("Eve"), 28);
    my_map.insert(String::from("Frank"), 35);

    // Handle a potential error when inserting a duplicate key
    if let Some(old_value) = my_map.insert(String::from("Eve"), 29) {
        println!("Eve's age was {}, updated to 29", old_value);
    } else {
        println!("Eve was not in the HashMap, inserted with age 29");
    }

    // Use entry API to update or insert
    my_map.entry(String::from("Grace")).or_insert(40);

    // Print the final HashMap
    println!("Final HashMap: {:?}", my_map);
}
