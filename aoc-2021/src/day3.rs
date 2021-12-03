use std::fs;

fn parse_binary_list(filename: &str) -> Vec<Vec<bool>> {
    let txt = fs::read_to_string(filename).expect("input failed!");
    let mut res: Vec<Vec<bool>> = Vec::new();
    for line in txt.split("\n") {
        res.push(line.chars().map(|ch| ch == '1').collect::<Vec<bool>>());
    }
    return res;
}

// return integer constructed by taking most common bit in each place
pub fn test1() -> u32 {
    let filename = "input/3.txt";
    let binlist = parse_binary_list(filename);
    let bits = 12;

    // count of '1's in the data for each bit position:
    let mut ones: Vec<u32> = vec![0; bits];

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let mut n = 0;
    for bin in binlist {
        for (b, x) in bin.iter().enumerate() {
            if *x {
                ones[b] = ones[b] + 1
            }
        }
        n = n + 1;
    }
    // println!("n = {}", n);

    let n_2 = n / 2;

    // reverse the count vector to get LSB first
    ones.reverse();
    for b in 0..bits {
        if ones[b] > n_2 {
            gamma = gamma | (1 << b);
        } else {
            epsilon = epsilon | (1 << b);
        }
    }
    // for b in 0..bits { 
    //     print!("{} ", ones[b]);
    // }
    // println!("");
    return gamma * epsilon;
}
