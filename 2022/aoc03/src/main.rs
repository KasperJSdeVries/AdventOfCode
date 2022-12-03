use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut total = 0;

    for line in input.lines() {
        let len = line.len();
        let (compartment_1, compartment_2) = line.split_at(len / 2);

        let double_item = compartment_1
            .chars()
            .find(|&p| compartment_2.contains(p))
            .unwrap();

        let value;
        if double_item.is_uppercase() {
            value = double_item as u8 - b'A' + 27;
        } else {
            value = double_item as u8 - b'a' + 1;
        }
        total += value as u64;
    }
    println!("{}", total);
}
