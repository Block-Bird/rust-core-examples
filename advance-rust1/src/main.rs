// used for parallel execution
use std::thread;

// used for shared ownership
use std::sync::{Arc, Mutex};

fn main() {
    // Create a shared counter using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc for each thread
        let counter = Arc::clone(&counter);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Access and modify the counter within the thread
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}