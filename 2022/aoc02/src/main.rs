extern crate core;

use std::fs::File;
use std::io::Read;

pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut score = 0;

    for line in input.lines() {
        let mut letters = line.split_whitespace();
        let opponent = match letters.next().unwrap() {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => {
                panic!()
            }
        };
        match letters.next().unwrap() {
            // Lose
            "X" => {
                score += match opponent {
                    RPS::Rock => { 3 }
                    RPS::Paper => { 1 }
                    RPS::Scissors => { 2 }
                };
                score += 0;
            }
            // Draw
            "Y" => {
                score += match opponent {
                    RPS::Rock => { 1 }
                    RPS::Paper => { 2 }
                    RPS::Scissors => { 3 }
                };
                score += 3;
            }
            // Win
            "Z" => {
                score += match opponent {
                    RPS::Rock => { 2 }
                    RPS::Paper => { 3 }
                    RPS::Scissors => { 1 }
                };
                score += 6;
            }
            _ => {}
        };
    }
    println!("{}", score);
}
