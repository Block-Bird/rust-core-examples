macro_rules! find_min {
    ($x: expr) => ($x); 
    ($x: expr, $($y: expr), +) => {
        std::cmp::min($x, find_min!($($y), + ));
    }
}

fn main() {
    println!("{}", find_min!(156487, 9, 7968,6578,7986,567));
}
