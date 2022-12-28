use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");

    let mut chars = content.chars();
    let mut current_idx = 3;
    let mut current_chars = VecDeque::new();

    current_chars.push_back(chars.next().unwrap());
    current_chars.push_back(chars.next().unwrap());
    current_chars.push_back(chars.next().unwrap());
    current_chars.push_back(chars.next().unwrap());

    let result_1;
    loop {
        if !is_dup(&current_chars) {
            println!("{:#?}", current_chars);
            result_1 = current_idx + 1;
            break;
        }

        current_chars.pop_front();
        current_chars.push_back(chars.next().unwrap());
        current_idx += 1;
    }

    (result_1, 0)
}

fn is_dup(v: &VecDeque<char>) -> bool {
    let mut s = HashSet::new();
    for x in v.iter() {
        if !s.insert(x) {
            // println!("{}", x);
            return true;
        }
    }
    return false;
}
