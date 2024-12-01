use std::{env, fs::File, io::Read};

#[derive(Debug)]
struct Range {
    source: u64,
    destination: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new() -> Map {
        Map { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }
}

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Please provide input file");

    let mut f = File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let seeds_line = buf.lines().next().unwrap();
    let (_, seeds) = seeds_line.split_once(": ").unwrap();
    let mut seeds: Vec<u64> = seeds
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut maps = Vec::new();
    for line in buf.lines().filter(|x| !x.is_empty()).skip(1) {
        if line.chars().any(char::is_alphabetic) {
            maps.push(Map::new());
        } else {
            let mut values = line.split_ascii_whitespace();
            maps.last_mut().unwrap().add_range(Range {
                destination: values.next().unwrap().parse().unwrap(),
                source: values.next().unwrap().parse().unwrap(),
                length: values.next().unwrap().parse().unwrap(),
            });
        }
    }

    for seed in &mut seeds {
        for map in &maps {
            for range in &map.ranges {
                if *seed >= range.source && *seed < (range.source + range.length) {
                    *seed = range.destination + (*seed - range.source);
                    break;
                }
            }
        }
    }

    println!("{}", seeds.iter().min().unwrap());

    Ok(())
}
