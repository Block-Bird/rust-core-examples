use std::sync::mpsc;
use std::thread;
use std::time::Duration;



fn main() {

    // creating a new channel
    // sender and receiver
    let (tx, rx) = mpsc::channel(); 

    thread::spawn( move || {

        let vals = vec![
            String::from("hi"), 
            String::from("From"), 
            String::from("the"), 
            String::from("Thread"),
        ];


        for val in vals {
            tx.send(val).unwrap(); 
            thread::sleep(Duration::from_secs(1)); 
        }

    });


    for i in rx {
        println!("Received Data is {} ", i); 
    }

}
