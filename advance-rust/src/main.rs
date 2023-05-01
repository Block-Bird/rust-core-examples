use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq)]

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let mut people: HashSet<Person> = HashSet::new();
    let mut age_map: HashMap<u32, Vec<&str>> = HashMap::new();
    
    let p1 = Person::new("Alice", 25);
    let p2 = Person::new("Bob", 30);
    let p3 = Person::new("Charlie", 35);
    
    people.insert(p1);
    people.insert(p2);
    people.insert(p3);
    
    for person in &people {
        let name = &person.name;
        let age = person.age;
        age_map.entry(age).or_insert(vec![]).push(name);
    }
    
    for (age, names) in &age_map {
        println!("Age {}: {:?}", age, names);
    }
}