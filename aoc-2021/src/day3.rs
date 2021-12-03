use std::fs;

const NBITS: usize = 12;

fn parse_binary_list(filename: &str) -> Vec<Vec<bool>> {
    let txt = fs::read_to_string(filename).expect("input failed!");
    let mut res: Vec<Vec<bool>> = Vec::new();
    for line in txt.split("\n") {
        res.push(line.chars().map(|ch| ch == '1').collect::<Vec<bool>>());
    }
    return res;
}

fn count_ones(binlist: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut ones: Vec<usize> = vec![0; NBITS];
    for bin in binlist {
        for (b, x) in bin.iter().enumerate() {
            if *x {
                ones[b] = ones[b] + 1
            }
        }
    }
    return ones;
}

pub fn test1() -> u32 {
    let filename = "input/3.txt";
    let binlist = parse_binary_list(filename);
    let n = binlist.len();
    let n_2 = n / 2;
    let mut ones = count_ones(&binlist);

    // integer constructed from most common bits:
    let mut gamma: u32 = 0;
    // and from least common bits:
    let mut epsilon: u32 = 0;

    // put LSB first:
    ones.reverse();
    for b in 0..NBITS {
        if ones[b] > n_2 {
            gamma = gamma | (1 << b);
        } else {
            epsilon = epsilon | (1 << b);
        }
    }

    return gamma * epsilon;
}
// return a new vector of bitvectors,
// containing only those bitvectors matching given pos/val
fn filter_bit(binlist: &Vec<Vec<bool>>, pos: usize, val: bool) -> Vec<Vec<bool>> {
    return binlist
        .iter()
        .filter(|bin| bin[pos] == val)
        .cloned()
        .collect();
}

fn apply_bit_criteria(binlist: &Vec<Vec<bool>>, target: &Vec<bool>) -> u32 {
    let mut remain = binlist.clone();
    let mut numremain = binlist.len();
    let mut pos: usize = 0;

    while numremain > 1 {
        remain = filter_bit(&remain, pos, target[pos]);
        // println!("remaining bit list: {:?}", remain.iter().map(|xx|
        //     xx.iter().map(|x| if *x {1} else {0}))).collect();
        numremain = remain.len();
        pos = pos + 1;
    }

    println!("final candidate: {:?}", remain[0]);
    let mut y = remain[0].clone();
    y.reverse();
    let mut z = 0;
    for (bit, val) in y.iter().enumerate() {
        if *val {
            z = z | (1 << bit);
        }
    }
    return z;
}

fn compute_oxy(binlist: &Vec<Vec<bool>>, ones: &Vec<usize>) -> u32 {
    let n = binlist.len();
    let n_2 = n / 2;
    let target: Vec<bool> = ones
        .iter()
        .map(|count| if count == &n_2 { true } else { count > &n_2 })
        .collect();
    println!("target: {:?}", target);
    return apply_bit_criteria(binlist, &target);
}

fn compute_co2(binlist: &Vec<Vec<bool>>, ones: &Vec<usize>) -> u32 {
    let n = binlist.len();
    let n_2 = n / 2;
    let target: Vec<bool> = ones
        .iter()
        .map(|count| if count == &n_2 { false } else { count < &n_2 })
        .collect();
    println!("target: {:?}", target);
    return apply_bit_criteria(binlist, &target);
}

pub fn test2() -> u32 {
    let filename = "input/3.txt";
    let binlist = parse_binary_list(filename);
    let ones = count_ones(&binlist);

    let oxy = compute_oxy(&binlist, &ones);
    let co2 = compute_co2(&binlist, &ones);

    return oxy * co2;
}
