use std::fs;

fn parse_integer_list(filename:&str) -> Vec<i32> { 
    let txt = fs::read_to_string(filename).expect("input failed!");
    let z = txt.split("\n").map(|str| str.parse::<i32>()).filter(|r| r.is_ok());
    return z.map(|r| r.unwrap()).collect();
}

fn test_1() -> i32 { 
    let filename = "input/1.txt";
    let z = parse_integer_list(filename);
    let mut n:i32 = 0;
    for pair in z.windows(2) { 
        if pair[1] > pair[0] { 
            n += 1;
        }
    }    
    return n;
}

fn main() {
    println!("test 1 result: {}", test_1());
}
