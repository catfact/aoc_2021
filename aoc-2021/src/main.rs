use std::fs;

fn parse_integer_list(filename:&str) -> Vec<i32> { 
    let txt = fs::read_to_string(filename).expect("input failed!");
    let z = txt.split("\n").map(|str| str.parse::<i32>()).filter(|r| r.is_ok());
    return z.map(|r| r.unwrap()).collect();
}

// count of increasing values
fn test_1() -> u32 { 
    let filename = "input/1.txt";
    let z = parse_integer_list(filename);
    let n:u32 = z.windows(2).map(|w| if w[1]>w[0] {1} else {0}).sum();
    return n;
}

// count of increasing sums over sliding window
fn test_2() -> u32 { 
    let filename = "input/1.txt";
    let z = parse_integer_list(filename);
    let sums:Vec<i32> = z.windows(3).map(|w| w.iter().sum()).collect();
    let n:u32 = sums.windows(2).map(|w| if w[1]>w[0] {1} else {0}).sum();
    return n;
}

fn main() {
    println!("test 1 result: {}", test_1());
    println!("test 2 result: {}", test_2());
}