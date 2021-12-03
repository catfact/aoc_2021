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

    let mut numpos: Vec<u32> = vec![0; bits];
    let mut res: u32 = 0;

    let mut n = 0;
    for bin in binlist {
        for (b, x) in bin.iter().enumerate() {
            if *x {
                numpos[b] = numpos[b] + 1
            }
        }
        n = n + 1;
    }

    let n_2 = n / 2;
    for b in 0..bits {
        if numpos[b] > n_2 {
            res = res | (1 << b);
        }
    }
    return res;
}
