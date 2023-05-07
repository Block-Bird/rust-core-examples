// Add attributes
// create struct to for Students

struct Students {
    name: String,
    age: u8,
    father_name: String,
}
fn add_data(name: String, age: u8, father_name: String) -> Students {
    Students {
        name,
        age,
        father_name,
    }
}

fn main() {
    let person1 = add_data(String::from("Asad Ali"), 23, String::from("Zafar Ali Khan")); 
    let person2 = add_data(String::from("Noman Ali"), 18, String::from("Mazhar Ali Khan")); 
    let person3 = add_data(String::from("Rehan Ali"), 23, String::from("Mazhar Ali Khan")); 

    println!("Name of P1 is {:?}", person1.name); 
    println!("Age of P2 is {:?}", person2.age); 
}
