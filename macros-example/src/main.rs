macro_rules! print_and_return {
    ($value:expr) => {
        println!("The value is: {}", $value);
        $value
    };
}

fn main() {
    let x = 42;
    let y = print_and_return!(x + 8);
    println!("The result is: {}", y);
}