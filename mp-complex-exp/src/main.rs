// Define the macro `generate_struct`, which will generate a struct and its associated methods
macro_rules! generate_struct {
    // Base case: No more fields to process
    ($name:ident, $($field:ident : $field_type:ty),*) => {
        // Define the struct
        pub struct $name {
            $(
                $field: $field_type,
            )*
        }

        // Implement getters and setters for each field
        impl $name {
            $(
                pub fn $field(&self) -> $field_type {
                    self.$field
                }

                pub fn set_$field(&mut self, value: $field_type) {
                    self.$field = value;
                }
            )*
        }
    };

    // Recursive case: Process one field and call the macro recursively for the rest
    ($name:ident, $field:ident : $field_type:ty, $($rest:tt)*) => {
        generate_struct!($name, $field : $field_type);
        generate_struct!($name, $($rest)*);
    };
}

// Use the `generate_struct` macro to create a struct with fields and associated methods
generate_struct!(Person, name: String, age: u32);

fn main() {
    let mut person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("Initial values: Name: {}, Age: {}", person.name(), person.age());

    person.set_name("Bob".to_string());
    person.set_age(25);

    println!("Updated values: Name: {}, Age: {}", person.name(), person.age());
}
