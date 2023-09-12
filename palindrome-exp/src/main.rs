
use std::io;

fn main(){
    let mut name = String::new();
    println!("Enter your name :");
    
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");


    println!("Name is {}", name); 

    // check if it's palindrome

    // get length of string
    // reverse string and compare
    let reversed : String = name.chars().rev().collect();

    println!("reversed is {}", reversed );

    if name == reversed  {
        println!("Palindrome");
    }
    else {
        println!("Not Palindrome");
    }
 }
