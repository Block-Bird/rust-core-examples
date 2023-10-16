use std::thread;

fn main() {
    // Spawn two threads
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 1: Count {}", i);
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 2: Count {}", i);
        }
    });

    // Wait for threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Both threads have finished.");
}
