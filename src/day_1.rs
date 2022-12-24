use std::{fs, path::Path};

pub fn solve(path: &Path) -> u32 {
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

    let result = elves_food_vec
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .max()
        .expect("Cannot calculate max sum");
    result
}
