use std::env;
use std::fs;
use std::process;

use std::collections::HashSet;

fn read_input(filename: &str) -> Vec<Vec<HashSet<i32>>> {
    let mut ret: Vec<Vec<HashSet<i32>>> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let mut assignment: Vec<HashSet<i32>> = Vec::new();
        for part in parts {
            let bounds: Vec<i32> = part.split('-').map(|x| x.parse::<i32>().unwrap()).collect();
            assignment.push(HashSet::from_iter(bounds[0]..=bounds[1]));
        }
        ret.push(assignment);
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
    let mut count = 0;
    for sets in contents {
        if sets.len() != 2 {
            panic!("Found more or less than 2 sets");
        }
        let overlap: HashSet<i32> = sets[0].intersection(&sets[1]).cloned().collect();
        if sets[0] == overlap || sets[1] == overlap {
            count += 1;
        }
    }
    println!("{} overlap", count);
}