// Define a macro to generate multiple structs with similar fields
macro_rules! define_structs {
    // Base case: no more structs to define
    () => {};

    // Define a struct with the given name and fields,
    // and recursively call the macro for the rest of the structs
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* } $($rest:tt)*) => {
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }

        // Recursively call the macro for the rest of the structs
        define_structs!($($rest)*);
    };
}

// Use the macro to define multiple structs
define_structs! {
    Person {
        name: String,
        age: u32,
    }

    Company {
        name: String,
        employees: u32,
    }

    Car {
        make: String,
        model: String,
        year: u32,
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let company = Company {
        name: "Acme Corp".to_string(),
        employees: 100,
    };

    let car = Car {
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2022,
    };

    // Do something with the generated structs
}
