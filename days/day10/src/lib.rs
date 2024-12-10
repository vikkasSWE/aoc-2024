use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

pub fn a() -> u32 {
    let mut map: HashMap<(isize, isize), u32> = HashMap::new();
    let mut starting_positions = Vec::new();

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap_or(20);
            map.insert((row as isize, col as isize), value);
            if value == 0 {
                starting_positions.push((row as isize, col as isize));
            }
        }
    }

    let mut score = 0;
    for start in starting_positions {
        let mut used = HashSet::new();
        let res = find_trail(start, &map, &mut used, 0, false);
        score += res;
    }

    score
}

fn find_trail(
    pos: (isize, isize),
    map: &HashMap<(isize, isize), u32>,
    used: &mut HashSet<(isize, isize)>,
    depth: u32,
    part_b: bool,
) -> u32 {
    if depth == 9 {
        if used.contains(&pos) {
            return part_b as u32;
        } else {
            used.insert(pos);
            return 1;
        }
    }

    let mut score = 0;

    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

    for dir in DIRECTIONS {
        let ahead = (pos.0 + dir.0, pos.1 + dir.1);

        if let Some(next) = map.get(&ahead) {
            if *next == depth + 1 {
                score += find_trail((ahead.0, ahead.1), map, used, depth + 1, part_b);
            }
        }
    }

    score
}

pub fn b() -> u32 {
    let mut map = HashMap::new();
    let mut starting_positions = Vec::new();

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap_or(20);
            map.insert((row as isize, col as isize), value);
            if value == 0 {
                starting_positions.push((row as isize, col as isize));
            }
        }
    }

    let mut score = 0;
    for start in starting_positions {
        let mut used = HashSet::new();
        let res = find_trail(start, &map, &mut used, 0, true);
        score += res;
    }

    score
}
