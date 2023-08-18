use proc_macro;

// Define a procedural macro called `hello_macro`
#[proc_macro]
pub fn hello_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Generate new code using the input and manipulate it
    let output = format!("println!(\"Hello from macro: {}\");", input);

    // Convert the generated code back into a token stream
    output.parse().unwrap()
}


use hello_macro::hello_macro;

#[hello_macro]
fn main() {
    let message = "world";
    println!("Hello, {}!", message);
}

