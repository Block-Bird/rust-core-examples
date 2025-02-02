#[tokio::main]
async fn main() {
    let result = async {
        let response = reqwest::get("https://asadalisial.netlify.app/").await?;
        response.text().await
    }
    .await;

    match result {
        Ok(text) => println!("Received text: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
}
