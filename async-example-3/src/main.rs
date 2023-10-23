#[tokio::main]
async fn main() {
    let task1 = async_task(1);
    let task2 = async_task(2);

    let (result1, result2) = tokio::join!(task1, task2);

    println!("Task 1 result: {}", result1);
    println!("Task 2 result: {}", result2);
}

async fn async_task(id: i32) -> i32 {
    println!("Task {} started.", id);
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Task {} completed.", id);
    id
}
