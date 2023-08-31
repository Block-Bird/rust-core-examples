// Define the trait
trait DebugPrint {
    fn debug_print(&self);
}

// Implement the trait for i32
impl DebugPrint for i32 {
    fn debug_print(&self) {
        println!("Value: {}", self);
    }
}

// Implement the trait for f64
impl DebugPrint for f64 {
    fn debug_print(&self) {
        println!("Value: {:.2}", self);
    }
}

// Macro to implement the DebugPrint trait for any type that implements Debug trait
macro_rules! impl_debug_print {
    ($t:ty) => {
        impl DebugPrint for $t {
            fn debug_print(&self) {
                println!("DebugPrint for {}", stringify!($t));
            }
        }
    };
}

// Use the macro to implement the trait for String
impl_debug_print!(String);

fn main() {
    let number = 42;
    number.debug_print();

    let float_number = 3.14159;
    float_number.debug_print();

    let text = String::from("Hello, Rust!");
    text.debug_print();
}
