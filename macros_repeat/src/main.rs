macro_rules! repeat_n_times {
    ($expr:expr, $n:expr) => {
        {
            for _ in 0..$n {
                println!("{}", $expr);
            }
        }
    };
}


macro_rules! repeat_m_times {
    ($name:expr , $number:expr) => {
        for _ in 0..$number  {
            println!("{}", $number);
        }
    };
}

fn main() {
    repeat_n_times!("Rust is awesome!", 5);
    repeat_m_times!("M times = ", 56);
}


