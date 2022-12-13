use std::env;
use std::fs;
use std::process;

use simple_matrix::Matrix;

fn read_input(filename: &str) -> Matrix<(char, usize)> {
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    let height = content.lines().count();
    let width = content.lines().nth(0).unwrap().trim().chars().count();
    Matrix::from_iter(height, width, content.chars().filter(|x| *x != '\n' && *x != '\r').map(|x| (x, std::usize::MAX)))
}

fn can_climb(from: (usize, usize), to: (usize, usize), matrix: &Matrix<(char, usize)>) -> bool {
    let mut from_char = matrix.get(from.1, from.0).unwrap().0;
    let mut to_char = matrix.get(to.1, to.0).unwrap().0; 

    if from_char == 'S' {
        from_char = 'a';
    }

    if to_char == 'E' {
        to_char = 'z';
    }

    u32::from(to_char) <= u32::from(from_char) + 1
}

fn gen_neighbors(from: (usize, usize), matrix: &Matrix<(char, usize)>) -> Vec<(usize, usize)> {
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

fn get_path(from: (usize, usize), matrix: &mut Matrix<(char, usize)>) {
    for neighbor in gen_neighbors(from, matrix) {
        let n_val = *matrix.get(neighbor.1, neighbor.0).unwrap();
        let our_val = *matrix.get(from.1, from.0).unwrap();
        if can_climb(from, neighbor, matrix) && n_val.1 > our_val.1 + 1 {
            matrix.set(neighbor.1, neighbor.0, (n_val.0, our_val.1 + 1));
            get_path(neighbor, matrix);
        }
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
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut end = (0,0);
    for x in 0..contents.cols() {
        for y in 0..contents.rows() {
            if contents.get(y, x).unwrap().0 == 'S' || contents.get(y, x).unwrap().0 == 'a' {
                starts.push((x, y));
            }
            if contents.get(y, x).unwrap().0 == 'E' {
                end = (x, y);
            }
        }
    }

    let mut shortest: Vec<(usize, (usize, usize))> = Vec::new();
    for start in starts {
        let mut matrix = contents.clone();
        matrix.set(start.1, start.0, ('S', 0));
        get_path(start, &mut matrix);
        shortest.push((matrix.get(end.1, end.0).unwrap().1, start));
    }

    shortest.sort_by_key(|x| x.0);
    println!("{:?}", shortest);
    println!("Shortest : {} at {:?}", shortest[0].0, shortest[0].1);
}