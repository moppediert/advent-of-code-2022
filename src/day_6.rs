use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::Path,
    str::Chars,
};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let result_1 = solve_part(&mut content.chars(), 4);
    let result_2 = solve_part(&mut content.chars(), 14);
    (result_1, result_2)
}

fn solve_part(chars: &mut Chars, count: usize) -> u32 {
    let mut active_chars = VecDeque::new();
    let mut active_chars_count = HashMap::<char, u32>::new();

    let mut result = 0;
    for c in chars {
        if active_chars.len() >= count {
            remove_and_decrement(&mut active_chars, &mut active_chars_count)
        }
        insert_and_increment(&mut active_chars, &mut active_chars_count, c);
        result += 1;
        if !is_dup(&active_chars_count) && active_chars.len() >= count {
            return result;
        }
    }
    0
}

fn insert_and_increment(vec: &mut VecDeque<char>, map: &mut HashMap<char, u32>, c: char) {
    vec.push_back(c);
    map.entry(c).and_modify(|e| *e += 1).or_insert(1);
}

fn remove_and_decrement(vec: &mut VecDeque<char>, map: &mut HashMap<char, u32>) {
    if let Some(count) = map.get_mut(&vec.pop_front().unwrap()) {
        *count -= 1;
    }
}

fn is_dup(map: &HashMap<char, u32>) -> bool {
    map.values().any(|&x| x > 1)
}
