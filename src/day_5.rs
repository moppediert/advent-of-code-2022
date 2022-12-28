use std::{
    collections::{LinkedList, VecDeque},
    fs,
    path::Path,
    result,
};

pub fn solve(path: &Path) -> (String, String) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let mut split = content.split("\n\n");

    let stacks_str = split.next().unwrap();
    let num_stacks = stacks_str
        .split("\n")
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks_1 = vec![VecDeque::<char>::new(); num_stacks];

    let stacks_lines = stacks_str.split("\n");
    for line in stacks_lines {
        let chars_vec = line.char_indices();
        for c in chars_vec {
            if c.1.is_alphabetic() {
                stacks_1[(c.0 - 1) / 4].push_front(c.1);
            }
        }
    }

    let mut stacks_2 = stacks_1.clone();

    let moves_str = split.next().unwrap();
    for move_str in moves_str.split("\n") {
        let splits = move_str.split_whitespace().collect::<Vec<&str>>();
        let amount = splits[1].parse::<usize>().unwrap();
        let from = splits[3].parse::<usize>().unwrap() - 1;
        let to = splits[5].parse::<usize>().unwrap() - 1;

        // debug_stack(&stacks[from], &stacks[to]);
        let len_from = stacks_1[from].len();
        let mut drained = stacks_1[from]
            .drain(len_from - amount..).rev().collect();
        stacks_1[to].append(&mut drained);

        let len_from = stacks_2[from].len();
        let mut drained = stacks_2[from]
            .drain(len_from - amount..).collect();
        stacks_2[to].append(&mut drained);

        // debug_stack(&stacks[from], &stacks[to]);
    }

    let result_1 = stacks_1.iter().map(|x| x.back().unwrap()).collect::<String>();
    let result_2 = stacks_2.iter().map(|x| x.back().unwrap()).collect::<String>();
    (result_1, result_2)
}

fn debug_stack(v1: &VecDeque<char>, v2: &VecDeque<char>) {
    println!(
        "{:#?} &&& {:#?}",
        v1.iter().rev().collect::<Vec<&char>>(),
        v2.iter().rev().collect::<Vec<&char>>()
    );
}
