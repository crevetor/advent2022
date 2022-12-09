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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    println!("{:?}", contents);
    let mut hcoord: (i32, i32) = (0, 0);
    let mut tcoord: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for m in contents {
        println!("Move {} {:?}", m.1, m.0);
        for _ in 0..m.1 {
            hcoord = do_move(hcoord, m.0);
            if hcoord.0 != tcoord.0 && hcoord.1 != tcoord.1 {
                let diffx = hcoord.0 - tcoord.0;
                let diffy = hcoord.1 - tcoord.1;
                if diffx.abs() > 1 {
                    tcoord = if diffx.is_negative() {
                        (tcoord.0 + diffx + 1, tcoord.1 + diffy)
                    } else {
                        (tcoord.0 + diffx - 1, tcoord.1 + diffy)
                    };
                }
                if diffy.abs() > 1 {
                    tcoord = if diffy.is_negative() {
                        (tcoord.0 + diffx, tcoord.1 + diffy + 1)
                    } else {
                        (tcoord.0 + diffx, tcoord.1 + diffy - 1)
                    }

                }
                println!("Diagonal");
            } else if (hcoord.0 - tcoord.0).abs() == 2 || (hcoord.1 - tcoord.1).abs() == 2 {
                tcoord = do_move(tcoord, m.0);         
            }
            println!("H: {:?}, T: {:?}", hcoord, tcoord);

            visited.insert(tcoord);
        }
    }

    println!("{:?}", visited);
    println!("{}", visited.len());

}