use color_print::cformat;
use std::{collections::HashMap, fs, path::Path};

pub fn solve(path: &Path) -> (usize, u32) {
    let content = fs::read_to_string(path).expect("Cannot read input file");

    // The map containing position and visibility of each tree
    let mut index_map = HashMap::new();

    // The matrix containing characters as they appear in the string
    let mut matrix = vec![];

    for (i, line) in content.lines().enumerate() {
        index_map.extend(
            line.char_indices()
                .map(|(j, _)| ((i, j), false))
                .into_iter(),
        );

        matrix.push(
            line.chars()
                .enumerate()
                .map(|(j, c)| (j, c.to_digit(10).unwrap()))
                .collect::<Vec<(usize, u32)>>(),
        );
    }

    // println!("{:?}", index_map.keys());

    let height = matrix.len();
    let width = matrix[0].len();

    // The matrix containing characters in transposed order
    let mut transpose = vec![vec![(0, 0); height]; width];
    for i in 0..height {
        for j in 0..width {
            transpose[j][i] = matrix[i][j];
            transpose[j][i].0 = i;
        }
    }

    let backup_matrix = matrix.clone();

    // print_matrix(&matrix);
    // print_matrix(&transpose);

    // Rough idea
    // If a tree is horizontally visible, it must be either taller than every tree on the left, or taller than every tree on the right
    // Let's call the tree at position i in the horizontal array x_i. That means at least of of these two conditions must satisfy:
    // x_i > x_j for all j < i or x_i > x_j for all j > i
    // Let's sort the array from large to small values in a stable way, i.e. the indices of same values don't change their order
    // If x_j < x_i for all j < i, after the sort, all x_j will appear on the right side of x_i, which means on the left side of x_i,
    // there are only x_k with k > i. And due to the sort, x_k >= x_i for every x_k on the left side of x_i.
    // Similarly, if x_i > x_j for all j > i, there will be only x_k on the left side with k < i.
    // So what we have to do is to sort the array descendingly and check if every values on the left of x_i has ax exclusively
    // smaller or larger INDEX than x_i, i.e. k < i or k > i.

    // For equal values:
    // If the indices are going up and the value is the first of its kind, then it's visible
    // If the indices are going down
    // print_matrix(&matrix, false, &index_map);
    solve_visibility(&mut matrix, &mut index_map, false);

    // let result_1 = index_map.values().filter(|v| **v).count();
    // print_matrix(&matrix, true, &index_map);

    // println!("temp: {}", result_1);

    solve_visibility(&mut transpose, &mut index_map, true);
    // print_color(&matrix, &index_map);
    print_color(&backup_matrix, &index_map);
    let result_1 = index_map.values().filter(|v| **v).count();

    (result_1, 0)
}

struct Tree {
    row: usize,
    col: usize,
    height: u32,
    visible: bool,
}

impl Tree {
    fn new(row: usize, col: usize, height: u32, visible: bool) -> Tree {
        Tree {
            row,
            col,
            height,
            visible,
        }
    }
}

fn solve_visibility(
    matrix: &mut Vec<Vec<(usize, u32)>>,
    index_map: &mut HashMap<(usize, usize), bool>,
    transpose: bool,
) {
    for (r, row) in matrix.iter_mut().enumerate() {
        let mut min_idx = usize::MAX;
        let mut max_idx = 0usize;
        // Sort row descendingly
        row.sort_by(|a, b| b.1.cmp(&a.1));
        // println!("sorted: {:?}", row);
        for i in 0..row.len() {
            let c = row[i].0;
            let ascending = c > max_idx;
            let descending = c <= min_idx;
            // println!(
            //     "min: {}, max: {}, idx: {}, val: {}, up: {}, down: {}",
            //     min_idx, max_idx, row[i].0, row[i].1, ascending, descending
            // );

            if c < min_idx {
                min_idx = c;
            }

            if c > max_idx {
                max_idx = c;
            }

            if !ascending && !descending {
                continue;
            }

            if ascending && (i == (row.len() - 1) || row[i].1 != row[i + 1].1)
                || descending && (i == 0 || row[i].1 != row[i - 1].1)
            {
                // if ascending {
                //     println!("up and last");
                // }
                // if descending {
                //     println!("down and first");
                // }

                let visible = if !transpose {
                    index_map.get_mut(&(r, c)).unwrap()
                } else {
                    index_map.get_mut(&(c, r)).unwrap()
                };
                *visible = true;
            }
        }
    }
}

fn print_color(matrix: &Vec<Vec<(usize, u32)>>, visibility_map: &HashMap<(usize, usize), bool>) {
    let mut s = "".to_string();
    for (r, v) in matrix.iter().enumerate() {
        for (c, b) in v {
            if *visibility_map.get(&(r, *c)).unwrap() {
                s = cformat!("{}<bg:green><black>{}</black></bg:green> ", s, b);
            } else {
                s = cformat!("{}{} ", s, b);
            }
        }
        s = format!("{}{}", s, "\n");

    }
    println!("{}", s);

}

fn print_matrix(
    matrix: &Vec<Vec<(usize, u32)>>,
    visibility: bool,
    visibility_map: &HashMap<(usize, usize), bool>,
) {
    let mut s = "".to_string();
    for (r, v) in matrix.iter().enumerate() {
        for (c, b) in v {
            if !visibility {
                s = format!("{}{} ", s, b);
            } else {
                s = format!("{}{} ", s, visibility_map.get(&(r, *c)).unwrap());
            }
        }
        s = format!("{}{}", s, "\n");
    }
    println!("{}", s);
}
