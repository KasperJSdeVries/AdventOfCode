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

            if score > 268464 / 2 {
                print!("\x1b[38;5;1m{score:>8}\x1b[0m");
            } else if score > 268464 / 4 {
                print!("\x1b[38;5;2m{score:>8}\x1b[0m");
            } else if score > 268464 / 8 {
                print!("\x1b[38;5;3m{score:>8}\x1b[0m");
            } else if score > 268464 / 16 {
                print!("\x1b[38;5;4m{score:>8}\x1b[0m");
            } else if score > 268464 / 32 {
                print!("\x1b[38;5;5m{score:>8}\x1b[0m");
            } else if score > 268464 / 64 {
                print!("\x1b[38;5;6m{score:>8}\x1b[0m");
            } else if score > 268464 / 128 {
                print!("\x1b[38;5;7m{score:>8}\x1b[0m");
            } else if score > 268464 / 256 {
                print!("\x1b[38;5;8m{score:>8}\x1b[0m");
            } else if score > 268464 / 512 {
                print!("\x1b[38;5;9m{score:>8}\x1b[0m");
            } else if score > 268464 / 1024 {
                print!("\x1b[38;5;10m{score:>8}\x1b[0m");
            } else if score > 268464 / 2048 {
                print!("\x1b[38;5;11m{score:>8}\x1b[0m");
            } else if score > 268464 / 4096 {
                print!("\x1b[38;5;12m{score:>8}\x1b[0m");
            } else if score > 268464 / 8192 {
                print!("\x1b[38;5;13m{score:>8}\x1b[0m");
            } else if score > 268464 / 16384 {
                print!("\x1b[38;5;14m{score:>8}\x1b[0m");
            } else if score > 268464 / 32768 {
                print!("\x1b[38;5;16m{score:>8}\x1b[0m");
            } else if score > 268464 / 65536 {
                print!("\x1b[38;5;17m{score:>8}\x1b[0m");
            } else if score > 268464 / 131072 {
                print!("\x1b[38;5;18m{score:>8}\x1b[0m");
            } else if score > 268464 / 262144 {
                print!("\x1b[38;5;19m{score:>8}\x1b[0m");
            } else {
                print!("{score:>8}");
            }

            if score > highest_score {
                highest_score = score;
            }
        }
        println!();
    }

    println!("{highest_score}")
}
