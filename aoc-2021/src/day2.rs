use std::fs;

#[derive(Debug)]
enum Dir { 
    Forward, Up, Down, Invalid
}

fn parse_command_list(filename:&str) -> Vec<(Dir, i64)> { 
    let txt = fs::read_to_string(filename).expect("input failed!");
    let lines = txt.split("\n");
    let mut commands: Vec<(Dir, i64)> = Vec::new();
    for l in lines { 
        let cmd = l.split(" ").collect::<Vec<&str>>();
        let dir = match cmd[0].as_ref() {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => Dir::Invalid 
        };
        let val = cmd[1].parse::<i64>();
        if val.is_ok() {
            let com = (dir, val.unwrap());
            commands.push(com);
        } 
    }
    return commands;
}

pub fn test1() -> i64 { 
    let commands = parse_command_list("input/2.txt");
    let mut x:i64 = 0;
    let mut z:i64 = 0;
    for com in commands {
        match com.0 {
            Dir::Forward => { x = x + com.1; },
            Dir::Up      => { z = z - com.1; },
            Dir::Down    => { z = z + com.1; },
            Dir::Invalid => {}
        }
    }
    return x * z;
}

pub fn test2() -> i64 { 
    let commands = parse_command_list("input/2.txt");
    let mut x:i64 = 0;
    let mut z:i64 = 0;
    let mut a:i64 = 0;
    for com in commands {
        match com.0 {
            Dir::Forward => { 
                x = x + com.1;
                z = z + (a * com.1);
            },
            Dir::Up => { 
                a = a - com.1;
            },
            Dir::Down => {
                a = a + com.1;
            },
            Dir::Invalid => {}
        }
    }
    return x * z;
}