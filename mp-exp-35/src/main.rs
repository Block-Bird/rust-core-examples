// Define a struct named 'Person'
struct Person {
    name: String,
    
    age: u32,
}

// Implement methods for the 'Person' struct
impl Person {
    // Constructor method to create a new Person instance
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    // Method to get the person's name
    fn get_name(&self) -> &str {
        &self.name
    }

    // Method to get the person's age
    fn get_age(&self) -> u32 {
        self.age
    }

    // Method to wish the person a happy birthday
    fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday, {}! You are now {} years old.", self.name, self.age);
        println!("Happy Birthday, {}! You are now {} years old.", self.name, self.age);

        // println!("Happy Birthday, {}! You are now {} years old.", self.name, self.age);

    }
}

fn main() {
    // Create a new Person instance
    let mut person = Person::new(String::from("Alice"), 30);

    // Access and print the person's name and age
    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    // Celebrate the person's birthday
    person.celebrate_birthday();
    
}
