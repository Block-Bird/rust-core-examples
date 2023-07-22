// Create a macro named check_eq that takes two expressions and checks if they are equal. If they are equal, 
// the macro should print "Equal!" otherwise "Not Equal!". Use the macro to compare some variables.


macro_rules! conditional_macros {
    ($value1:expr, $value2:expr) => {
        if ($value1 == $value2 ) {
            println!("Equal");
        }
        else {
            println!("Not Equal");
        }
    };
}

fn main() {
    conditional_macros!(12,32);
    conditional_macros!(32,32);
}
