extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn make_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree.
    let ast = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct and the fields.
    let name = &ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields are supported."),
        },
        _ => panic!("Only structs are supported."),
    };

    // Generate the getters and setters for each field.
    let mut getters_setters = quote! {};

    for field in fields {
        let field_name = &field.ident;
        let ty = &field.ty;
        let getter = quote! {
            pub fn #field_name(&self) -> &#ty {
                &self.#field_name
            }
        };
        let setter = quote! {
            pub fn #field_name(&mut self, value: #ty) {
                self.#field_name = value;
            }
        };

        getters_setters = quote! {
            #getters_setters
            #getter
            #setter
        };
    }

    // Generate the final struct implementation with getters and setters.
    let expanded = quote! {
        #ast

        impl #name {
            #getters_setters
        }
    };

    TokenStream::from(expanded)
}


use my_macro::make_struct;

// Apply the procedural macro to the struct.
#[make_struct]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("Name: {}", person.name());
    println!("Age: {}", person.age());

    person.set_name("Bob".to_string());
    person.set_age(25);

    println!("Name: {}", person.name());
    println!("Age: {}", person.age());
}
