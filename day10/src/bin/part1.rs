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

fn report_execution(cycle: i32, x: i32) -> i32 {
    if cycle == 20 || (cycle - 20)  % 40 == 0 {
        println!("Cycle {}, x: {} -> {}", cycle, x, cycle*x);
        return cycle*x;
    }
    0
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
    let mut sum  = 0;
    for inst in contents {
        for _ in 0..num_cycles(inst) {
            cycle += 1;
            sum += report_execution(cycle, x);
        }
        if let Instruction::Addx(a) = inst {
            x += a;
        }
    }
    println!("Sum {}", sum);
}