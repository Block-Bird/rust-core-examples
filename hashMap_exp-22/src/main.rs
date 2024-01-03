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
}
