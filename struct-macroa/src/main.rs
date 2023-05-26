macro_rules! create_struct {
    ($struct_name:ident { $($field_name:ident: $field_type:ty),* }) => {
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $(
                        $field_name,
                    )*
                }
            }
        }
    };
}

create_struct!(Person {
    name: String,
    age: u32,
    email: String,
});

fn main() {
    let person = Person::new(
        String::from("John Doe"),
        30,
        String::from("john.doe@example.com"),
    );

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Email: {}", person.email);
}
