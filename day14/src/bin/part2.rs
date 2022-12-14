use std::env;
use std::fs;
use std::process;

use std::cmp::{min, max};

struct Rock {
    content: Vec<(i32, i32)>,
    minpoint: (i32, i32),
    maxpoint: (i32, i32),
}

impl Rock {
    fn new(line: &str) -> Self {
        let mut content = Vec::new();
        for pair in line.trim().split(" -> ") {
            let coords = pair.split(',').collect::<Vec<&str>>();
            content.push((coords[0].parse::<i32>().unwrap(), coords[1].parse::<i32>().unwrap()));
        }
        let minx = content.iter().map(|k| k.0).min().unwrap();
        let maxx = content.iter().map(|k| k.0).max().unwrap();
        let miny = content.iter().map(|k| k.1).min().unwrap();
        let maxy = content.iter().map(|k| k.1).max().unwrap();
        Rock { content, minpoint: (minx, miny), maxpoint: (maxx, maxy) }
    }

    fn hit(&self, coords: (i32, i32)) -> bool {
        if coords.0 < self.minpoint.0 || coords.0 > self.maxpoint.0 || coords.1 < self.minpoint.1 || coords.1 > self.maxpoint.1 {
            return false;
        }
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

fn hits(rocks: &Vec<Rock>, coord: (i32, i32), maxy: i32, sand: &Vec<(i32, i32)>) -> bool {
    coord.1 == maxy || sand.contains(&coord) || rocks.iter().any(|rock| rock.hit(coord))

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
        let mut newcoord = (sandcoord.0, sandcoord.1+ 1);
        if hits(&contents, newcoord, maxy + 2, &sand) {
            newcoord = (sandcoord.0 - 1, sandcoord.1 + 1);
            if hits(&contents, newcoord, maxy + 2, &sand) {
                newcoord = (sandcoord.0 + 1, sandcoord.1 + 1);
                if hits(&contents, newcoord, maxy + 2, &sand) {
                    sand.push(sandcoord);
                    if sandcoord == sandinit {
                        break
                    }
                    sandcoord = sandinit;
                    continue;
                }
            }
        }
        sandcoord = newcoord;
    }

    println!("{}", sand.len());
}
