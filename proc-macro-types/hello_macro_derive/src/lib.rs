extern crate proc_macro; 

use proc_macro::TokenStream; 
use quote::quote; 
use syn; 


#[proc_macro_derive(HelloMacro)]

// passing the token tree into syntax tree
pub fn hello_macro_derive (input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); 

    // Trnasforming that syntax tree
    impl_hello_macro(&ast);

}


fn impl_hello_macro (ast : &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; 
    let gen = quote! {
        impl hello_macro for #name {
            fn HelloMacro ( ) {
                println!(
                    "Hello Macro, My Name is {}!", 
                    stringify!(#name)
                );
            }
        }
    }; 

    gen.into()
}