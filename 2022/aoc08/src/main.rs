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

    let mut count = 0;

    for (row_idx, row) in forest.iter().enumerate() {
        for (col_idx, &tree) in row.iter().enumerate() {
            if row_idx == 0
                || col_idx == 0
                || row_idx == forest.len() - 1
                || col_idx == row.len() - 1
            {
                count += 1;
                continue;
            }

            // up
            {
                let mut largest = 0;
                for r in 0..row_idx {
                    if forest[r][col_idx] > largest {
                        largest = forest[r][col_idx];
                    }
                }
                if largest < tree {
                    count += 1;
                    continue;
                }
            }
            // down
            {
                let mut largest = 0;
                for r in row_idx + 1..forest.len() {
                    if forest[r][col_idx] > largest {
                        largest = forest[r][col_idx];
                    }
                }
                if largest < tree {
                    count += 1;
                    continue;
                }
            }
            // down
            {
                let mut largest = 0;
                for c in 0..col_idx {
                    if forest[row_idx][c] > largest {
                        largest = forest[row_idx][c];
                    }
                }
                if largest < tree {
                    count += 1;
                    continue;
                }
            }
            // down
            {
                let mut largest = 0;
                for c in col_idx + 1..row.len() {
                    if forest[row_idx][c] > largest {
                        largest = forest[row_idx][c];
                    }
                }
                if largest < tree {
                    count += 1;
                    continue;
                }
            }
        }
    }

    println!("{count}")
}
