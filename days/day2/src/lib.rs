use std::time::Instant;

fn check_line_iter(mut line: impl Iterator<Item = i16>) -> bool {
    let first = line.next().unwrap();
    let mut prev = line.next().unwrap();
    let mut diff = first - prev;
    let increasing = diff.is_positive();
    if diff == 0 || diff.abs() > 3 || increasing != diff.is_positive() {
        return false;
    }

    for next in line.by_ref() {
        diff = prev - next;
        if diff == 0 || diff.abs() > 3 || increasing != diff.is_positive() {
            return false;
        }

        prev = next;
    }

    true
}

const INPUT: &str = include_str!("input.txt");
pub fn a() {
    let start = Instant::now();

    let mut safe_count = 0;
    let mut safe_count_part1 = 0;
    INPUT.lines().for_each(|line_str| {
        let line = line_str
            .split_whitespace()
            .map(|v| v.parse::<i16>().unwrap());

        if check_line_iter(line.clone()) {
            safe_count += 1;
            safe_count_part1 += 1;
        } else {
            let count = line.clone().count();
            for i in 0..count {
                let mut new_line = line
                    .clone()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, item)| item);

                if check_line_iter(&mut new_line) {
                    safe_count += 1;
                    break;
                }
            }
        }
    });

    let end = start.elapsed().as_micros();

    println!("{}", end);

    println!("Part 1: {}", safe_count_part1);
    println!("Part 2: {}", safe_count);
}
