// Define a macro that generates getter and setter methods for a struct field.
macro_rules! generate_accessor {
    // The macro takes the name of the field as input.
    ($field_name:ident) => {
        // Generate a getter method.
        fn $field_name(&self) -> &u32 {
            &self.$field_name
        }

        // Generate a setter method.
        fn set_$field_name(&mut self, value: u32) {
            self.$field_name = value;
        }
    };
}

// Define a struct with a field.
struct Person {
    age: u32,
}

// Use the macro to generate getter and setter methods for the 'age' field.
impl Person {
    generate_accessor!(age);
}

fn main() {
    let mut person = Person { age: 30 };

    // Use the generated getter and setter methods.
    println!("Age: {}", person.age());
    person.set_age(35);
    println!("Updated Age: {}", person.age());
}
