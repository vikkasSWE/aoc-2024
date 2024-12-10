const INPUT: &str = include_str!("input.txt");

struct Map<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Map<T> {
    fn get(&self, pos: &(usize, usize)) -> Option<&T> {
        if pos.0 < self.rows && pos.1 < self.cols {
            Some(&self.data[pos.0 * self.cols + pos.1])
        } else {
            None
        }
    }
}

fn find_trail_a(pos: (usize, usize), map: &Map<u32>, used: &mut [bool], depth: u32) -> u32 {
    if depth == 9 {
        let index = pos.0 * map.cols + pos.1;
        if used[index] {
            return 0;
        } else {
            used[index] = true;
            return 1;
        }
    }

    let mut score = 0;

    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    for (dir_row, dir_col) in &DIRECTIONS {
        let next_row = pos.0 as isize + dir_row;
        let next_col = pos.1 as isize + dir_col;

        if next_row < 0
            || next_row >= map.rows as isize
            || next_col < 0
            || next_col >= map.cols as isize
        {
            continue;
        }

        let next_pos = (next_row as usize, next_col as usize);
        if let Some(&next) = map.get(&next_pos) {
            if next == depth + 1 {
                score += find_trail_a(next_pos, map, used, depth + 1);
            }
        }
    }

    score
}

pub fn a() -> u32 {
    let lines: Vec<&str> = INPUT.lines().collect();
    let rows = lines.len();
    let cols = if rows > 0 { lines[0].len() } else { 0 };

    let mut data = Vec::with_capacity(rows * cols);
    let mut starting_positions = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap_or(20);
            if value == 0 {
                starting_positions.push((row, col));
            }
            data.push(value);
        }
    }

    let map = Map { rows, cols, data };

    let mut score = 0;

    for start in starting_positions {
        let mut used = vec![false; rows * cols];
        score += find_trail_a(start, &map, &mut used, 0);
    }

    score
}

fn find_trail_b(pos: (usize, usize), map: &Map<u32>, depth: u32) -> u32 {
    if depth == 9 {
        return 1;
    }

    let mut score = 0;

    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    for (dir_row, dir_col) in &DIRECTIONS {
        let next_row = pos.0 as isize + dir_row;
        let next_col = pos.1 as isize + dir_col;

        if next_row < 0
            || next_row >= map.rows as isize
            || next_col < 0
            || next_col >= map.cols as isize
        {
            continue;
        }

        let next_pos = (next_row as usize, next_col as usize);
        if let Some(&next) = map.get(&next_pos) {
            if next == depth + 1 {
                score += find_trail_b(next_pos, map, depth + 1);
            }
        }
    }

    score
}

pub fn b() -> u32 {
    let lines: Vec<&str> = INPUT.lines().collect();
    let rows = lines.len();
    let cols = if rows > 0 { lines[0].len() } else { 0 };

    let mut data = Vec::with_capacity(rows * cols);
    let mut starting_positions = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap_or(20);
            if value == 0 {
                starting_positions.push((row, col));
            }
            data.push(value);
        }
    }

    let map = Map { rows, cols, data };

    let mut score = 0;
    for start in starting_positions {
        score += find_trail_b(start, &map, 0);
    }

    score
}
