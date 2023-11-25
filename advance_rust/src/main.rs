// Procedural macro example
// use proc_macro;

#[proc_macro]
pub fn generate_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Input is the code inside the macro invocation
    let input_str = input.to_string();

    // Generating new code
    let output = format!("fn generated_function() {{ println!(\"Generated: {}\"); }}", input_str);

    // Convert the generated code back into a TokenStream
    output.parse().unwrap()
}
// Using the procedural macro
// 

fn main() {
    // Calling the generated function
    // generated_function();
    generate_function!(Hello, world!);
}
