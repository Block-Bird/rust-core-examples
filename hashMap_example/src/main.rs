use std::collections::{btree_map::Values, HashMap};


fn main() {

    let mut my_map: HashMap<String, i32> = HashMap::new(); 
    my_map.insert(String::from("Asad Ali"), 32); 
    my_map.insert(String::from("Sial"), 23); 
    my_map.insert(String::from("Here and There1 "), 98); 
    my_map.insert(String::from("Here and There2 "), 98); 
    my_map.insert(String::from("Here and There 3"), 98); 
    my_map.insert(String::from("Here and There 4"), 98); 
    my_map.insert(String::from("Here and There 5"), 98); 
    my_map.insert(String::from("Here and There 6"), 98); 
    my_map.insert(String::from("Here and There 7"), 98); 
    my_map.insert(String::from("Here and There 8"), 98); 
    my_map.insert(String::from("Here and There 9"), 98); 
    my_map.insert(String::from("Here and There 10"), 98); 
    my_map.insert(String::from("Here and There 11"), 98); 
    my_map.insert(String::from("Here and There 12"), 98); 
    my_map.insert(String::from("Here and There 13"), 98); 
    my_map.insert(String::from("Asad Ali 14"), 45); 



    println!("Value of 2 is = {:?} ", my_map.get("Sial")); 


    for (key, Value) in &my_map {
        
        println!("First Key is {:?} and Value is {:?}", key, Value); 

    }

    if (my_map.contains_key("Asad Ali 14")) {
        println!("here's Shaka"); 
    }
    else {
        println!("Shaka Pro Max"); 
    }
    
}
