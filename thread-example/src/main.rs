use std::sync::mpsc;
use std::thread;



fn main() {

    // creating a new channel
    // sender and receiver
    let (tx, rx) = mpsc::channel(); 

    thread::spawn( move || {
        let val = String::from("Hi"); 

        let temp = &val; 
        tx.send(temp).unwrap(); 
        println!("The Value {} is sent ", val);
    });

}
