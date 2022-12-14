use std::{collections::HashSet, fs, path::Path};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let pairs = content.split("\n").map(|x| x.split(","));

    let mut result_1 = 0;
    let mut result_2 = 0;
    pairs.for_each(|mut pair| {
        let first_section = pair.next().unwrap();
        let first_set = range_set(first_section);

        let second_section = pair.next().unwrap();
        let second_set = range_set(second_section);

        if first_set.is_subset(&second_set) || second_set.is_subset(&first_set) {
            result_1 += 1;
        }

        if first_set.intersection(&second_set).next() != None {
            result_2 += 1;
        }
    });

    (result_1, result_2)
}

fn range_set(section: &str) -> HashSet<u32> {
    let mut section_split = section.split("-");
    let start = section_split.next().unwrap().parse::<u32>().unwrap();
    let end = section_split.next().unwrap().parse::<u32>().unwrap();
    let range = HashSet::from_iter(start..end + 1);
    range
}
