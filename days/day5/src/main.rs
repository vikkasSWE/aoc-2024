use std::{cmp::Ordering, time::Instant};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let mut updates = Vec::new();
    let mut rules = Vec::new();

    let mut first_section = true;
    for line in INPUT.lines() {
        if line.is_empty() {
            first_section = false;
            continue;
        }

        if first_section {
            let mut split = line.split_terminator("|");

            let a = split.next().unwrap().parse::<i32>().unwrap();
            let b = split.next().unwrap().parse::<i32>().unwrap();

            rules.push((a, b));
        } else {
            let split = line.split_terminator(",");

            let mut update = Vec::new();

            for v in split {
                update.push(v.parse::<i32>().unwrap());
            }

            updates.push(update);
        }
    }

    println!("half: {}", start.elapsed().as_micros());

    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.iter_mut() {
        let mut is_correct = true;

        for window in update.windows(2) {
            let prev = window[0];
            let next = window[1];

            if !rules.iter().any(|(r1, r2)| prev == *r1 && next == *r2) {
                is_correct = false;
                break;
            }
        }

        if is_correct {
            let middle = update[update.len() / 2];
            assert!(update.len() % 2 == 1); // All Odd

            p1 += middle;
        } else {
            update.sort_by(|v1, v2| {
                if rules.iter().any(|(r1, r2)| *v1 == *r1 && *v2 == *r2) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let middle = update[update.len() / 2];
            p2 += middle;
        }
    }
    println!("Time: {}", start.elapsed().as_micros());
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
