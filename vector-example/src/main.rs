fn main() {

    let v = vec![1,2,3,4,5,6]; 
    let third =  &v[2]; 

    println!("The third Element is {}", third );

    let third = v.get(3); 

    match third {
        Some(third) => println!("The Third Element is {}", third), 
        None => println!("There is nothing on that address"), 
        
    }


    // printing all elemments through for loop

    for i in &v {
        println!("on Index {} ", i ); 
    }

}
