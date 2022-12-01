use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut elves = Vec::new();
    let mut new_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves.push(new_elf);
            new_elf = 0;
        } else {
            new_elf += line.parse::<u32>().unwrap();
        }
    }

    elves.sort();

    let mut top_three = 0;

    let index = elves.len() - 3;
    for i in 0..3 {
        top_three += elves[index + i];
    }

    println!("{}", top_three);
}
