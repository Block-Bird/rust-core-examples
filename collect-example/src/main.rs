// use std::collections; 

fn main()
{
// static  A : [i32; 3] = [1, 2, 3];

// let doubled : Vec<i32 > = A.iter().map(|&x| x*2 ).collect();

// assert_eq!(vec![2,4,6], doubled);

// println!("{:?}", doubled); 


let numbers = vec![1, 2, 3, 4 ]; 
let squared_numbers= numbers.iter().map(|x| x*x); 
println!("Here's squared Number {:?}", squared_numbers);
let collected_number : Vec<i32>  = squared_numbers.collect(); 
println!("Here're Collected Numbers {:?}", collected_number); 

}