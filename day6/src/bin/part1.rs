use std::env;
use std::fs;
use std::process;

use std::collections::HashSet;

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    for line in contents {
        let line: Vec<char> = line.trim().chars().collect();
        for i in 0..line.len() {
            let marker: HashSet<char> = line.iter().skip(i).take(4).cloned().collect();
            if marker.len() == 4 {
                println!("Found marker at {}", i + 4);
                break;
            }
        }
    }
}