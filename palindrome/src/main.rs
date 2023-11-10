// check Palindrome or not ?

use std::io;

fn main() {


    println!("Enter a String = ");
    let mut pal = String::new();
    io::stdin().read_line(&mut pal).expect("Failed to read Line");
    let input = pal.trim();

    if is_palindrome (input) {
        println!("The String is Palindrome");
    }
    else {
        println!("String is not Palindrome");
    }

}

fn is_palindrome(s : &str) -> bool {
    let reversed_string : String = s.chars().rev().collect();
    s== reversed_string
}