use std::cmp::max;
use std::fs::File;
use std::io::Read;

type Stacks = Vec<Vec<char>>;

trait Stack {
    fn from_config(input: &str) -> Self;
    fn move_items(&mut self, amount: usize, from: usize, to: usize);
    fn print_tops(&self);
}

impl Stack for Stacks {
    fn from_config(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();
        lines.reverse();
        let column_numbers = lines.remove(0);
        let column_count = column_numbers.split_whitespace().count();

        let mut result = Vec::new();
        for _ in 0..column_count {
            result.push(Vec::new());
        }

        for &line in lines.iter() {
            let mut remaining = line;
            let mut row = Vec::new();
            for i in 0..column_count - 1 {
                let mut element: &str = "";
                (element, remaining) = remaining.split_at(3);
                let mut space = "";
                (space, remaining) = remaining.split_at(1);
                row.insert(i, element);
            }
            row.push(remaining);

            let row: Vec<char> = row.iter().map(|x| x.chars().nth(1).unwrap()).collect();
            for (i, &char) in row.iter().enumerate() {
                match char {
                    ' ' => {}
                    _ => {
                        result[i].push(char);
                    }
                }
            }
        }

        result
    }

    fn move_items(&mut self, amount: usize, from: usize, to: usize) {
        let from_col = &mut self[from - 1];
        let at = max(0, from_col.len() - amount);
        let mut temp = from_col.split_off(at);
        let to_col = &mut self[to - 1];
        to_col.append(&mut temp);
    }

    fn print_tops(&self) {
        for stack in self {
            print!("{}", stack.last().unwrap());
        }
    }
}

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut split = input.split("\r\n\r\n");
    let config = split.next().unwrap();
    let moves = split.next().unwrap();

    let mut stacks = Stacks::from_config(config);

    for line in moves.lines() {
        let mut split = line.split_whitespace();
        split.next();
        let amount: usize = split.next().unwrap().parse().unwrap();
        split.next();
        let from: usize = split.next().unwrap().parse().unwrap();
        split.next();
        let to: usize = split.next().unwrap().parse().unwrap();
        stacks.move_items(amount, from, to);
    }

    stacks.print_tops();
}
