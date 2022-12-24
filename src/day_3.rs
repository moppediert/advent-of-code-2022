use std::{collections::HashMap, fs, path::Path};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    (solve_1(&content), solve_2(&content))
}

fn solve_2(content: &str) -> u32 {
    let mut items = content.split("\n");
    let mut priority_sum = 0;
    loop {
        let mut item_map_1 = HashMap::new();
        let first_item = items.next();
        if first_item.is_none() {
            break;
        }
        first_item.unwrap().chars().for_each(|i| {
            item_map_1.entry(i).or_insert(1);
        });

        let mut item_map_2 = HashMap::new();
        items.next().unwrap().chars().for_each(|i| {
            item_map_2.entry(i).or_insert(1);
        });

        let mut item_map_3 = HashMap::new();
        items.next().unwrap().chars().for_each(|i| {
            item_map_3.entry(i).or_insert(1);
        });

        for (&k, _) in item_map_2.iter() {
            item_map_1.entry(k).and_modify(|e| *e += 1).or_insert(1);
        }

        for (&k, _) in item_map_3.iter() {
            item_map_1.entry(k).and_modify(|e| *e += 1).or_insert(1);
        }

        for (&k, &v) in item_map_1.iter() {
            if v > 2 {
                priority_sum += get_priority(k);
            }
        }
    }
    priority_sum
}

fn solve_1(content: &str) -> u32 {
    let item_pairs = content.split("\n").map(|x| x.split_at(x.len() / 2));

    let mut priority_sum = 0;
    for pair in item_pairs {
        let mut item_map_1 = HashMap::new();
        pair.0.chars().for_each(|i| {
            item_map_1.entry(i).or_insert(1);
        });

        let mut item_map_2 = HashMap::new();
        pair.1.chars().for_each(|i| {
            item_map_2.entry(i).or_insert(1);
        });

        for (&k, _) in item_map_2.iter() {
            item_map_1.entry(k).and_modify(|e| *e += 1).or_insert(1);
        }

        for (&k, &v) in item_map_1.iter() {
            if v > 1 {
                priority_sum += get_priority(k);
            }
        }
    }
    priority_sum
}

// Convert from utf8 encoding https://www.utf8-chartable.de/unicode-utf8-table.pl?utf8=dec
// Assuming input is [a-zA-Z]
fn get_priority(item: char) -> u32 {
    let mut buffer = [0; 1];
    item.encode_utf8(&mut buffer);
    if buffer[0] <= 90 {
        return (buffer[0] - 65 + 27).into();
    }
    (buffer[0] - 97 + 1).into()
}
