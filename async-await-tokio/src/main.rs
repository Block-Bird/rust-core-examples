use tokio;

async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://cran.r-project.org/bin/macosx/")
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let result = fetch_data().await;

    match result {
        Ok(data) => println!("Received data: {}", data),
        Err(err) => eprintln!("Error: {}", err),
    }
}
