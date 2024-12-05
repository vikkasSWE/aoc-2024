use std::{cmp::Ordering, collections::HashSet, time::Instant};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let mut updates = Vec::new();
    let mut rules = HashSet::new();

    let mut first_section = true;
    for line in INPUT.lines() {
        if line.is_empty() {
            first_section = false;
            continue;
        }

        if first_section {
            let mut split = line.split("|");

            let a = split.next().unwrap().parse::<i32>().unwrap();
            let b = split.next().unwrap().parse::<i32>().unwrap();

            rules.insert((a, b));
        } else {
            let update = line
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(update);
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.iter_mut() {
        let is_correct = update
            .windows(2)
            .all(|window| rules.contains(&(window[0], window[1])));

        if is_correct {
            let middle = update[update.len() / 2];

            p1 += middle;
        } else {
            update.sort_unstable_by(|v1, v2| {
                if rules.contains(&(*v1, *v2)) {
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
    println!("Part 1: {p1}, Correct: 4924");
    println!("Part 2: {p2}, Correct: 6085");
}
