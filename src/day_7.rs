use std::{cell::RefCell, collections::HashMap, fs, path::Path, rc::Rc};

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
    let mut file_system = HashMap::<String, Rc<RefCell<File>>>::new(); // <path, file>
    let root_dir = Rc::new(RefCell::new(dir(None, "/")));
    let mut current_dir = Rc::clone(&root_dir);
    file_system.insert("/".to_string(), root_dir);
    let mut lines = content.split("\n");
    lines.next(); // Ignore the first command, since it's (supposedly) always cd /
    for line in lines {
        let mut split = line.split_whitespace();

        let first = split.next().unwrap();
        if first.starts_with("$") {
            if split.next().unwrap() == "ls" {
                // read files
            } else {
                // cd
                let to_dir = split.next().unwrap();
                match to_dir {
                    "/" => {
                        let root_dir = Rc::new(RefCell::new(dir(None, "/")));
                        current_dir = Rc::clone(&root_dir);
                        file_system.insert("/".to_string(), root_dir);
                    }
                    ".." => {
                        let current_dir_path = Rc::as_ref(&current_dir).borrow().path.clone();
                        let split_path = current_dir_path.split("/");
                        let new_path = if let Some(last) = split_path.last() {
                            current_dir_path.strip_suffix(last).unwrap()
                        } else {
                            "/"
                        };
                        current_dir = file_system.get(new_path).unwrap().clone();
                    }
                    some_dir => {
                        // Assume that the dir following the `cd` command already appeared in the result of one of the `ls` commands before
                        current_dir = file_system.get(some_dir).unwrap().clone();
                    }
                }
            }
        } else if first.starts_with("dir") {
            let dir_name = split.next().unwrap();
            let new_dir = Rc::new(RefCell::new(dir(Some(current_dir), dir_name)));
            let new_dir_path = Rc::as_ref(&new_dir).borrow().path.clone();
            file_system.insert(new_dir_path, new_dir);
        } else {
            let file_size = first.parse::<usize>().unwrap();
            let file_name = split.next().unwrap();
            let file = Rc::new(RefCell::new(file(Some(current_dir), file_name, file_size)));
            let file_path = Rc::as_ref(&file).borrow().path.clone();
            file_system.insert(file_path, file);
        }
    }

    (0, 0)
}

fn file(parent_dir: Option<Rc<RefCell<File>>>, name: &str, size: usize) -> File {
    let parent = parent_dir.unwrap(); // A file always has a parent, which is its directory
    Rc::as_ref(&parent).borrow_mut().update_size(size);

    File {
        parent: Some(Rc::clone(&parent)),
        path: format!("{}/{}", Rc::as_ref(&parent).borrow().path, name),
        is_dir: false,
        name: name.to_string(),
        size,
    }
}

fn dir(parent_dir: Option<Rc<RefCell<File>>>, name: &str) -> File {
    if let Some(parent) = parent_dir {
        File {
            parent: Some(Rc::clone(&parent)),
            path: format!("{}/{}", Rc::as_ref(&parent).borrow().path, name),
            is_dir: true,
            name: name.to_string(),
            size: 0,
        }
    } else {
        File {
            parent: None,
            path: "/".to_string(),
            is_dir: true,
            name: "/".to_string(),
            size: 0,
        }
    }
}

struct File {
    parent: Option<Rc<RefCell<File>>>,
    path: String,
    is_dir: bool,
    name: String,
    size: usize,
}

impl File {
    fn new() -> File {
        File {
            parent: None,
            path: "".to_string(),
            is_dir: false,
            name: "".to_string(),
            size: 0,
        }
    }
    fn update_size(&mut self, child_size: usize) {
        self.size += child_size;
        if let Some(parent) = &self.parent {
            Rc::as_ref(&parent).borrow_mut().update_size(child_size);
        }
    }
}
