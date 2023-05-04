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
}