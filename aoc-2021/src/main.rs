use std::fs;

fn parse_integer_list(filename:&str) -> Vec<i32> { 
    let txt = fs::read_to_string(filename).expect("input failed!");
    let z = txt.split("\n").map(|str| str.parse::<i32>()).filter(|r| r.is_ok());
    return z.map(|r| r.unwrap()).collect();
}

// find count of values that are greater than previous
fn test_1() -> u32 { 
    let filename = "input/1.txt";
    let z = parse_integer_list(filename);

    let mut n:u32 = 0;
    for pair in z.windows(2) { 
        if pair[1] > pair[0] { 
            n += 1;
        }
    }    
    return n;
}

// count number of moving sums which are increasing with windowsize=3
fn test_2() -> u32 { 
    let filename = "input/1.txt";
    let z = parse_integer_list(filename);
    let sums:Vec<i32> = z.windows(3).map(|w| w.iter().sum()).collect();
    
    let mut n:u32 = 0;
    for pair in sums.windows(2) { 
        if pair[1] > pair[0] { 
            n += 1;
        }
    }    
    return n;
}

fn main() {
    println!("test 1 result: {}", test_1());
    println!("test 2 result: {}", test_2());
}
