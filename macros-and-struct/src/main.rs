struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rectangle = Rectangle::new(10, 20);
    println!("Rectangle Area: {}", rectangle.area());
    println!("Is Square: {}", rectangle.is_square());

    let square = Rectangle::new(15, 15);
    println!("Square Area: {}", square.area());
    println!("Is Square: {}", square.is_square());
}
