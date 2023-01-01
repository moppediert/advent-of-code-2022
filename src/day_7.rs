use std::{cell::RefCell, collections::HashMap, fmt, fs, path::Path, rc::Rc};

pub fn solve(path: &Path) -> (usize, usize) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let mut file_system = HashMap::<String, Rc<RefCell<File>>>::new(); // <path, file>

    // Assume the first command is always `cd /`
    let mut current_dir = Rc::new(RefCell::new(File::root()));
    file_system.insert("/".to_string(), Rc::clone(&current_dir));

    let lines = content.split("\n").skip(1);
    for line in lines {
        let mut split = line.split_whitespace();

        let first = split.next().unwrap();
        if first.starts_with("$") {
            if split.next().unwrap() == "ls" {
                // ignore since current dir is already set by `cd`
                continue;
            } else {
                // cd
                let to_dir = split.next().unwrap();
                match to_dir {
                    "/" => {
                        let root_dir = Rc::new(RefCell::new(File::root()));
                        current_dir = Rc::clone(&root_dir);
                    }
                    ".." => {
                        let current_dir_path = current_dir.borrow().path.clone();
                        let split_path = current_dir_path.split_inclusive("/"); // To avoid empty last element with a dir path
                        let new_path = if let Some(last) = split_path.last() {
                            current_dir_path.strip_suffix(last).unwrap()
                        } else {
                            "/"
                        };
                        current_dir = file_system.get(new_path).unwrap().clone();
                    }
                    some_dir => {
                        let full_path = current_dir.borrow().path.clone() + some_dir + "/";

                        // Assume that the dir following the `cd` command already appeared in the result of one of the `ls` commands before
                        current_dir = file_system.get(full_path.as_str()).unwrap().clone();
                    }
                }
            }
        } else if first.starts_with("dir") {
            let dir_name = split.next().unwrap();
            let new_dir = Rc::new(RefCell::new(File::dir(
                Some(Rc::clone(&current_dir)),
                dir_name,
            )));
            let new_dir_path = new_dir.borrow().path.clone();
            file_system.insert(new_dir_path, new_dir);
        } else {
            let file_size = first.parse::<usize>().unwrap();
            let file_name = split.next().unwrap();
            let file = Rc::new(RefCell::new(File::file(
                Some(Rc::clone(&current_dir)),
                file_name,
                file_size,
            )));
            let file_path = file.borrow().path.clone();
            file_system.insert(file_path, file);
        }
    }
    let root = file_system.get("/").unwrap();
    let usage = root.borrow().size;
    let need_to_delete = 30000000 - (70000000 - usage);

    let mut space_to_delete = usize::MAX;

    let mut sum = 0;
    for v in file_system.values() {
        let borrow = v.borrow();

        if borrow.size <= 100000 && borrow.is_dir {
            sum += borrow.size;
        }

        if borrow.size >= need_to_delete && borrow.size < space_to_delete {
            space_to_delete = borrow.size;
        }
    }

    (sum, space_to_delete)
}

struct File {
    parent: Option<Rc<RefCell<File>>>,
    path: String,
    is_dir: bool,
    name: String,
    size: usize,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.size)
    }
}

impl File {
    fn root() -> File {
        File {
            parent: None,
            path: "/".to_string(),
            is_dir: true,
            name: "/".to_string(),
            size: 0,
        }
    }

    fn dir(parent_dir: Option<Rc<RefCell<File>>>, name: &str) -> File {
        let parent = parent_dir.unwrap(); // A dir that is not root always has a parent dir
        File {
            parent: Some(Rc::clone(&parent)),
            path: format!("{}{}/", parent.borrow().path, name),
            is_dir: true,
            name: name.to_string(),
            size: 0,
        }
    }

    fn file(parent_dir: Option<Rc<RefCell<File>>>, name: &str, size: usize) -> File {
        let parent = parent_dir.unwrap(); // A file always has a parent, which is its directory
        parent.borrow_mut().update_size(size);

        File {
            parent: Some(Rc::clone(&parent)),
            path: format!("{}{}", parent.borrow().path, name),
            is_dir: false,
            name: name.to_string(),
            size,
        }
    }

    fn update_size(&mut self, child_size: usize) {
        self.size += child_size;
        if let Some(parent) = &self.parent {
            Rc::as_ref(&parent).borrow_mut().update_size(child_size);
        }
    }
}
