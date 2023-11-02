use std::fs::File;
use std::io::{BufReader, BufRead};


// let file_contents = fs::read_to_string("info.txt");



fn main() {
    println!("Started Reading Fille");

    let ans = read_file();

    println!("Answer is {:?} ", ans);

}




fn read_file() -> std::io::Result<()> {
    let file = File::open("essay.txt")?; 
    let reader = BufReader::new(file);

    for line in reader.lines() {
        
        let line = line ?;
        print!("{:?}", line);
    }

    Ok(())
}