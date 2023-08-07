macro_rules! define_point {
    ($point_name:ident, $($field:ident : $field_type:ty),*) => {
        struct $point_name {
            $($field: $field_type),*
        }

        impl $point_name {
            fn new($($field: $field_type),*) -> Self {
                $point_name { $($field),* }
            }

            fn print_info(&self) {
                println!("{} info:", stringify!($point_name));
                $(
                    println!("{}: {:?}", stringify!($field), self.$field);
                )*
            }
        }
    };
}

define_point!(Point2D, x: f64, y: f64);
define_point!(Point3D, x: f64, y: f64, z: f64);

fn main() {
    let point2d = Point2D::new(3.0, 4.0);
    let point3d = Point3D::new(1.0, 2.0, 3.0);

    point2d.print_info();
    point3d.print_info();
    
}
