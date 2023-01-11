fn main () {
    let mut v = vec![1,2,3];
    // let add = &v; 
    let mut v2 = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    // println!("Before V is {}", &v[0]);
    println!("Before V2 is {}", *v2[0]);
    // println!("Before V2 is {:?}", v);
    *v2[0] = 5; 
    let a = *v2[0];
    let b= v [0];
    println!("{a} {b}");
}