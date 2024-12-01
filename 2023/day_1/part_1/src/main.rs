use std::{env, fs::File, io::Read, iter::FromIterator};

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Please provide input file");

    let mut f = File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let result: i32 = buf
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<Vec<char>>();

            String::from_iter([digits.first().unwrap(), digits.last().unwrap()])
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    println!("{}", result);

    Ok(())
}
