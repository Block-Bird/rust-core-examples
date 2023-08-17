use std::thread;

fn main() {
    // Create a vector to hold thread handles
    let mut handles = vec![];

    for i in 0..5 {
        // Spawn a new thread and pass it a closure
        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            for j in 0..3 {
                println!("Thread {}: Count {}", i, j);
            }
            println!("Thread {} finished", i);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished.");
}
