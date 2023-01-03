use std::{fs, path::Path};

pub fn solve(path: &Path) -> (u32, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");

    let mut matrix = vec![];

    for (j, line) in content.split("\n").enumerate() {
        let vec = line
            .chars()
            .enumerate()
            .map(|(i, c)| (j, i, c.to_digit(10).unwrap()))
            .collect::<Vec<(usize, usize, u32)>>();
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
        // left[i] = matrix[i][0];
        // right[i] = matrix[i][width - 1];

        visibility[i][0] = true;
        visibility[i][height - 1] = true;
    }

    // Rough idea
    // If a tree is horizontally visible, it must be either taller than every tree on the left, or taller than every tree on the right
    // Let's call the tree at position i in the horizontal array x_i. That means at least of of these two conditions must satisfy:
    // x_i > x_j for all j < i or x_i > x_j for all j > i
    // Let's sort the array from large to small values in a stable way, i.e. the indices of same values don't change their order
    // If x_i > x_j for all j < i, after the sort, all x_j will appear on the right side of x_i, which means on the left side of x_i,
    // there are only x_k with k > i. And due to the sort, x_k >= x_i for every x_k on the left side of x_i.
    // Similarly, if x_i > x_j for all j > i, there will be only x_k on the left side with k < i.
    // So what we have to do is to sort the array descendingly and check if every values on the left of x_i has ax exclusively
    // smaller or larger INDEX than x_i, i.e. k < i or k > i.

    let matrix = matrix.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, val)| (j, i, val));
    });

    (0, 0)
}
