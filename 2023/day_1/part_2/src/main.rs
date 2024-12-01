use std::{env, fs::File, io::Read, iter::FromIterator};

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Please provide input file");

    let mut f = File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let result: i32 = buf
        .lines()
        .map(|line| {
            let mut digits: Vec<char> = Vec::new();

            for i in 0..line.len() {
                let (_, rest) = line.split_at(i);
                if rest.starts_with("one") || rest.starts_with("1") {
                    digits.push('1');
                }
                if rest.starts_with("two") || rest.starts_with("2") {
                    digits.push('2');
                }
                if rest.starts_with("three") || rest.starts_with("3") {
                    digits.push('3');
                }
                if rest.starts_with("four") || rest.starts_with("4") {
                    digits.push('4');
                }
                if rest.starts_with("five") || rest.starts_with("5") {
                    digits.push('5');
                }
                if rest.starts_with("six") || rest.starts_with("6") {
                    digits.push('6');
                }
                if rest.starts_with("seven") || rest.starts_with("7") {
                    digits.push('7');
                }
                if rest.starts_with("eight") || rest.starts_with("8") {
                    digits.push('8');
                }
                if rest.starts_with("nine") || rest.starts_with("9") {
                    digits.push('9');
                }
            }

            String::from_iter([digits.first().unwrap(), digits.last().unwrap()])
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    println!("{}", result);

    Ok(())
}
