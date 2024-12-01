use std::{env, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Please provide the input file");

    let mut f = File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut total = 0;

    for line in buf.lines() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, numbers) = numbers.split_once(" | ").unwrap();
        let winning: Vec<i32> = winning
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        let numbers: Vec<i32> = numbers
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();

        let mut card_score = 0;
        for n in &winning {
            if numbers.contains(n) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        println!("card_score: {}", card_score);
        total += card_score;
    }

    println!("total: {}", total);

    Ok(())
}
