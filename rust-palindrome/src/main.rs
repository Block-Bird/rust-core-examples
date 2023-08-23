use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'initialize' function below.
 *
 * The function accepts STRING s as parameter.
 */

fn initialize(s: &str) {
    // This function is called once before all queries.

}

/*
 * Complete the 'answerQuery' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER l
 *  2. INTEGER r
 */

fn answerQuery(l: i32, r: i32) -> i32 {
    // Return the answer for this query modulo 1000000007.
    l
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    initialize(&s);

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let l = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let r = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = answerQuery(l, r);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
