// Define a macro that generates getter methods for a struct
macro_rules! generate_getters {
    // Entry point for the macro, takes the struct name and field names as arguments
    ($struct_name:ident, $($field:ident),*) => {
        // For each field, generate a getter method
        $(
            // Getter method for the field
            fn $field(&self) -> &i32 {
                &self.$field
            }
        )*
    };
}

// Define a macro that generates setter methods for a struct
macro_rules! generate_setters {
    // Entry point for the macro, takes the struct name and field names as arguments
    ($struct_name:ident, $($field:ident),*) => {
        // For each field, generate a setter method
        $(
            // Setter method for the field
            fn $field(&mut self, value: i32) {
                self.$field = value;
            }
        )*
    };
}

// Define a struct with fields
struct Person {
    name: String,
    age: i32,
}

// Use the macros to generate getters and setters for the struct
generate_getters!(Person, name, age);
generate_setters!(Person, name, age);

fn main() {
    // Create a person instance
    let mut person = Person {
        name: String::from("John"),
        age: 30,
    };

    // Use the generated getter methods
    println!("Name: {}", person.name());
    println!("Age: {}", person.age());

    // Use the generated setter methods
    person.set_name(String::from("Alice"));
    person.set_age(25);

    // Print the updated values
    println!("Updated Name: {}", person.name());
    println!("Updated Age: {}", person.age());
}
