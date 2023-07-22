// Create a code generation macro named generate_struct that takes the name and fields of a struct and generates 
// the corresponding Rust code for it. Use the macro to create a custom struct and use it in your code.

macro_rules! generate_struct {
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* $(,)? }) => {
        struct $struct_name {
            $($field_name: $field_type),*
        }
    };
}

fn main() {
    // Use the macro to generate a custom struct named "Person"
    generate_struct!(Person {
        name: String,
        age: u32,
        is_student: bool,
    });

    // Create an instance of the custom struct and use it
    let person = Person {
        name: String::from("John Doe"),
        age: 30,
        is_student: false,
    };

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Is Student: {}", person.is_student);
}
