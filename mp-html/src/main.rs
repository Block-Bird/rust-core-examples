macro_rules! generate_methods {
    ($struct_name:ident, $($method:ident),*) => {
        impl $struct_name {
            $(fn $method(&self) {
                println!("Calling {} method", stringify!($method));
            })*
        }
    };
}

struct MyStruct;

generate_methods!(MyStruct, method1, method2, method3);

fn main() {
    let instance = MyStruct;
    instance.method1();
    instance.method2();
    instance.method3();
}
