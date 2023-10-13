use std::thread;

fn main() {
    // Define a large list of numbers.
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let num_threads = 4; // Number of threads to use

    // Split the list into chunks for parallel processing.
    let chunk_size = numbers.len() / num_threads;
    let mut handles = vec![];
    let mut results = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            numbers.len()
        } else {
            (i + 1) * chunk_size
        };

        // Clone a portion of the list for each thread.
        let chunk = numbers[start..end].to_vec();

        // Spawn a thread to calculate the sum of the chunk.
        let handle = thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            sum
        });

        handles.push(handle);
    }

    // Collect the results from each thread.
    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }

    // Calculate the final sum by combining the results.
    let final_sum: i32 = results.iter().sum();

    // Print the final sum.
    println!("Final Sum: {}", final_sum);
}
