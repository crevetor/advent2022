use std::env;
use std::fs;
use std::process;

use simple_matrix::Matrix;

fn read_input(filename: &str) -> Matrix<char> {
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    let height = content.lines().count();
    let width = content.lines().nth(0).unwrap().trim().chars().count();
    Matrix::from_iter(height, width, content.chars().filter(|x| *x != '\n' && *x != '\r'))
}

fn can_climb(from: (usize, usize), to: (usize, usize), matrix: &Matrix<char>) -> bool {
    let mut from_char = *matrix.get(from.1, from.0).unwrap();
    let mut to_char = *matrix.get(to.1, to.0).unwrap(); 

    if from_char == 'S' {
        from_char = 'a';
    }

    if to_char == 'E' {
        to_char = 'z';
    }

    //u32::from(to_char) <= u32::from(from_char) + 1
    u32::from(to_char) <= u32::from(from_char) + 1 && u32::from(to_char) >= u32::from(from_char)
}

fn gen_neighbors(from: (usize, usize), matrix: &Matrix<char>) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    if from.0 >= 1 {
        ret.push((from.0 - 1, from.1));
    }
    if from.0 + 1 < matrix.cols().try_into().unwrap() {
        ret.push((from.0 + 1, from.1));
    }
    if from.1 >= 1 {
        ret.push((from.0, from.1 - 1));
    }
    if from.1 + 1 < matrix.rows().try_into().unwrap() {
        ret.push((from.0, from.1 + 1));
    }

    ret
}

fn get_path(from: (usize, usize), prev: Vec<(usize, usize)>, matrix: &Matrix<char>) -> Vec<Vec<(usize, usize)>> {
    let mut ret: Vec<Vec<(usize, usize)>> = Vec::new();
    if matrix.get(from.1, from.0).unwrap() == &'E' {
        ret.push(vec![from]);
        println!("Found path");
        return ret;
    }
    let mut prev = prev.to_vec();
    prev.push(from);
    for neighbor in gen_neighbors(from, matrix) {
        if !prev.contains(&neighbor) && can_climb(from, neighbor, matrix) {
            let mut paths = get_path(neighbor, prev.clone(), matrix);
            for path in paths.iter_mut() {
                path.insert(0, from);
            }
            if !paths.is_empty() {
                ret.append(&mut paths);
            }
        }
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
    println!("{:?}", contents);
    let mut start = (0,0);
    for x in 0..contents.cols() {
        for y in 0..contents.rows() {
            if contents.get(y, x).unwrap() == &'S' {
                start = (x, y);
            }
        }
    }

    let mut paths = get_path(start, vec![start], &contents);
    paths.sort_by_key(|k| k.len());
    for path in paths {
        println!("{:?} {}", path, path.len());
    }
}