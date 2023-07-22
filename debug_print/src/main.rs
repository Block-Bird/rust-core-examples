// Implement a macro named debug_print that takes an expression and prints 
// both the expression and its value to the console. Use the macro to debug some variables in your code.

macro_rules! debug_print {
    ($name:expr, $rollNum:expr) => {
        println!("One is {} and Two is  {}", $name , $rollNum);
    };
}

fn main() {
    println!("Hello, world!");
    debug_print!(22, 89); 
}
