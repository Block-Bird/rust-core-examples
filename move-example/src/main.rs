fn main() {
    let x = vec![1, 2, 3]; 
    let closure = move || println!("This is {:?}", x);
    closure(); 
}