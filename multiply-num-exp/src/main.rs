macro_rules! multiply {
    // The base case for multiplying one number by another is simply to
    // return their product.
    ($x:expr , $y:expr) => {{
        $x * $y
    }};
    // For multiplying more than two numbers, we recursively call the
    // macro with the first two numbers, then multiply their result by
    // the remaining numbers.
    ($x:expr,* $($y:expr)*) => {{
        multiply!($x * multiply!($($y)*))
    }};
}

fn main() {
    let x = 2;
    let y = 3;
    let z = 4;

    // let result = multiply!(x * y * z);
    let result = 10; 
    println!("{}", result); // prints "24"
}
