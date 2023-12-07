#[derive(Debug)]
struct A; 


#[derive(Debug)]
struct Single(A); 



#[derive(Debug)]
struct SingleGen<T>(T); 

// Introducing generic Functions

fn concrete_function(s: A ) {
    println!("Printing Concrete Function {:?}", s); 
}


fn single_function (s: Single) {
    println!("Printing the Single A {:?}", s); 
}


fn generic_function(s: SingleGen<i32>) -> SingleGen<i32> {
    // println!("Printing Generic number {:?}", s); 
    s
}


fn main() {

    let number : SingleGen<i32> =  SingleGen(1234); 

    

    println!("Printing generic Number is here {:?}", generic_function(number));

    let _s = Single(A); 

    let _char : SingleGen<char> = SingleGen('a'); 

    let _number : SingleGen<i32> = SingleGen(123); 

    let _string: SingleGen<String> = SingleGen(String::from("My Name is Asad")); 

    println!("String is {:?}", _string); 

}



