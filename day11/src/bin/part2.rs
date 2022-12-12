use std::env;
use std::fs;
use std::process;

use regex::Regex;

#[derive(Debug, Clone)]
enum Operand {
    Num(i32),
    Old,
}

#[derive(Debug, Clone)]
struct Monkey {
    idx: usize,
    items: Vec<i128>,
    operation: (char, Operand),
    test: (i32, usize, usize),
    inspect: usize,
}

impl Monkey {
    fn inspect(&mut self) -> Vec<(usize, i128)> {
        let mut ret: Vec<(usize, i128)> = Vec::new();
        for i in &self.items {
            self.inspect += 1;
            let mut lvl = *i;
            if let Operand::Num(operand) = self.operation.1 {
                lvl = match self.operation.0 {
                    '+' => i + i128::from(operand),
                    '-' => i - i128::from(operand),
                    '*' => i * i128::from(operand),
                    '/' => i / i128::from(operand),
                    o => panic!("Got an unexpected operation {}", o),
                };
            }
            if lvl % i128::from(self.test.0) == 0 {
                ret.push((self.test.1, i128::from(self.test.0)));
            } else {
                ret.push((self.test.2, lvl));
            }
        }

        ret
    }
}

fn read_input(filename: &str) -> Vec<Monkey> {
    let monkey_re = Regex::new(r"^Monkey (\d+):$").unwrap();
    let items_re = Regex::new(r"^\s+Starting items: (.+)$").unwrap();
    let operation_re = Regex::new(r"\s+Operation: new = old ([\*/\+\-]) (\d+|old)$").unwrap();
    let test_re = Regex::new(r"^\s+Test: divisible by (\d+)").unwrap();
    let true_re = Regex::new(r"^\s+If true: throw to monkey (\d+)$").unwrap();
    let false_re = Regex::new(r"^\s+If false: throw to monkey (\d+)$").unwrap();

    let mut ret: Vec<Monkey> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    let mut monkey = Monkey {
        idx: 0,
        items: Vec::new(),
        operation: ('/', Operand::Old),
        test: (1, 1, 1),
        inspect: 0,
    };
    for line in content.lines() {
        if monkey_re.is_match(line) {
            let idx = monkey_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            monkey.idx = idx;
        }

        if items_re.is_match(line) {
            monkey.items = items_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .split(", ")
                .map(|x| x.parse::<i128>().unwrap())
                .collect();
        }

        if operation_re.is_match(line) {
            let caps = operation_re.captures(line).unwrap();
            monkey.operation.0 = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            monkey.operation.1 = match caps.get(2).unwrap().as_str() {
                "old" => Operand::Old,
                a => Operand::Num(a.parse::<i32>().unwrap()),
            }
        }

        if test_re.is_match(line) {
            monkey.test.0 = test_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
        }

        if true_re.is_match(line) {
            monkey.test.1 = true_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        }

        if false_re.is_match(line) {
            monkey.test.2 = false_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        }

        if line.is_empty() {
            ret.push(monkey);
            monkey = Monkey {
                idx: 0,
                items: Vec::new(),
                operation: ('/', Operand::Old),
                test: (1, 1, 1),
                inspect: 0,
            };
        }
    }
    ret.push(monkey);

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let mut contents = read_input(&args[1]);
    contents.sort_by_key(|k| k.idx);
    println!("{:?}", contents);

    for _ in 0..10000 {
        for idx in 0..contents.len() {
            let items = contents[idx].inspect();
            for item in items {
                contents[item.0].items.push(item.1);
            }
            contents[idx].items.clear();
        }
        println!("{:?}", contents);
    }
    contents.sort_by_key(|k| k.inspect);
    println!("Most active {:?} {:?}", contents.iter().rev().nth(0).unwrap(), contents.iter().rev().nth(1).unwrap());
    println!("{}", contents.iter().rev().nth(0).unwrap().inspect * contents.iter().rev().nth(1).unwrap().inspect);
}
