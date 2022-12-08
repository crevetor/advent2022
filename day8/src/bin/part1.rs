use std::env;
use std::fs;
use std::process;

use simple_matrix::Matrix;
use std::collections::HashSet;

fn read_input(filename: &str) -> Matrix<u32> {
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    let mut trees: Vec<u32> = Vec::new();
    let cols = content.lines().nth(0).unwrap().trim().chars().count();
    let rows = content.lines().count();
    for line in content.lines() {
        trees.append(&mut line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }
    Matrix::from_iter(rows, cols, trees)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    println!("{:?}", contents);

    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    // Add the border trees
    for i in 0..contents.cols() {
        visible.insert((i, 0));
        visible.insert((i, contents.rows()-1));
    }
    for i in 0..contents.rows() {
        visible.insert((0, i));
        visible.insert((contents.cols()-1, i));
    }
    println!("Edges {}", visible.len());

    for x in 1..contents.cols()-1 {
        for y in 1..contents.rows()-1 {
            let cell = contents.get(y, x).unwrap();
            let row: Vec<u32> = contents.get_row(y).unwrap().map(|x| *x).collect();
            let col: Vec<u32> = contents.get_col(x).unwrap().map(|x| *x).collect();

            let mut maxes: Vec<u32> = Vec::new();
            maxes.push(*row.iter().take(x).max().unwrap());
            maxes.push(*row.iter().skip(x+1).max().unwrap());
            maxes.push(*col.iter().take(y).max().unwrap());
            maxes.push(*col.iter().skip(y+1).max().unwrap());
            println!("Max for ({},{}) ({}), {:?}", x, y, cell, maxes);
            if cell > maxes.iter().min().unwrap() {
                println!("{} bigger than {}", cell, maxes.iter().min().unwrap());
                visible.insert((x, y));
            }
        }
    }
    println!("{:?}", visible);
    println!("{}", visible.len());
}