// Define a macro to generate a simple struct with given fields
macro_rules! generate_struct {
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* }) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field_name: $field_type),*
        }
    };
}

// Use the macro to generate a struct named "Person" with fields "name" and "age"
generate_struct!(Person {
    name: String,
    age: u32,
});




fn main() {
    // Create an instance of the generated "Person" struct
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };

    // Print the struct using the derived Debug trait
    println!("{:?}", person);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
