use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone)]
enum Val {
    Int(i32),
    List(Vec<Val>),
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

fn read_input(filename: &str) -> Vec<(Val, Val)> {
    let mut ret: Vec<(Val, Val)> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    let mut lines_iter = content.lines();
    loop {
        if let Some(line) = lines_iter.next() {
            if !line.is_empty() {
                ret.push((
                    parse(&mut line.chars()),
                    parse(&mut lines_iter.next().unwrap().chars()),
                ));
            }
        } else {
            break;
        }
    }
    ret
}

fn cmp(left: &Val, right: &Val) -> Option<bool> {
    if let Val::Int(l) = left {
        match right {
            Val::Int(r) => {
                if l < r {
                    return Some(true);
                } else if l > r {
                    return Some(false);
                }
            }
            Val::List(_) => {
                let ret = cmp(&Val::List(vec![left.clone()]), right);
                if ret.is_some() {
                    return ret;
                }
            }
        }
    }
    if let Val::List(l) = left {
        match right {
            Val::List(r) => {
                for i in 0..l.len() {
                    if i < r.len() {
                        let r = cmp(&l[i], &r[i]);
                        if r.is_some() {
                            return r;
                        }
                    } else {
                        return Some(false);
                    }
                }
                if l.len() < r.len() {
                    return Some(true);
                }
            },
            Val::Int(_) => {
                let ret = cmp(left, &Val::List(vec![right.clone()]));
                if ret.is_some() {
                    return ret;
                }
            }
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    println!("{:?}", contents);
    let mut idxsum = 0;

    for (i, (left, right)) in contents.iter().enumerate() {
        let ret = cmp(left, right);
        println!("{} : {:?}", i, ret);
        if ret == Some(true) {
            idxsum += i + 1;
        }
    }
    println!("{}", idxsum);
}
