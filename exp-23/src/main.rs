// Define a macro named `generate_struct` that takes an identifier (name) and a list of fields
macro_rules! generate_struct {
    // Base case: when there is no field left, generate the struct definition
    ($name:ident { $($field:ident : $field_type:ty),* }) => {
        // Define the struct
        pub struct $name {
            $($field: $field_type),*
        }

        // Implement getter and setter methods for each field
        impl $name {
            $(
                // Getter method
                pub fn $field(&self) -> &$field_type {
                    &self.$field
                }

                // Setter method
                pub fn set_$field(&mut self, value: $field_type) {
                    self.$field = value;
                }
            )*
        }
    };
}

// Use the macro to generate a struct named 'Person' with fields 'name' and 'age'
generate_struct!(Person {
    name: String,
    age: u32,
});

fn main() {
    // Create an instance of the generated struct
    let mut person = Person {
        name: String::from("John"),
        age: 30,
    };

    // Use getter methods
    println!("Name: {}", person.name());
    println!("Age: {}", person.age());

    // Use setter methods
    person.set_name(String::from("Jane"));
    person.set_age(25);

    // Print updated values
    println!("Updated Name: {}", person.name());
    println!("Updated Age: {}", person.age());
}
