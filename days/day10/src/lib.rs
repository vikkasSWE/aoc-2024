use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

pub fn a() -> u32 {
    let mut map = HashMap::new();
    let mut starting_positions = Vec::new();

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap_or(20);
            map.insert((row, col), value);
            if value == 0 {
                starting_positions.push((row, col));
            }
        }
    }

    let mut score = 0;
    for start in starting_positions {
        let mut used = HashSet::new();
        let res = find_trail(start, &map, &mut used, 0);
        score += res;
    }

    score
}

fn find_trail(
    pos: (usize, usize),
    map: &HashMap<(usize, usize), u32>,
    used: &mut HashSet<(usize, usize)>,
    depth: u32,
) -> u32 {
    if depth == 9 {
        if used.contains(&pos) {
            return 0;
        } else {
            used.insert(pos);
            return 1;
        }
    }

    let mut score = 0;

    let down_one = (pos.0 + 1, pos.1);
    let right_one = (pos.0, pos.1 + 1);
    let left_one = (pos.0, pos.1 - 1);
    let up_one = (pos.0 - 1, pos.1);

    if let Some(down) = map.get(&down_one) {
        if *down == depth + 1 {
            score += find_trail(down_one, map, used, depth + 1);
        }
    }

    if let Some(right) = map.get(&right_one) {
        if *right == depth + 1 {
            score += find_trail(right_one, map, used, depth + 1);
        }
    }

    if let Some(left) = map.get(&left_one) {
        if *left == depth + 1 {
            score += find_trail(left_one, map, used, depth + 1);
        }
    }

    if let Some(up) = map.get(&up_one) {
        if *up == depth + 1 {
            score += find_trail(up_one, map, used, depth + 1);
        }
    }

    score
}

pub fn b() -> usize {
    0
}
