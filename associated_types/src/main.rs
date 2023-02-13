use std::{fmt, hash::Hash};

pub trait CacheableItem: Clone + Default + fmt::Debug {
    type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    fn is_null(&self) -> bool;

}

struct Container (i32, i32); 

trait Contains <A, B> {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool;
    fn first(&self) -> i32;
    fn last (&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&number_1 == &number_2)
    }

    fn first (&self) -> i32 {
        self.0
    }

    fn last (&self) -> i32 {
        self.1
    }
}

fn difference <A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main () {
    let number_1 = 3;
    let number_2 = 10; 
    let container = Container (number_1, number_2);
    println!("Does container contain {}, {}: {}", &number_1, &number_2, container.contains(&number_1, &number_2));

    println!("First Number is {}", &number_1);
    println!("Second Number is {}", &number_2);
    println!("The Difference is {} ", difference(&container));

}