use std::fmt::Write;

macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_sum = 0;
            $(
                temp_sum += $x;
            )*
            temp_sum
        }
    };
}


macro_rules! find_min {
    ($x: expr) => ($x); 
    ($x: expr, $($y: expr), +) => {
        std::cmp::min($x, find_min!($($y), + ))
    }
}

fn calculate_hash(block: &Block) -> String {
    let mut s = String::new();
    write!(&mut s, "{}{}{}{}", block.index, block.timestamp, block.data, block.previous_hash).unwrap();
    s
}

fn generate_block(previous_block: &Block, data: String) -> Block {
    let index = previous_block.index + 1;
    let timestamp = 0; // TODO: implement timestamp
    let previous_hash = previous_block.hash.clone();
    let hash = calculate_hash(&Block { index, timestamp, data: data.clone(), previous_hash: previous_hash.clone(), hash: String::new() });
    Block { index, timestamp, data, previous_hash, hash }
}

fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
    if previous_block.index + 1 != new_block.index {
        return false;
    }
    if calculate_hash(previous_block) != new_block.previous_hash {
        return false;   
    }
    if calculate_hash(new_block) != new_block.hash {
        return false;
    }
    true
}

fn is_valid_chain(chain: &[Block]) -> bool {
    for i in 1..chain.len() {
        if !is_valid_new_block(&chain[i], &chain[i - 1]) {
            return false;
        }
    }
    true
}


macro_rules! print_binary {
    ($val:expr) => {
        println!("{:#b}", $val);
    };
}




macro_rules! add_func {
    () => (
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    );
}

add_func!();

macro_rules! sub_func {
    () => (
        fn sub(a: i32, b: i32) -> i32 {
            a - b
        }
    );
}



sub_func!();


fn main() {
    // println!("{}", find_min!(156487, 9, 7968,6578,7986,567));
    // let result = add(2, 3);
    // println!("Result: {}", result);

    // let result = sub(5, 2);
    // println!("Result: {}", result);

    let my_vector = my_vec![1, 2, 3];
    println!("{:?}", my_vector); // prints "[1, 2, 3]"


    let num: u8 = 42;
    print_binary!(num);

    let number: u8 = 77; 

    println!("Binary of 77 is {:#b} ", number);
    let mut blockchain = vec![Block {
        index: 0,
        timestamp: 0,
        data: String::from("Genesis block"),
        previous_hash: String::new(),
        hash: String::new(),
    }];

    blockchain.push(generate_block(&blockchain[0], String::from("First block")));
    blockchain.push(generate_block(&blockchain[1], String::from("Second block")));

    println!("Blockchain: {:?}", blockchain);

    if is_valid_chain(&blockchain) {
        println!("Blockchain is valid");
    } else {
        println!("Blockchain is not valid");
    }


    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let total = sum!(a, b, c, d);
    println!("The sum of {} + {} + {} + {} is {}", a, b, c, d, total);
    

}

