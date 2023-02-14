fn main() {
    println!("Hello, world!");
}


// Function-like procedural macros
#[proc_macro]
pub fn foo(body: String) -> String { 
    body
}
foo!("Here and there"); 



// Custom derive procedural macros
#[proc_macro_derive(Bar)]
pub fn bar(body: TokenStream) -> TokenStream { ... }

#[derive(Bar)]
struct S;

Custom attributes
#[proc_macro_attribute]
pub fn baz (attr: TokenStream, 
item : TokenStream ) -> TokenStream {

}

#[baz]
fn some_item() {
    
}