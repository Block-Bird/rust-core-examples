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

macro_rules! fiveTime {
    ($x: expr) => {
        $x * 5
    };

}

fn main() {
    let v = vec_strs!["one", "two", "three"];
    println!("{:?}", v);

    println!("Five time is {:?}", fiveTime!(5*2+1));
}
