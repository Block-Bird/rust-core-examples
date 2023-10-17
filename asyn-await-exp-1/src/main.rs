use tokio::time::Duration;

async fn async_operations() {
    let future1 = tokio::time::sleep(Duration::from_secs(2));
    let future2 = tokio::time::sleep(Duration::from_secs(3));

    let result1 = future1.await;
    let result2 = future2.await;

    println!("Operation 1 completed: {:?}", result1);
    println!("Operation 2 completed: {:?}", result2);
}

#[tokio::main]
async fn main() {
    async_operations().await;
}
