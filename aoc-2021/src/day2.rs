use std::fs;

#[derive(Debug)]
enum Dir { 
    Forward, Up, Down, Invalid
}

fn parse_command_list(filename:&str) -> Vec<(Dir, i32)> { 
    let txt = fs::read_to_string(filename).expect("input failed!");
    let lines = txt.split("\n");
    let mut commands: Vec<(Dir, i32)> = Vec::new();
    for l in lines { 
        let cmd = l.split(" ").collect::<Vec<&str>>();
        let dir = match cmd[0].as_ref() {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => Dir::Invalid 
        };
        let val = cmd[1].parse::<i32>();
        if val.is_ok() {
            let com =  (dir, val.unwrap());
            commands.push(com);
        } 
    }
    return commands;
}

pub fn test1() -> i32 { 
    let commands = parse_command_list("input/2.txt");
    let mut x:i32 = 0;
    let mut z:i32 = 0;
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

pub fn test2() -> u32 { 
    return 0;
}