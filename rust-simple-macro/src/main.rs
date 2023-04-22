macro_rules! print_nested {
    // The base case: print out a single value.
    ($val:expr) => {
        println!("{}", $val);
    };
    // The recursive case: print out a nested list of values.
    ($($x:expr),+) => {
        println!("{{");
        $(
            print_nested!($x);
        )+
        println!("}}");
    };
}

fn main() {
    let nested_list = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // print_nested!(nested_list);
}