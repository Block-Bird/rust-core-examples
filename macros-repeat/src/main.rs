macro_rules! find_min {
    ($x: expr) => ($x); 
    ($x: expr, $($y: expr), +) => {
        std::cmp::min($x, find_min!($($y), + ))
    }
}

fn main() {
    println!("{}", find_min!(156487, 9, 7968,6578,7986,567));
    let result = add(2, 3);
    println!("Result: {}", result);

    let result = sub(5, 2);
    println!("Result: {}", result);


}


macro_rules! add_func {
    () => (
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    );
}

add_func!();

macro_rules! sub_func {
    () => (
        fn sub(a: i32, b: i32) -> i32 {
            a - b
        }
    );
}

sub_func!();