use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut count = 0;

    for line in input.lines() {
        let mut split = line.split(",");
        let (elf_1, elf_2) = (split.next().unwrap(), split.next().unwrap());

        let mut split = elf_1.split("-");
        let (elf_1_begin, elf_1_end) = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        let mut split = elf_2.split("-");
        let (elf_2_begin, elf_2_end) = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );

        if (elf_1_begin >= elf_2_begin && elf_1_end <= elf_2_end
            || elf_1_begin <= elf_2_begin && elf_1_end >= elf_2_end)
            || (elf_1_begin <= elf_2_begin && elf_2_begin <= elf_1_end
                || elf_1_begin <= elf_2_end && elf_2_end <= elf_1_end)
        {
            count += 1;
        }
    }

    println!("{}", count);
}
