use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut forest: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        forest.push(Vec::new());

        for char in line.chars() {
            let height = char.to_digit(10).unwrap();
            forest.last_mut().unwrap().push(height as u8);
        }
    }

    let mut highest_score = 0;

    for (row_idx, row) in forest.iter().enumerate() {
        for (col_idx, &tree) in row.iter().enumerate() {
            let mut score = 1;

            // up
            if row_idx == 0 {
                score *= 0;
            } else {
                let mut trees = 1;
                let mut r = row_idx - 1;
                loop {
                    if r == 0 || forest[r][col_idx] >= tree {
                        score *= trees;
                        break;
                    }
                    trees += 1;
                    r -= 1;
                }
            }
            // down
            if row_idx == forest.len() - 1 {
                score *= 0;
            } else {
                let mut trees = 1;
                let mut r = row_idx + 1;
                loop {
                    if r == forest.len() - 1 || forest[r][col_idx] >= tree {
                        score *= trees;
                        break;
                    }
                    trees += 1;
                    r += 1;
                }
            }
            // left
            if col_idx == 0 {
                score *= 0;
            } else {
                let mut trees = 1;
                let mut c = col_idx - 1;
                loop {
                    if c == 0 || forest[row_idx][c] >= tree {
                        score *= trees;
                        break;
                    }
                    trees += 1;
                    c -= 1;
                }
            }
            // right
            if col_idx == row.len() - 1 {
                score *= 0;
            } else {
                let mut trees = 1;
                let mut c = col_idx + 1;
                loop {
                    if c == row.len() - 1 || forest[row_idx][c] >= tree {
                        score *= trees;
                        break;
                    }
                    trees += 1;
                    c += 1;
                }
            }
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("{highest_score}")
}
