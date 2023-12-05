use std::sync::mpsc;
use std::thread;



fn main() {

    // creating a new channel
    // sender and receiver
    let (tx, rx) = mpsc::channel(); 

    thread::spawn( move || {
        let val = String::from("Hi"); 
        tx.send(val).unwrap(); 
    });


    let receiver = rx.recv().unwrap(); 
    println!("Received Data is {} ", receiver); 

}
