extern crate core;

use std::fs::File;
use std::io::Read;

struct Cycle(i32);

impl Cycle {
    fn new() -> Self {
        Self(0)
    }

    fn next(&mut self, register: i32, total: &mut i32) {
        self.0 += 1;
        match self.0 {
            20 | 60 | 100 | 140 | 180 | 220 => *total += register * self.0,
            _ => {}
        }
    }
}

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut cycle = Cycle::new();
    let mut x = 1;
    let mut total = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();

        match command {
            "noop" => cycle.next(x, &mut total),
            "addx" => {
                cycle.next(x, &mut total);
                cycle.next(x, &mut total);
                x += split.next().unwrap().parse::<i32>().unwrap();
            }
            _ => panic!(),
        }
    }

    println!("{}", total)
}
