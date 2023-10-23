use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Create a shared counter using Arc (atomic reference counting) and Mutex
    let counter = Arc::new(Mutex::new(0));

    // Number of threads to run in parallel
    let num_threads = 4;
    let mut handles = vec![];

    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Lock the mutex to access and update the counter
            let mut data = counter.lock().unwrap();
            *data += 1;
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the mutex to retrieve the final counter value
    let final_value = counter.lock().unwrap();
    println!("Final Counter Value: {}", *final_value);
}
