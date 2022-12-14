use std::env;
use std::fs;
use std::process;

use std::cmp::{min, max};

struct Rock {
    content: Vec<(i32, i32)>
}

impl Rock {
    fn new(line: &str) -> Self {
        let mut content = Vec::new();
        for pair in line.trim().split(" -> ") {
            let coords = pair.split(',').collect::<Vec<&str>>();
            content.push((coords[0].parse::<i32>().unwrap(), coords[1].parse::<i32>().unwrap()));
        }
        Rock { content }
    }

    fn hit(&self, coords: (i32, i32)) -> bool {
        for i in 0..self.content.len() - 1 {
            if self.content[i].0 == self.content[i+1].0 {
                if coords.0 == self.content[i].0 && coords.1 >= min(self.content[i].1, self.content[i+1].1) && coords.1 <= max(self.content[i].1, self.content[i+1].1) {
                    return true
                }
            }
            if self.content[i].1 == self.content[i+1].1 {
                if coords.1 == self.content[i].1 && coords.0 >= min(self.content[i].0, self.content[i+1].0) && coords.0 <= max(self.content[i].0, self.content[i+1].0) {
                    return true
                }
            }
        }
        false
    }

    fn maxy(&self) -> i32 {
        self.content.iter().max_by_key(|k| k.1).unwrap().1
    }
}

fn read_input(filename: &str) -> Vec<Rock> {
    let mut ret: Vec<Rock> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(Rock::new(line));
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
    let maxy = contents.iter().map(|rock| rock.maxy()).max().unwrap();
    let sandinit = (500, 0);
    let mut sandcoord = sandinit;
    let mut sand: Vec<(i32, i32)> = Vec::new();
    loop {
        if sandcoord.1 > maxy {
            break;
        }
        let mut newcoord = (sandcoord.0, sandcoord.1+ 1);
        if contents.iter().any(|rock| rock.hit(newcoord)) || sand.contains(&newcoord) {
            newcoord = (sandcoord.0 - 1, sandcoord.1 + 1);
            if contents.iter().any(|rock| rock.hit(newcoord)) || sand.contains(&newcoord) {
                newcoord = (sandcoord.0 + 1, sandcoord.1 + 1);
                if contents.iter().any(|rock| rock.hit(newcoord)) || sand.contains(&newcoord) {
                    sand.push(sandcoord);
                    sandcoord = sandinit;
                    continue;
                }
            }
        }
        sandcoord = newcoord;
    }

    println!("{}", sand.len());
}
