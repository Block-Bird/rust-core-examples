use std::thread; 
fn main() {

    let x = vec![1, 2, 3]; 
    let closure = move || println!("This is {:?}", x);
    // closure(); 
    // println!("X : {:?}", x); 



    // Example 2
    let x = 5; 
    let closure = move || {
        let result = &x; 
        println!("Result: {}", result);
    }; 

    // closure(); 
    println!(" X is shaa  {:?} ", x);



    // Third Example
    // Importedd Tread Crate
    let mut v = vec![1,2,3];
    thread::spawn( move || {
        v.push(32);
    }); 

    // println!(" V is {:?}", v); 

}