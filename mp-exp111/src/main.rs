use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn print_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function's name
    let fn_name = &input.sig.ident;

    // Generate the new function that calls the original function and prints a message
    let output = quote! {
        #input

        fn #fn_name() {
            println!("Calling function: {}", stringify!(#fn_name));
            #fn_name();
            println!("Function call completed: {}", stringify!(#fn_name));
        }
    };

    output.into()
}

// Example usage:

#[print_message]
fn my_function() {
    println!("Inside my_function");
}

fn main() {
    my_function();
}
