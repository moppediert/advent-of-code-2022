use std::{collections::HashMap, fs, path::Path};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let rounds_tr = content.split("\n");
    let scores = rounds_tr
        .map(|p| get_score(p))
        .reduce(|acc, i| (acc.0 + i.0, acc.1 + i.1))
        .expect("Cannot calculate score");
    scores
}

fn get_score(play: &str) -> (u32, u32) {
    let scoreboard_1: HashMap<&str, u8> = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);
    let scoreboard_2: HashMap<&str, u8> = HashMap::from([
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ]);
    (
        scoreboard_1.get(play).expect("Invalid move").clone().into(),
        scoreboard_2.get(play).expect("Invalid move").clone().into(),
    )
}
