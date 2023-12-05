// get a vector with strings and numbers and then print it while ysing loops

use core::num;



fn main() {

    let number = vec![1,234,23,4,234,345,345,456,456,45,645,64,56,567,56,776,876,867,8,678,678,78,56,345,32,4]; 


    // for i in number {
    //     println!("Number is {}", &i);
    // }

    for (i , value) in number.iter().enumerate() {
        println!("number is {} and Value is {}", i, value); 
    }

}
