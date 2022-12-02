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
            "A" => { RPS::Rock }
            "B" => { RPS::Paper }
            "C" => { RPS::Scissors }
            _ => { panic!() }
        };
        match letters.next().unwrap() {
            "X" => {
                match opponent {
                    RPS::Scissors => { score += 6 }
                    RPS::Rock => { score += 3 }
                    _ => {}
                }
                score += 1;
            }
            "Y" => {
                match opponent {
                    RPS::Rock => { score += 6 }
                    RPS::Paper => { score += 3 }
                    _ => {}
                }
                score += 2;
            }
            "Z" => {
                match opponent {
                    RPS::Paper => { score += 6 }
                    RPS::Scissors => { score += 3 }
                    _ => {}
                }
                score += 3;
            }
            _ => {}
        };
    }
    println!("{}", score);
}
