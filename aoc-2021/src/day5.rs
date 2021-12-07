use std::collections::HashMap;
use std::fs;

fn parse_input() -> Vec<[[i32; 2]; 2]> {
    let filename = "input/5.txt";
    let txt = fs::read_to_string(filename).expect("input failed!");
    let mut z: Vec<[[i32; 2]; 2]> = Vec::new();

    for line in txt.split("\n") {
        let vals: Vec<Vec<i32>> = line
            .split(" -> ")
            .map(|spl| spl.split(",").map(|s| s.parse::<i32>().unwrap()).collect())
            .collect();
        z.push([[vals[0][0], vals[0][1]], [vals[1][0], vals[1][1]]]);
    }
    println!("{:?}", z);
    return z;
}

fn build_zmap(segments: &Vec<[[i32; 2]; 2]>) -> HashMap<(i32, i32), i32> {
    let mut zmap: HashMap<(i32, i32), i32> = HashMap::new();
    for seg in segments {
        if seg[0][0] == seg[1][0] {
            //println!("processing vertical segment: {:?}", seg);
            let x = seg[0][0];
            let ay = seg[0][1].min(seg[1][1]);
            let by = seg[0][1].max(seg[1][1]);
            for y in ay..(by + 1) {
                let k = (x, y);
                let z = zmap.get(&k).unwrap_or(&0) + 1;
                //println!("new z = {}", z);
                zmap.insert(k, z);
            }
        } else if seg[0][1] == seg[1][1] {
            //println!("processing horizontal segment: {:?}", seg);
            let y = seg[0][1];
            let ax = seg[0][0].min(seg[1][0]);
            let bx = seg[0][0].max(seg[1][0]);
            for x in ax..(bx + 1) {
                let k = (x, y);
                let z = zmap.get(&k).unwrap_or(&0) + 1;
                zmap.insert(k, z);
            }
        }
    }
    return zmap;
}

fn get_range(segments: &Vec<[[i32; 2]; 2]>) -> ((i32, i32), (i32, i32)) {
    let xmin = segments
        .iter()
        .map(|seg| seg[0][0].min(seg[1][0]))
        .min()
        .unwrap();
    let xmax = segments
        .iter()
        .map(|seg| seg[0][0].max(seg[1][0]))
        .max()
        .unwrap();
    let ymin = segments
        .iter()
        .map(|seg| seg[1][0].min(seg[1][1]))
        .min()
        .unwrap();
    let ymax = segments
        .iter()
        .map(|seg| seg[1][0].max(seg[1][1]))
        .max()
        .unwrap();
    return ((xmin, ymin), (xmax, ymax));
}

pub fn test1() -> usize {
    let segments = parse_input();
    // segments = segments
    //     .iter()
    //     .filter(|seg| seg[0][0] == seg[1][0] || seg[1][0] == seg[1][1])
    //     .map(|x| *x)
    //     .collect();

    let zmap = build_zmap(&segments);
    // debug display
    let ((xmin, ymin), (xmax, ymax)) = get_range(&segments);

    for j in ymin..(ymax + 1) {
        for i in xmin..(xmax + 1) {
            let z = zmap.get(&(i, j)).unwrap_or(&0);
            print!(
                "{}",
                if z > &0 {
                    z.to_string()
                } else {
                    ".".to_string()
                }
            );
        }
        println!("");
    }

    let mut count = 0;
    for (_, z) in zmap {
        if z > 1 {
            count += 1;
        }
    }

    return count;
}
