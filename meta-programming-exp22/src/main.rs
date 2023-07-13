use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;


#[proc_macro_attribute]
pub fn my_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code using the `quote` crate
    let output = quote! {
        // ... Generate your code here ...
    };

    // Return the generated code as a token stream
    output.into()
}

#[proc_macro_derive(MyTrait)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code using the `quote` crate
    let output = quote! {
        // ... Generate your code here ...
    };

    // Return the generated code as a token stream
    output.into()
}

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let input = parse_macro_input!(input as syn::LitInt);

    // Extract the integer value from the input
    let value = input.base10_parse::<u32>().unwrap();

    // Generate code using the `quote` crate
    let output = quote! {
        const VALUE: u32 = #value;
    };

    // Return the generated code as a token stream
    output.into()
}