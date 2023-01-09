fn main() {

    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    println!("Address of Var is {}", first);
    v.push(6);
    let number = v.get(5);
    println!("Sixth Index is {:?} ", number);
    v.push(67);
    print!("OK ");
}
