use std::{fs, path::Path};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");

    let mut matrix = vec![];

    for line in content.split("\n") {
        let vec = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        matrix.push(vec);
    }
    let height = matrix.len();
    let width = matrix[0].len();

    let mut left = vec![0; height];
    let mut right = vec![0; height];
    let mut top = matrix[0].clone();
    let mut bottom = matrix[width - 1].clone();



    let mut visibility = vec![vec![false; width]; height];
    visibility[0] = vec![true; width];
    visibility[height - 1] = vec![true; width];
    for i in 0..height {
        left[i] = matrix[i][0];
        right[i] = matrix[i][width - 1];

        visibility[i][0] = true;
        visibility[i][height - 1] = true;
    }

    for w in 0..width {
        
    }


    (0, 0)
}
