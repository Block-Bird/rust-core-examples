use std::fs;
use std::io::{BufReader, BufRead};


// let file_contents = fs::read_to_string("info.txt");



fn main() {
    println!("Started Reading Fille");

    let ans = read_file();

    println!("Answer is {:?} ", ans);

}




fn read_file() -> std::io::Result<()> {
    // let file = File::open("essay.txt")?; 
    // let reader = BufReader::new(file);

    let file_content = fs::read_to_string("essay.txt")?;

    let upper_case_text = file_content.to_uppercase();

    print!("{}", upper_case_text);

    Ok(())
}