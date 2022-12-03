use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut total = 0;

    let mut i = 0;

    let len = input.lines().count();

    const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut lines = input.lines();

    loop {
        if len <= i {
            break;
        }

        let mut group = Vec::new();
        for _ in 0..3 {
            group.push(lines.next().unwrap());
            i += 1;
        }

        let mut items = ITEMS.to_owned();
        for elf in group {
            items.retain(|c| elf.contains(c));
        }

        let item = items.chars().next().unwrap();

        let value;
        if item.is_uppercase() {
            value = item as u8 - b'A' + 27;
        } else {
            value = item as u8 - b'a' + 1;
        }
        total += value as u64;
    }
    println!("{}", total);
}
