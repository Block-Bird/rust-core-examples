pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
// Here is a simple example of how to implement the future trait:

struct  MyCounterFuture {
	cnt : u32,
	cnt_final : u32
}

impl MyCounterFuture {
	pub fn new(final_value : u32) -> Self {
		Self {
			cnt : 0,
			cnt_final : final_value
		}
	}
}
 
impl Future for MyCounterFuture {
	type Output = u32;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32>{
		self.cnt += 1;
		if self.cnt >= self.cnt_final {
			println!("Counting finished");
			return Poll::Ready(self.cnt_final);
		}

		cx.waker().wake_by_ref();
		Poll::Pending
	}
}

#[tokio::main]
async fn main(){
	let my_counter = MyCounterFuture::new(42);

	let final_value = my_counter.await;
	println!("Final value: {}", final_value);
}