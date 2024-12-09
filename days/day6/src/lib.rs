use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn a() -> i32 {
    let mut map = HashMap::new();

    let mut start = (0, 0);

    let mut dir = Dir::Up;

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line
            .chars()
            .filter(|c| *c != '\r' || *c != '\n')
            .enumerate()
        {
            map.insert((row as i32, col as i32), c);
            if c == '^' {
                start = (row as i32, col as i32);
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(start);

    loop {
        let look_ahead = match dir {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };
        if let Some(value) = map.get(&(start.0 + look_ahead.0, start.1 + look_ahead.1)) {
            if *value == '#' {
                dir = match dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                };

                visited.insert(start);
            } else {
                visited.insert(start);
            }
        } else {
            visited.insert(start);

            break;
        }

        match dir {
            Dir::Up => start.0 -= 1,
            Dir::Down => start.0 += 1,
            Dir::Left => start.1 -= 1,
            Dir::Right => start.1 += 1,
        }
    }

    visited.len() as i32
}

pub fn b() -> i32 {
    let mut map = HashMap::new();

    let mut start = (0, 0);

    let mut dir;

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line
            .chars()
            .filter(|c| *c != '\r' || *c != '\n')
            .enumerate()
        {
            map.insert((row as i32, col as i32), c);
            if c == '^' {
                start = (row as i32, col as i32);
            }
        }
    }

    let mut count = 0;
    for xy in map.keys() {
        let mut modded_map = map.clone();
        if modded_map.get(xy) == Some(&'^') || modded_map.get(xy) == Some(&'#') {
            continue;
        }
        modded_map.insert(*xy, '#');

        let mut new_start = start;
        dir = Dir::Up;

        let time = Instant::now();

        'inner: loop {
            if time.elapsed() > Duration::from_millis(2) {
                count += 1;
                break 'inner;
            }

            let look_ahead = match dir {
                Dir::Up => (-1, 0),
                Dir::Down => (1, 0),
                Dir::Left => (0, -1),
                Dir::Right => (0, 1),
            };
            if let Some(value) =
                modded_map.get(&(new_start.0 + look_ahead.0, new_start.1 + look_ahead.1))
            {
                if *value == '#' {
                    dir = match dir {
                        Dir::Up => Dir::Right,
                        Dir::Right => Dir::Down,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Up,
                    };
                }
            } else {
                break 'inner;
            }

            // Walk
            loop {
                let look_ahead = match dir {
                    Dir::Up => (-1, 0),
                    Dir::Down => (1, 0),
                    Dir::Left => (0, -1),
                    Dir::Right => (0, 1),
                };

                if let Some(value) =
                    modded_map.get(&(new_start.0 + look_ahead.0, new_start.1 + look_ahead.1))
                {
                    if *value != '#' {
                        match dir {
                            Dir::Up => new_start.0 -= 1,
                            Dir::Down => new_start.0 += 1,
                            Dir::Left => new_start.1 -= 1,
                            Dir::Right => new_start.1 += 1,
                        }
                        break;
                    } else {
                        dir = match dir {
                            Dir::Up => Dir::Right,
                            Dir::Right => Dir::Down,
                            Dir::Down => Dir::Left,
                            Dir::Left => Dir::Up,
                        };
                    }
                }
            }
        }
    }

    count
}
