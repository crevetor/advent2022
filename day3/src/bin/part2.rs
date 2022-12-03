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
    let mut i = 0;
    while i <= contents.len()-3 {
        let sets: Vec<HashSet<char>> = contents.iter().skip(i).take(3).map(|x| x.chars().collect::<HashSet<char>>()).collect();
        let common = &(&sets[0] & &sets[1]) & &sets[2];
        let common = common.iter().nth(0).unwrap();
        println!("Common letter is {}", common);
        let idx = alphabet.iter().position(|&x| x == *common).unwrap();
        println!("Idx of {} is {}", common, idx);
        sum += idx + 1;
        i += 3;
    } 
    println!("Total sum : {}", sum);
}