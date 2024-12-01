use std::{env, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Please provide the input file");

    let mut f = File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut copies = vec![1; buf.lines().count()];

    for (i, line) in buf.lines().enumerate() {
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
                card_score += 1;
            }
        }

        if card_score > 0 {
            for j in 1..=card_score {
                copies[i + j] += copies[i];
            }
        }
    }

    let total: i32 = copies.iter().sum();

    println!("{:?}", copies);
    println!("total: {}", total);

    Ok(())
}
