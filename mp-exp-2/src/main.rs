use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro]
pub fn debuggable(input: TokenStream) -> TokenStream {
    // Parse the input as a Rust syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct being derived for.
    let struct_name = &input.ident;

    // Generate the implementation of the Debug trait.
    let generated = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    let field_names = fields_named.named.iter().map(|field| &field.ident);
                    quote! {
                        impl std::fmt::Debug for #struct_name {
                            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                                f.debug_struct(stringify!(#struct_name))
                                    #(.field(stringify!(#field_names), &self.#field_names))*
                                    .finish()
                            }
                        }
                    }
                }
                Fields::Unnamed(fields_unnamed) => {
                    let field_indices = (0..fields_unnamed.unnamed.len()).map(|i| syn::Index::from(i));
                    quote! {
                        impl std::fmt::Debug for #struct_name {
                            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                                f.debug_tuple(stringify!(#struct_name))
                                    #(.field(&self.#field_indices))*
                                    .finish()
                            }
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        impl std::fmt::Debug for #struct_name {
                            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                                f.debug_struct(stringify!(#struct_name))
                                    .finish()
                            }
                        }
                    }
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => {
            quote! {
                // Only structs are supported for the Debug trait auto-implementation.
                // Enums and unions are not supported.
                compile_error!("Only structs are supported for #[debuggable]");
            }
        }
    };

    // Return the generated implementation as a TokenStream.
    generated.into()
}
