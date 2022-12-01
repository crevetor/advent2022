use std::env;
use std::fs;
use std::process;

fn read_input(filename: &str) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        if line.is_empty() {
            ret.push(-1);
        } else {
            ret.push(line.parse().expect("Unable to parse int"));
        }
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let calories = read_input(&args[1]);
    let mut total: Vec<i32> = Vec::new();
    let mut acc: i32 = 0;
    for cal in calories {
        if cal == -1 {
            total.push(acc);
            acc = 0;
        } else {
            acc += cal;
        }
    }
    total.push(acc);
    total.sort();
    println!("Max is {}", total[total.len() - 1]);
    let mut top3 = 0;
    for i in 1..4 {
        println!("Top {} is {}", i, total[total.len() - i]);
        top3 += total[total.len() - i];
    }
    println!("Top 3 total : {}", top3);
}