use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut positions_touched = BTreeSet::new();
    positions_touched.insert(tail);

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let amount: i32 = split.next().unwrap().parse().unwrap();

        for _ in 0..amount {
            match direction {
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "L" => head.0 -= 1,
                "D" => head.1 -= 1,
                other => {
                    panic!("{}", other)
                }
            }

            if distance_from(&head, &tail) > 1 {
                move_tail(&head, &mut tail);
                positions_touched.insert(tail);
            }
        }
    }

    println!("{}", positions_touched.len());
}

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    if head.0 == tail.0 {
        tail.1 += if (head.1 - tail.1).is_positive() {
            1
        } else {
            -1
        };
    } else if head.1 == tail.1 {
        tail.0 += if (head.0 - tail.0).is_positive() {
            1
        } else {
            -1
        };
    } else {
        tail.0 += if (head.0 - tail.0).is_positive() {
            1
        } else {
            -1
        };
        tail.1 += if (head.1 - tail.1).is_positive() {
            1
        } else {
            -1
        };
    }
}

fn distance_from(head: &(i32, i32), tail: &(i32, i32)) -> i32 {
    (((head.0 - tail.0).abs().pow(2) + (head.1 - tail.1).abs().pow(2)) as f32)
        .sqrt()
        .round() as i32
}
