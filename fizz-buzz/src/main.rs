use std::io;


fn main() {
    print!("Enter the ending number = ");

    for i in 0..100 {
        if i%3 == 0  {
            println!("Fizz");
        }
        else if i%5 == 0 {
           println!("Buzz"); 
        }
        else 
        {
            println!("{}", i);
        }
    }

}
