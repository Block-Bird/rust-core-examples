// The basic example of Box 
// fn main() {
//     let b = Box::new(5);
//     println!("B is {}", *b);
// }



// The Main use case for box(recursive)
// Use Vec heap data structure instead of that complex shit 
enum List{
    Cons(i32, List), 
    Nil, 
}

use List::{Cons, Nil};
fn main () {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}