macro_rules! greet {
    () => {
        println!("Hello, world!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!();                 // Prints: Hello, world!
    greet!("John");           // Prints: Hello, John!
}