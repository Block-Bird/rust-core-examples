use proc_macro;

#[proc_macro]
pub fn generate_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_str = input.to_string();
    let output = format!("fn generated_function() {{ println!(\"Generated: {}\"); }}", input_str);
    output.parse().expect("Failed to parse the generated code as TokenStream")
}
