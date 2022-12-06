use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let input: VecDeque<_> = input.chars().collect();
    let mut first_four = VecDeque::new();
    'outer: for (i, char) in input.iter().enumerate() {
        first_four.push_back(char);
        if first_four.len() > 4 {
            first_four.pop_front();
        } else {
            continue 'outer;
        }

        let mut four = Vec::new();
        for &x in &first_four {
            if !four.contains(&x) {
                four.push(x);
            } else {
                continue 'outer;
            }
        }
        println!("{}", i + 1);
        break;
    }
}
