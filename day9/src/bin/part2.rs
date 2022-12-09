use std::env;
use std::fs;
use std::process;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn read_input(filename: &str) -> Vec<(Direction, usize)> {
    let mut ret: Vec<(Direction, usize)> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        let dir = match line.chars().nth(0).unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            d => panic!("Unexpected direction {}", d),
        };
        let num = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        ret.push((dir, num));
    }

    ret
}

fn do_move(coord: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Left => (coord.0 - 1, coord.1),
        Direction::Right => (coord.0 + 1, coord.1),
        Direction::Up => (coord.0, coord.1 + 1),
        Direction::Down => (coord.0, coord.1 - 1),
    }
}

fn dir_from_dist(from: (i32, i32), to: (i32, i32)) -> Direction {
    if from == to {
        panic!("From is same as to");
    }
    if (to.0 - from.0).is_negative() {
        return Direction::Left;
    } else if to.0 != from.0 {
        return Direction::Right;
    }
    if (to.1 - from.1).is_negative() {
        return Direction::Down;
    } else {
        return Direction::Up;
    }
}

fn new_coords(tcoord: (i32, i32), hcoord: (i32, i32)) -> (i32, i32) {
    let mut newcoord = tcoord;
    if tcoord.0 != hcoord.0 && tcoord.1 != hcoord.1 {
        if (tcoord.0 - hcoord.0).abs() > 1 || (tcoord.1 - hcoord.1).abs() > 1 {
            newcoord = do_move(newcoord, dir_from_dist((0, newcoord.1), (0, hcoord.1)));
            newcoord = do_move(newcoord, dir_from_dist((newcoord.0, 0), (hcoord.0, 0)));
        }
    } else if ((tcoord.0 - hcoord.0).abs() > 1 && tcoord.1 == hcoord.1)|| (tcoord.0 == hcoord.0 && (tcoord.1 - hcoord.1).abs() > 1) {
        newcoord = do_move(tcoord, dir_from_dist(tcoord, hcoord));
    }
    newcoord
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    println!("{:?}", contents);
    let mut coords: Vec<(i32, i32)> = [(0, 0); 10].to_vec();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for m in contents {
        println!("Move {} {:?}", m.1, m.0);
        for _ in 0..m.1 {
            coords[0] = do_move(coords[0], m.0);
            for i in 1..coords.len() {
                coords[i] = new_coords(coords[i], coords[i - 1]);
            }
            visited.insert(coords[9]);
        }
        println!("{:?}", coords);
    }

    println!("{:?}", visited);
    println!("{}", visited.len());
}
