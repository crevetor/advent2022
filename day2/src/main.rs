use std::env;
use std::fs;
use std::process;

fn read_input(filename: &str) -> Vec<(i32, i32)> {
    let mut ret: Vec<(i32, i32)> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        let a = line.trim().chars().nth(0).unwrap();
        let b = line.trim().chars().nth(2).unwrap();
        ret.push((match a {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!("Unexpected value {} at line {}", a, line),
        },
        match b {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Unexpected value {} at line {}", b, line),
        }));
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let strat = read_input(&args[1]);
    let mut total_score = 0;
    for play in strat {
        let round_score = match play.1 {
            1 => if play.0 == 1 { 3 } else { play.0 - 1 },
            2 => 3 + play.0,
            3 => 6 + if play.0 == 3 { 1 } else { play.0 + 1},
            _ => 0,
        };
        println!("Round score {}", round_score);
        total_score += round_score;
    }

    println!("Total score {}", total_score);
}