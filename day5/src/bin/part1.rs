use std::env;
use std::fs;
use std::process;

use regex::Regex;

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<(usize, usize, usize)> = Vec::new();
    let crate_re = Regex::new(r"( {3} ?)|(\[[a-zA-Z]\] ?)").unwrap();
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    let mut proc_moves = false;
    for line in content.lines() {
        //println!("Processing {}", line.trim());
        if line.is_empty() {
            println!("Line is empty, changing to moves");
            proc_moves = true;
            continue;
        }
        if !proc_moves {
            println!("Crates");
            for (i, caps) in crate_re.find_iter(line).enumerate() {
                let crate_name = caps.as_str().chars().nth(1).unwrap();
                println!("Crate '{}'", crate_name);
                if let Some(stack) = stacks.get_mut(i) {
                    stack.insert(0, crate_name);
                } else {
                    stacks.push(Vec::from([crate_name]))
                }
                println!("Added crate {} in stack {}", crate_name, i+1);
            }
        } else {
            let caps: Vec<usize> = move_re.captures(line.trim()).unwrap().iter().skip(1).map(|x| x.unwrap().as_str().parse::<usize>().unwrap()).collect();
            moves.push((caps[0], caps[1], caps[2]));
            //println!("Adding move {} {} {}", caps[0], caps[1], caps[2]);
        }
    }
    for i in 0..stacks.len() {
        stacks[i] = stacks.get(i).unwrap().iter().filter(|x| **x != ' ').cloned().collect();
    }

    (stacks, moves)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let (mut stacks, moves) = read_input(&args[1]);

    for (num, from, to) in moves {
        for (i, stack) in stacks.iter().enumerate() {
            println!("{} : {}", i+1, String::from_iter(stack));
        }
        println!("Moving {} from {} to {}", num, from, to);
        for i in 0..num {
            let poped_crate = stacks[from-1].pop().unwrap();
            stacks[to-1].push(poped_crate);
        }
    }

    println!("{}", stacks.iter().map(|x| x.iter().last().unwrap()).collect::<String>());

}