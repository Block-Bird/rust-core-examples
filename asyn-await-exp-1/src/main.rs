use tokio::time::Duration;
extern crate tokio;


async fn async_operations() {
    println!("Start");
    let future1 = tokio::time::sleep(Duration::from_secs(5));
    println!("5 Sec wait");
    let future2 = tokio::time::sleep(Duration::from_secs(30));
    println!("30 Sec Wait");
    let result1 = future1.await;
    let result2 = future2.await;

    println!("Operation 1 completed: {:?}", result1);
    
    println!("Operation 2 completed: {:?}", result2);
}

#[tokio::main]
async fn main() {
    async_operations().await;
}
