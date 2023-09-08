macro_rules! vec_strs {
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(String::from($x));
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec_strs!["one", "two", "three"];
    println!("{:?}", v);
}
