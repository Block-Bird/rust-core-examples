use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key:expr => $value:expr ),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

fn main() {
    // Create a HashMap using the hashmap! macro
    let my_map = hashmap![
        "apple" => 1,
        "banana" => 2,
        "orange" => 3,
    ];

    // Access and print values from the HashMap
    println!("Value for 'apple': {:?}", my_map.get("apple")); // Some(1)
    println!("Value for 'banana': {:?}", my_map.get("banana")); // Some(2)
    println!("Value for 'grape': {:?}", my_map.get("grape")); // None
}
