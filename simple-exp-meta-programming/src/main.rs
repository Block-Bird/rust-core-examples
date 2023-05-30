macro_rules! generate_functions {
    ($($name:ident),*) => {
        $(
            fn $name() {
                println!("This is the {} function.", stringify!($name));
            }
        )*
    };
}

generate_functions!(foo, bar, baz);

fn main() {
    foo();
    bar();
    baz();
}
