use std::collections::HashSet;
use std::fs;

type BoardValues = [[u8; 5]; 5];
type BoardHits = [[bool; 5]; 5];

#[derive(Debug)]
struct BoardState {
    values: BoardValues,
    hits: BoardHits,
}

impl BoardState {
    fn check_number(&mut self, val: u8) {
        for (i, row) in self.values.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if *num == val {
                    self.hits[i][j] = true;
                }
            }
        }
    }

    fn test_row(&self, i: usize) -> bool {
        for j in 0..5 {
            if !(self.hits[i][j]) {
                return false;
            }
        }
        //println!("hit row {}", i);
        return true;
    }

    fn test_col(&self, j: usize) -> bool {
        for i in 0..5 {
            if !(self.hits[i][j]) {
                return false;
            }
        }
        //println!("hit col {}", j);
        return true;
    }

    fn update(&mut self, val: u8) -> bool {
        self.check_number(val);
        for i in 0..5 {
            if self.test_row(i) {
                return true;
            }
            if self.test_col(i) {
                return true;
            }
        }
        return false;
    }

    fn calc_score(&mut self, num: u8) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !(self.hits[i][j]) {
                    sum += self.values[i][j] as u32;
                }
            }
        }
        return sum * (num as u32);
    }
}

fn parse_input() -> (Vec<u8>, Vec<BoardValues>) {
    let filename = "input/4.txt";
    let txt = fs::read_to_string(filename).expect("input failed!");

    let mut boards: Vec<BoardValues> = Vec::new();

    let mut chunks = txt.split("\n\n");
    let numline = chunks.next().unwrap();
    let nums = numline
        .split(",")
        .map(|str| str.parse::<u8>())
        .filter(|res| res.is_ok())
        .map(|x| x.unwrap())
        .collect();

    for chunk in chunks {
        let lines = chunk.split("\n");
        let mut board: BoardValues = [[0; 5]; 5];
        let mut row = 0;
        for line in lines {
            let mut col = 0;
            for str in line.split(" ") {
                if str == " " {
                    continue;
                }
                let res = str.parse::<u8>();
                if res.is_ok() {
                    let num = res.unwrap();
                    board[row][col] = num;
                    col += 1;
                }
            }
            row += 1;
        }
        boards.push(board);
    }
    return (nums, boards);
}

// find the first winning board and return its score
pub fn test1() -> u32 {
    let mut score: Option<u32> = None;

    let (nums, values) = parse_input();
    let mut boards: Vec<BoardState> = values
        .iter()
        .map(|vals| BoardState {
            values: *vals,
            hits: [[false; 5]; 5],
        })
        .collect();

    let nboards = boards.len();
    let mut done = false;

    for num in nums {
        if done {
            break;
        }
        for i in 0..nboards {
            if boards[i].update(num) {
                score = Some(boards[i].calc_score(num));
                done = true;
                break;
            }
        }
    }
    // if winner.is_some() {
    //     let w = winner.unwrap();
    //     println!("winning board ({}): \n", w);
    //     println!("{:?}", boards[w]);
    // }
    return score.unwrap_or(0);
}


// find the last winning board and return its index
pub fn test2() -> u32 {
    let (nums, values) = parse_input();
    let mut boards: Vec<BoardState> = values
        .iter()
        .map(|vals| BoardState {
            values: *vals,
            hits: [[false; 5]; 5],
        })
        .collect();

    let nboards = boards.len();
    let mut done = false;
    let mut won: Vec<usize> = Vec::new();
    let mut notwon = HashSet::new();
    let mut won_this_time = HashSet::new();

    for i in 0..nboards {
        notwon.insert(i);
    }

    let mut lastnum: u8 = 0;
    for num in nums {
        if done {
            break;
        }
        let _ = &won_this_time.clear();
        for i in &notwon {
            if boards[*i].update(num) {
                won_this_time.insert(*i);
                won.push(*i);
            }
        }
        for i in &won_this_time {
            notwon.remove(&i);
        }
        if notwon.len() < 1 {
            lastnum = num;
            done = true;
        }
    }
    let last = *won.last().unwrap();
    return boards[last].calc_score(lastnum);
    //return *won.last().unwrap();
}
