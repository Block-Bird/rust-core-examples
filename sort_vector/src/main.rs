#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String, 
    age: u32, 
}

impl Person {
    pub fn new (name: String, age: u32) -> Self {
        Person {
            name, 
            age,
        }
    }
}

fn main() {
    
    let mut people = vec![
        Person::new("asad".to_string(), 32),
        Person::new("Ali".to_string(), 45), 
        Person::new("Sial".to_string(), 12), 
        Person::new("Zafar".to_string(), 62)
    ];

    people.sort(); 

    println!("People are {:?}", people);  
    
}
