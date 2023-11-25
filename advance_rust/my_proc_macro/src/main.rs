use my_proc_macro::generate_function;

generate_function!(Hello, world!);

fn main() {
    // Calling the generated function
    generated_function();
}
