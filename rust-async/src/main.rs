use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open a file asynchronously
    let file_path = "/Users/asadali/Rust-Projects/rust-examples/rust-async/src/main.rs";
    let mut file = File::open(file_path).await?;

    // Create a buffer to read the file content into
    let mut buffer = Vec::new();

    // Read the file asynchronously and store its content in the buffer
    file.read_to_end(&mut buffer).await?;

    // Convert the buffer to a string and print its content
    let file_contents = String::from_utf8_lossy(&buffer);
    println!("File content:\n{}", file_contents);

    Ok(())
}
