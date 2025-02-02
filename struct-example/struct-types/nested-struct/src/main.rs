pub struct  Person {
    name: String, 
    age: u8, 
}

struct  Point{ 
    point_x : u8 , 
    point_y: u8, 
}

struct number; 

// #[derive(Debug)]
struct Rectangle {
    lenth: Point, 
    width: Point, 
}
fn main () {
    let name = String::from("Asad Ali"); 
    
    // Add Data in Rectangle
    let point1 : Point  = Point { point_x: (32), point_y: (33) };

    let  point2: Point = Point { point_x: (42), point_y: (43) }; 


    let rec1 : Rectangle = Rectangle { lenth: (point1), width: (point2) };

    println!("Printing  Rec1 Length {:?} , {:?}", rec1.lenth.point_x, rec1.lenth.point_x); 
    println!("Printing Rec 1 Width {:?}, {:?}", rec1.width.point_x, rec1.width.point_y);
    // println!("Printing  Rec2 Width {:?}", point2); 
    
}


// // An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// // A unit struct
// struct Unit;

// // A tuple struct
// struct Pair(i32, f32);

// // A struct with two fields
// struct Point {
//     x: f32,
//     y: f32,
// }

// // Structs can be reused as fields of another struct
// struct Rectangle {
//     // A rectangle can be specified by where the top left and bottom right
//     // corners are in space.
//     top_left: Point,
//     bottom_right: Point,
// }

// fn main() {
//     // Create struct with field init shorthand
//     let name = String::from("Peter");
//     let age = 27;
//     let peter = Person { name, age };

//     // Print debug struct
//     println!("{:?}", peter);

//     // Instantiate a `Point`
//     let point: Point = Point { x: 10.3, y: 0.4 };

//     // Access the fields of the point
//     println!("point coordinates: ({}, {})", point.x, point.y);

//     // Make a new point by using struct update syntax to use the fields of our
//     // other one
//     let bottom_right = Point { x: 5.2, ..point };

//     // `bottom_right.y` will be the same as `point.y` because we used that field
//     // from `point`
//     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

//     // Destructure the point using a `let` binding
//     let Point { x: left_edge, y: top_edge } = point;

//     let _rectangle = Rectangle {
//         // struct instantiation is an expression too
//         top_left: Point { x: left_edge, y: top_edge },
//         bottom_right: bottom_right,
//     };

//     // Instantiate a unit struct
//     let _unit = Unit;

//     // Instantiate a tuple struct
//     let pair = Pair(1, 0.1);

//     // Access the fields of a tuple struct
//     println!("pair contains {:?} and {:?}", pair.0, pair.1);

//     // Destructure a tuple struct
//     let Pair(integer, decimal) = pair;

//     println!("pair contains {:?} and {:?}", integer, decimal);
// }