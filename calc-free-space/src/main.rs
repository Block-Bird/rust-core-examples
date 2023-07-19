use std::fs;

fn main() {
    // Get the free space information for the current directory
    let space = fs::space(".").expect("Failed to retrieve space information.");

    // Print the free space
    println!("Free space: {} bytes", space.free);
    println!("Total space: {} bytes", space.total);
    println!("Available space: {} bytes", space.avail);
}