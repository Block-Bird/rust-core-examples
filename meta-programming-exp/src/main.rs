use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &ast.ident;

    // Generate the implementation code
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}


#[derive(HelloMacro)]
struct MyStruct;

fn main() {
    MyStruct::hello_macro(); // Prints: "Hello, Macro! My name is MyStruct"
}
