use std::error::Error;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://jsonplaceholder.typicode.com/users/1";
    let mut stream = TcpStream::connect("jsonplaceholder.typicode.com:443").await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: jsonplaceholder.typicode.com\r\n\r\n", url);

    // Write the request to the server
    let mut writer = BufWriter::new(&mut stream);
    writer.write_all(request.as_bytes()).await?;

    // Read the response from the server
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).await?;

    // Deserialize the response into a User struct
    let user: User = serde_json::from_str(&buffer)?;

    // Print the user's details
    println!("User ID: {}\nName: {}\nEmail: {}", user.id, user.name, user.email);

    // Set up a Tokio channel to send a message after a delay
    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async move {
        let delay = time::sleep(Duration::from_secs(5));
        delay.await;
        tx.send("Done").await.unwrap();
    });

    // Wait for the channel message to arrive
    let message = rx.recv().await.unwrap();
    println!("Message received: {}", message);

    Ok(())
}
