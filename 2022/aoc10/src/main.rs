#![feature(int_roundings)]
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

struct Communicator {
    register: i32,
    cycle: usize,
    screen: [[bool; 40]; 6],
}

impl Communicator {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 0,
            screen: [[false; 40]; 6],
        }
    }

    fn next_cycle(&mut self) {
        let current_pixel = (self.cycle % 40) as i32;
        self.screen[self.cycle.div_floor(40) % 6][self.cycle % 40] = current_pixel
            == self.register - 1
            || current_pixel == self.register
            || current_pixel == self.register + 1;
        self.cycle += 1;
    }

    fn noop(&mut self) {
        self.next_cycle();
    }

    fn addx(&mut self, value: i32) {
        self.next_cycle();
        self.next_cycle();
        self.register += value;
    }
}

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut communicator = Communicator::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();

        match command {
            "noop" => communicator.noop(),
            "addx" => {
                communicator.addx(split.next().unwrap().parse().unwrap());
            }
            _ => panic!(),
        }
    }

    for row in communicator.screen {
        for pixel in row {
            match pixel {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!()
    }
}
