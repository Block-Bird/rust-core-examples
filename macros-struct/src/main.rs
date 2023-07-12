// Define the macro `generate_getters_setters`
macro_rules! generate_getters_setters {
    // Match the macro input with a struct declaration
    (
        $(
            // Match the visibility, struct name, and field type
            $(#[$struct_meta:meta])*
            $vis:vis struct $struct_name:ident {
                $(
                    // Match the field name and type
                    $(#[$field_meta:meta])*
                    $field_vis:vis $field_name:ident: $field_type:ty
                ),* $(,)?
            }
        )*
    ) => {
        $(
            // Implement the getters and setters for each field
            impl $struct_name {
                $(
                    // Generate a getter method for the field
                    $vis fn $field_name(&self) -> &$field_type {
                        &self.$field_name
                    }

                    // Generate a setter method for the field
                    $vis fn $field_name(&mut self, value: $field_type) {
                        self.$field_name = value;
                    }
                )*
            }
        )*
    };
}

// Define a struct and use the macro to generate getters and setters
generate_getters_setters! {
    #[derive(Debug)]
    pub struct Person {
        #[doc = "The person's name"]
        pub name: String,

        #[doc = "The person's age"]
        pub age: u32,
    }
}

fn main() {
    // Create a new instance of the struct
    let mut person = Person {
        name: String::from("John"),
        age: 30,
    };

    // Use the generated getter and setter methods
    println!("Name: {}", person.name());
    println!("Age: {}", person.age());

    person.set_name(String::from("Alice"));
    person.set_age(25);

    println!("Updated Name: {}", person.name());
    println!("Updated Age: {}", person.age());
}
