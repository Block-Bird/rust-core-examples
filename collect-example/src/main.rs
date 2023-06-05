// use std::collections; 

fn main()
{
static  A : [i32; 3] = [1, 2, 3];

let doubled : Vec<i32 > = A.iter().map(|&x| x*2 ).collect();

assert_eq!(vec![2,4,6], doubled);

println!("{:?}", doubled); 

}