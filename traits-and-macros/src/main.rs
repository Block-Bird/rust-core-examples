// Define a trait
trait Print {
    fn print(&self);
}

// Implement the Print trait for i32
impl Print for i32 {
    fn print(&self) {
        println!("Printing i32: {}", self);
    }
}

// Implement the Print trait for String
impl Print for String {
    fn print(&self) {
        println!("Printing String: {}", self);
    }
}

// Define a macro that accepts an expression implementing the Print trait
macro_rules! print_value {
    ($value:expr) => {
        $value.print();
    };
}

fn main() {
    let x: i32 = 10;
    let y: String = String::from("Hello, world!");

    // Call the macro with different values
    print_value!(x);
    print_value!(y);
}
