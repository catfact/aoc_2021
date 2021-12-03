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

// return vector with count of set bits for each position
fn count_ones(readings: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut ones: Vec<usize> = vec![0; NBITS];
    for list in readings {
        for (b, x) in list.iter().enumerate() {
            if *x {
                ones[b] += 1;
            }
        }
    }
    return ones;
}

// count readings with given bit set
fn count_ones_pos(readings: &Vec<Vec<bool>>, pos: usize) -> usize {
    return readings.iter().filter(|list| list[pos]).count();
}

pub fn test1() -> u32 {
    let filename = "input/3.txt";
    let binlist = parse_binary_list(filename);
    let n = binlist.len();
    let mut ones = count_ones(&binlist);
    let mut zeros: Vec<usize> = ones.iter().map(|x| n - x).collect();
    // integer constructed from most common bits:
    let mut gamma: u32 = 0;
    // and from least common bits:
    let mut epsilon: u32 = 0;

    // put LSB first:
    ones.reverse();
    zeros.reverse();
    for b in 0..NBITS {
        if ones[b] > zeros[b] {
            gamma |= 1 << b;
        } else {
            epsilon |= 1 << b;
        }
    }
    return gamma * epsilon;
}

// return a new vector of bitvectors,
// containing only those bitvectors matching given pos/val
fn apply_bit_criteria(candidates: &Vec<Vec<bool>>, invert: bool) -> u32 {
    let mut remaining = candidates.clone();
    let mut pos: usize = 0;
    let mut target;
    while remaining.len() > 1 {
        let ones = count_ones_pos(&remaining, pos);
        let zeros = remaining.len() - ones;
        if ones == zeros {
            target = true;
        } else {
            target = ones > zeros;
        }
        target ^= invert;
        remaining = remaining
            .iter()
            .filter(|bin| bin[pos] == target)
            .cloned()
            .collect();
        println!("checked bit pos {}; remaining = {}", pos, remaining.len());
        pos += 1;
    }

    println!(
        "final:  {:?}",
        remaining[0]
            .iter()
            .map(|v| if *v { 1 } else { 0 })
            .collect::<Vec<i32>>()
    );
    let mut finalbits = remaining[0].clone();
    finalbits.reverse();
    return (0..NBITS).fold(0, |acc, b| acc + if finalbits[b] { 1 << b } else { 0 });
}

pub fn test2() -> u32 {
    let filename = "input/3.txt";
    let readings = parse_binary_list(filename);

    let oxy = apply_bit_criteria(&readings, true);
    let co2 = apply_bit_criteria(&readings, false);

    return oxy * co2;
}

// wrong: 2503197
