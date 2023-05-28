// Define an enum to represent different types of shapes
enum Shape {
    Circle(f64),    // Circle with a radius
    Rectangle(f64, f64),    // Rectangle with width and height
    Triangle(f64, f64, f64),    // Triangle with side lengths
}

// Define a struct to hold information about a specific shape
struct ShapeInfo {
    name: String,
    shape: Shape,
}

// Implement a method for the ShapeInfo struct
impl ShapeInfo {
    // Calculate the area of the shape
    fn calculate_area(&self) -> f64 {
        match self.shape {
            Shape::Circle(radius) => 3.14 * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(side1, side2, side3) => {
                let semiperimeter = (side1 + side2 + side3) / 2.0;
                (semiperimeter * (semiperimeter - side1) * (semiperimeter - side2)
                    * (semiperimeter - side3))
                    .sqrt()
            }
        }
    }
}

fn main() {
    // Create instances of different shapes
    let circle = ShapeInfo {
        name: String::from("Circle"),
        shape: Shape::Circle(5.0),
    };

    let rectangle = ShapeInfo {
        name: String::from("Rectangle"),
        shape: Shape::Rectangle(4.0, 6.0),
    };

    let triangle = ShapeInfo {
        name: String::from("Triangle"),
        shape: Shape::Triangle(3.0, 4.0, 5.0),
    };

    // Calculate and print the area of each shape
    println!("Area of {} is {:.2}", circle.name, circle.calculate_area());
    println!(
        "Area of {} is {:.2}",
        rectangle.name,
        rectangle.calculate_area()
    );
    println!("Area of {} is {:.2}", triangle.name, triangle.calculate_area());
}