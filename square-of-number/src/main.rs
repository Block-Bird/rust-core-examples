macro_rules! square_of_number {
    ($x:expr) => {
        $x * $x 
    };
}

fn main() {

    let number = 5; 
    println!("Square of Number is {:?}", square_of_number!(number));
}
