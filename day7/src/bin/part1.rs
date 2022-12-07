use std::env;
use std::fs;
use std::process;
use std::iter;

struct FileSystem {
    contents: Vec<File>
}

impl FileSystem {
    fn add_content(&mut self, name: &str, size: Option<usize>, parent: Option<usize>) -> usize {
        let idx = self.contents.len();
        self.contents.push(File::new(idx, name, size, parent));
        if let Some(parentidx) = parent {
            self.contents[parentidx].contents.push(idx);
        }
        idx
    }

    fn find_dir(&self, name: &str, parent: Option<usize>) -> Option<usize> {
        for c in &self.contents {
            if c.name == name && c.size.is_none() && c.parent == parent {
                return Some(c.idx);
            }
        }
        None
    }
    
    fn add_dir(&mut self, name: &str, parent: Option<usize>) -> usize {
        if let Some(idx) = self.find_dir(name, parent) {
            idx
        } else {
            self.add_content(name, None, parent)
        }
    }

    fn add_file(&mut self, name: &str, size: usize, parent: usize) {
        self.add_content(name, Some(size), Some(parent));
    }

    fn size(&self, dir: usize) -> usize {
        let mut size = 0;
        for idx in &self.contents[dir].contents {
            if self.contents[*idx].is_dir() {
                size += self.size(*idx);
            } else {
                size += self.contents[*idx].size.unwrap();
            }
        }
        size
    }

    fn print_all(&self) {
        let rootidx = self.find_dir("/", None).unwrap();
        println!("/");
        self.print(rootidx, 0);
    }

    fn print(&self, diridx: usize, num_spaces: usize) {
        for idx in &self.contents[diridx].contents {
            if self.contents[*idx].is_dir() {
                println!("{} {}", iter::repeat(' ').take(num_spaces).collect::<String>(), &self.contents[*idx].name);
                self.print(*idx, num_spaces + 2);
            } else {
                println!("{} {} ({})", iter::repeat(' ').take(num_spaces).collect::<String>(), &self.contents[*idx].name, self.contents[*idx].size.unwrap());
            }
        }
    }

    fn get_dirs_smaller_than(&self, size: usize, startidx: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();
        for idx in &self.contents[startidx].contents {
            let dirsize = self.size(*idx);
            if self.contents[*idx].is_dir() {
                ret.append(&mut self.get_dirs_smaller_than(size, *idx));
            }
            if dirsize < size {
                ret.push(dirsize);
            }
        }
        ret
    }
}

struct File {
    idx: usize,
    name: String,
    parent: Option<usize>,
    contents: Vec<usize>,
    size: Option<usize>,
}

impl File {
    fn new(idx: usize, name: &str, size: Option<usize>, parent: Option<usize>) -> Self {
        File {
            idx,
            name: name.to_string(),
            parent,
            contents: Vec::new(),
            size,
        }
    }

    fn is_dir(&self) -> bool {
        self.size.is_none()
    }
}

fn read_input(filename: &str) -> FileSystem {
    let mut filesystem = FileSystem { contents: Vec::new() };
    let mut cur_path: Vec<usize> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    cur_path.pop();
                } else {
                    let idx = if cur_path.is_empty() {
                        filesystem.add_dir(parts[2], None)
                    } else {
                        filesystem.add_dir(parts[2], Some(*cur_path.last().unwrap()))
                    };
                    cur_path.push(idx);
                }
            }
        } else {
            if parts[0] == "dir" {
                filesystem.add_dir(parts[1], Some(*cur_path.last().unwrap()));
            } else {
                filesystem.add_file(parts[1], parts[0].parse::<usize>().unwrap(), *cur_path.last().unwrap());
            }
        }
    }
    filesystem
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let fs = read_input(&args[1]);
    fs.print_all();

    let sizes = fs.get_dirs_smaller_than(100000, fs.find_dir("/", None).unwrap());
    println!("{}", sizes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    println!("Total sum {}", sizes.iter().sum::<usize>());
}