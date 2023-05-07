pub struct Person {
    name: String,
    age: u8,
}

struct Point {
    point_x: u8,
    point_y: u8,
}

struct number;

// #[derive(Debug)]
struct Rectangle {
    lenth: Point,
    width: Point,
}
fn main() {
    let name = String::from("Asad Ali");

    // Add Data in Rectangle
    let point1: Point = Point {
        point_x: (32),
        point_y: (33),
    };

    let point2: Point = Point {
        point_x: (42),
        point_y: (43),
    };

    let rec1: Rectangle = Rectangle {
        lenth: (point1),
        width: (point2),
    };

    println!(
        "Printing  Rec1 Length {:?} , {:?}",
        rec1.lenth.point_x, rec1.lenth.point_x
    );
    println!(
        "Printing Rec 1 Width {:?}, {:?}",
        rec1.width.point_x, rec1.width.point_y
    );
    // println!("Printing  Rec2 Width {:?}", point2);
}
