use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    const NUM_THREADS: usize = 4;
    const NUM_VALUES: usize = 1000000;
    
    // Create a vector of values from 1 to NUM_VALUES
    let values: Vec<u64> = (1..=NUM_VALUES as u64).collect();
    
    // Create shared data structures for parallel processing
    let values_shared = Arc::new(Mutex::new(values));
    let result_shared = Arc::new(Mutex::new(0u64));
    
    // Create channels for communication between threads
    let (tx, rx) = mpsc::channel();
    
    // Spawn NUM_THREADS threads
    for _ in 0..NUM_THREADS {
        let values_shared = values_shared.clone();
        let result_shared = result_shared.clone();
        let tx = tx.clone();
        
        thread::spawn(move || {
            let mut sum = 0u64;
            
            // Process a subset of values
            loop {
                let value = {
                    let mut values = values_shared.lock().unwrap();
                    match values.pop() {
                        Some(val) => val,
                        None => break,
                    }
                };
                sum += value * value;
            }
            
            // Send the result back through the channel
            tx.send(sum).unwrap();
        });
    }
    
    // Receive results from threads and update the final sum
    for _ in 0..NUM_THREADS {
        let partial_sum = rx.recv().unwrap();
        let mut result = result_shared.lock().unwrap();
        *result += partial_sum;
    }
    
    // Get the final sum
    let result = result_shared.lock().unwrap();
    println!("Sum of squares from 1 to {} is: {}", NUM_VALUES, *result);
}
