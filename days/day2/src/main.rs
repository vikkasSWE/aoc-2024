fn check_line(line: &[i32]) -> bool {
    let mut increasing = Vec::new();
    let mut safe = true;
    for pair in line.windows(2) {
        let a = pair[0];
        let b = pair[1];

        let diff = a - b;

        increasing.push(diff.is_positive());

        if diff == 0 || diff.abs() > 3 {
            safe = false;
        }
    }

    let all_increasing = increasing.iter().all(|v| *v);
    let all_decreasing = increasing.iter().all(|v| !*v);

    safe && all_increasing ^ all_decreasing
}

fn main() {
    let input = include_str!("input.txt");

    let mut safe_count = 0;
    let mut safe_count_part1 = 0;
    for line_str in input.lines() {
        let line = line_str
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if check_line(&line) {
            safe_count += 1;
            safe_count_part1 += 1;
            continue;
        }

        for i in 0..line.len() {
            let new_line: Vec<_> = line
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &item)| item)
                .collect();

            if check_line(&new_line) {
                safe_count += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", safe_count_part1);
    println!("Part 2: {}", safe_count);
}
