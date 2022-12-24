use std::{fs, path::Path, collections::HashMap};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");
    let rounds_tr = content.split("\n");
    let score = rounds_tr.map(|p| get_score(p)).sum::<u32>();
    

    (score, 0)
}

fn get_score(play: &str) -> u32 {
    let scoreboard: HashMap<&str, u8> = HashMap::from(
        [
            ("A X", 1 + 3),
            ("A Y", 2 + 6),
            ("A Z", 3 + 0),
            ("B X", 1 + 0),
            ("B Y", 2 + 3),
            ("B Z", 3 + 6),
            ("C X", 1 + 6),
            ("C Y", 2 + 0),
            ("C Z", 3 + 3)
        ]
    );
    scoreboard.get(play).expect("Invalid move").clone().into()
}