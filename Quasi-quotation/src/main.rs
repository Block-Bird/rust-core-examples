macro_rules! vec_strs {
    (
        $(
            $element:expr
        ),*
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

fn main() {
    let v: Vec<String> = vec_strs![1, 2, "three", 4.0];
    println!("{:?}", v);  // Prints ["1", "2", "three", "4"]
}
