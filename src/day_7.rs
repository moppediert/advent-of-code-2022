use std::{collections::HashMap, fmt, fs, path::Path, rc::Rc};
// use FileSystem::{Dir, File};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    // let root = Dir(
    //     vec![
    //         Box::new(Dir(vec![], "subdir".to_string())),
    //         Box::new(File(2762, "subfile".to_string())),
    //     ],
    //     "root".to_string(),
    // );
    // println!("{}", root);
    let mut file_system = HashMap::<String, File>::new();
    let mut current_dir: Rc<File>;
    let lines = content.split("\n");

    for line in lines {
        if line.starts_with("$") {
            let split = line.split_whitespace();
            // TODO
        }
    }

    (0, 0)
}

fn file(current: Rc<File>, line: &str) -> File {
    let mut split = line.split_whitespace();
    let size = split.next().unwrap().to_string().parse::<usize>().unwrap();
    let name = split.next().unwrap().to_string();
    File {
        parent: Rc::clone(&current),
        path: format!("{}/{}", current.path, name),
        is_dir: false,
        name,
        size,
    }
}

fn dir(current: Rc<File>, line: &str) -> File {
    let name = line.split_whitespace().last().unwrap().to_string();
    File {
        parent: Rc::clone(&current),
        path: format!("{}/{}", current.path, name),
        is_dir: true,
        name,
        size: 0,
    }
}

struct File {
    parent: Rc<File>,
    path: String,
    is_dir: bool,
    name: String,
    size: usize,
}
