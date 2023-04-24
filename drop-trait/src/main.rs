struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop (&mut self) {
        println!("Droping {} ", ); 
    }
}