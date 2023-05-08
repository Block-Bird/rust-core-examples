fn main() {
    
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &num;  
    println!("R1 is {:?}", *r1); 
    println!("R2 is {:?}", r2); 
    println!("R3 is {:?}", *r3); 


}
