macro_rules! calculate {
    // Base case: Single numeric value
    ($value:literal) => {
        $value
    };
    // Addition
    ($($rest:tt)+) => {
        {
            let mut sum = 0;
            $(
                sum += calculate!($rest);
            )+
            sum
        }
    };
    // Subtraction
    ($first:literal - $($rest:tt)*) => {
        $first - calculate!($($rest)*)
    };
    // Multiplication
    ($($rest:tt)*) => {
        {
            let mut product = 1;
            $(
                product *= calculate!($rest);
            )*
            product
        }
    };
}

fn main() {
    // Define a complex mathematical expression
    let result = calculate!(10 + 2 * 5 - 4 / 2 + 1);

    println!("Result: {}", result); // Output: Result: 19
}
