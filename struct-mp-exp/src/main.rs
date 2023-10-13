macro_rules! create_generic_struct {
    ($struct_name:ident, $($field_name:ident : $field_type:ty),*) => {
        struct $struct_name<$($field_name),*> {
            $($field_name: $field_name),*
        }
        
        impl<$($field_name),*> $struct_name<$($field_name),*> {
            fn new($($field_name: $field_type),*) -> Self {
                $struct_name {
                    $($field_name),*
                }
            }
        }
    };
}

create_generic_struct!(MyStruct, field1: i32, field2: f64);

fn main() {
    let my_struct: MyStruct<field1::i32, field2::f64> = MyStruct::new(42, 3.14);
    println!("field1: {}, field2: {}", my_struct.field1, my_struct.field2);
}
