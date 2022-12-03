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
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);

    let mut sum = 0;
    for line in contents {
        if line.len() % 2 != 0 {
            panic!("Found odd number of letters in {}", line);
        }
        let left: HashSet<char> = line.chars().take(line.len()/2).collect();
        let right: HashSet<char> = line.chars().skip(line.len()/2).collect();
        let common = left.intersection(&right).nth(0).unwrap();
        println!("Common letter for is {}", common);
        let idx = alphabet.iter().position(|&x| x == *common).unwrap();
        println!("Idx of {} is {}", common, idx);
        sum += idx + 1;
    } 
    println!("Total sum : {}", sum);
}