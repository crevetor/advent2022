use std::env;
use std::fs;
use std::process;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn read_input(filename: &str) -> Vec<Instruction> {
    let mut ret: Vec<Instruction> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        let parts = line.trim().split(" ").collect::<Vec<&str>>();
        if parts[0] == "addx" {
            ret.push(Instruction::Addx(parts[1].parse::<i32>().unwrap()));
        } else if parts[0] == "noop" {
            ret.push(Instruction::Noop);
        } else {
            panic!("Found unexpected instruction {}", parts[0]);
        }
    }

    ret
}

fn num_cycles(instruction: Instruction) -> usize {
    match instruction {
        Instruction::Noop => 1,
        Instruction::Addx(_) => 2,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    println!("{:?}", contents);
    let mut x = 1;
    let mut cycle = 0;
    for inst in contents {
        for _ in 0..num_cycles(inst) {
            if cycle > 0 && cycle % 40 == 0 {
                println!("");
            }
            if cycle % 40 >= x - 1 && cycle % 40 <= x + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
        }
        if let Instruction::Addx(a) = inst {
            x += a;
        }
    }
}