use std::{fs, path::Path};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let elves_food_str = content.split("\n\n");

    let mut elves_food_vec = Vec::new();
    for elf_food_str in elves_food_str {
        let elf_food_items_str = elf_food_str.split("\n");
        let mut elf_food_vec = Vec::new();
        for f in elf_food_items_str {
            let elf_food = str::parse::<u32>(f).expect("Cannot parse input");
            elf_food_vec.push(elf_food);
        }
        elves_food_vec.push(elf_food_vec);
    }

    let result_1 = sum_n_max_elements(&elves_food_vec, 1);
    let result_2 = sum_n_max_elements(&elves_food_vec, 3);
    (result_1, result_2)
}

fn sum_n_max_elements(vec: &Vec<Vec<u32>>, n: usize) -> u32 {
    let mut result = vec
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    result.sort();
    result.reverse();

    let result = result[..n].iter().sum::<u32>();
    result
}
