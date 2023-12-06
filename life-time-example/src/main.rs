fn main() {
    let string1 = String::from("Here is ");

    let string2 = String::from("Ali is your name");

    let result = longest(&string1, &string2);
    println!("Here's the result {}", result);


    let novel = String::from("CAll my name Asad Ali . here isss "); 
    let first_sentense = novel.split(".").next().expect("could not find"); 
    println!("Here is = {}", first_sentense); 

    // let i = ImportantExcerpt {
    //     part: first_sentense, 
    // }; 
}

fn longest<'lifetime>(a: &'lifetime str, b: &'lifetime str) -> &'lifetime str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}


struct ImportantExcerpt <'a> {
    part: &'a str, 

}



