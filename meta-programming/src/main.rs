macro_rules! generate_functions {
    ($func_name:ident, $return_type:ty, $arg_type:ty) => {
        fn $func_name(arg: $arg_type) -> $return_type {
            // function body
        }
    };
}

generate_functions!(add_integers, i32, (i32, i32));
generate_functions!(add_floats, f32, (f32, f32));

fn main() {
    let sum = add_integers((1, 2));
    let result = add_floats((1.0, 2.0));
    println!("sum: {}", sum);
    println!("result: {}", result);
}