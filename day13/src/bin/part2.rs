use std::env;
use std::fs;
use std::process;

use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
enum Val {
    Int(i32),
    List(Vec<Val>),
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Val::Int(l) = self {
            match other {
                Val::Int(r) => {
                    if l < r {
                        return Ordering::Less;
                    } else if l > r {
                        return Ordering::Greater;
                    }
                }
                Val::List(_) => {
                    let ret = Val::List(vec![self.clone()]).cmp(other);
                    if ret != Ordering::Equal {
                        return ret;
                    }
                }
            }
        }
        if let Val::List(l) = self {
            match other {
                Val::List(r) => {
                    for i in 0..l.len() {
                        if i < r.len() {
                            let r = l[i].cmp(&r[i]);
                            if r != Ordering::Equal {
                                return r;
                            }
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    if l.len() < r.len() {
                        return Ordering::Less;
                    }
                }
                Val::Int(_) => {
                    let ret = self.cmp(&Val::List(vec![other.clone()]));
                    if ret != Ordering::Equal {
                        return ret;
                    }
                }
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn parse(list: &mut std::str::Chars) -> Val {
    let mut acc = String::new();
    let mut ret: Vec<Val> = Vec::new();
    while let Some(c) = list.next() {
        match c {
            '[' => ret.push(parse(list)),
            ',' => {
                if !acc.is_empty() {
                    ret.push(Val::Int(acc.parse::<i32>().unwrap()));
                }
                acc = String::new();
            }
            ']' => {
                if !acc.is_empty() {
                    ret.push(Val::Int(acc.parse::<i32>().unwrap()));
                }
                return Val::List(ret);
            }
            other => acc.push(other),
        }
    }
    Val::List(ret)
}

fn read_input(filename: &str) -> Vec<Val> {
    let mut ret: Vec<Val> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        if !line.is_empty() {
            ret.push(parse(&mut line.chars()));
        }
    }
    ret.push(parse(&mut "[[2]]".chars()));
    ret.push(parse(&mut "[[6]]".chars()));
    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let mut contents = read_input(&args[1]);
    contents.sort();

    let mut dividers: Vec<Val> = Vec::new();
    dividers.push(parse(&mut "[[2]]".chars()));
    dividers.push(parse(&mut "[[6]]".chars()));

    let mut solution = 1;
    for d in dividers {
        let pos = contents.iter().position(|x| x == &d).unwrap();
        solution *= pos + 1;
    }
    println!("Solution {}", solution);

}
