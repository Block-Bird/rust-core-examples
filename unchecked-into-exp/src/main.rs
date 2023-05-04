use std::mem;
fn main() {
    let x: u32 = 42; 
    let y: f32 = unsafe{mem::transmute_copy(&x)}; 

    println!("Printing X {:?} ", x); 
    println!("Printing Y {:?} ", y); 

}
