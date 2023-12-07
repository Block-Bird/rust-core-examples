struct A; 


struct Single(A); 



#[derive(Debug)]

struct SingleGen<T>(T); 



fn main() {
    println!("Hello, world!");

    let _s = Single(A); 

    let _char : SingleGen<char> = SingleGen('a'); 

    let _number : SingleGen<i32> = SingleGen(123); 

    let _string: SingleGen<String> = SingleGen(String::from("My Name is Asad")); 

    println!("String is {:?}", _string); 





}
