const INPUT: &str = include_str!("input.txt");

struct Map<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Map<T> {
    #[inline(always)]
    fn get_unchecked(&self, pos: &(usize, usize)) -> &T {
        unsafe { self.data.get_unchecked(pos.0 * self.cols + pos.1) }
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

#[inline(always)]
fn find_trail_a(
    pos: (usize, usize),
    map: &Map<u8>,
    used: &mut [u32],
    depth: u8,
    generation: u32,
) -> u32 {
    if depth == 9 {
        let index = pos.0 * map.cols + pos.1;
        if used[index] == generation {
            return 0;
        } else {
            used[index] = generation;
            return 1;
        }
    }

    let mut score = 0;

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
        let next = map.get_unchecked(&next_pos);

        if *next == depth + 1 {
            score += find_trail_a(next_pos, map, used, depth + 1, generation);
        }
    }

    score
}

pub fn a() -> u32 {
    let mut lines_iter = INPUT.lines();
    let first_line = lines_iter.next().unwrap();
    let rows = 1 + lines_iter.clone().count();
    let cols = first_line.len();

    let mut data = vec![0u8; rows * cols];
    let mut starting_positions = Vec::with_capacity(256);

    {
        let bytes = first_line.as_bytes();
        for col in 0..cols {
            let value = bytes[col] - b'0';
            if value == 0 {
                starting_positions.push((0, col));
            }
            data[col] = value;
        }
    }

    for (row, line) in lines_iter.enumerate() {
        let row = row + 1;
        let bytes = line.as_bytes();

        for col in 0..cols {
            let value = bytes[col] - b'0';
            if value == 0 {
                starting_positions.push((row, col));
            }
            data[row * cols + col] = value;
        }
    }

    let map = Map { rows, cols, data };

    let mut score = 0;
    let mut used = vec![0; rows * cols];
    for (generation, start) in starting_positions.into_iter().enumerate() {
        score += find_trail_a(start, &map, &mut used, 0, (generation + 1) as u32);
    }

    score
}

#[inline(always)]
fn find_trail_b(pos: (usize, usize), map: &Map<u8>, depth: u8) -> u32 {
    if depth == 9 {
        return 1;
    }

    let mut score = 0;

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
        let next = map.get_unchecked(&next_pos);

        if *next == depth + 1 {
            score += find_trail_b(next_pos, map, depth + 1);
        }
    }

    score
}

pub fn b() -> u32 {
    let mut lines_iter = INPUT.lines();
    let first_line = lines_iter.next().unwrap();
    let rows = 1 + lines_iter.clone().count();
    let cols = first_line.len();

    let mut data = vec![0u8; rows * cols];
    let mut starting_positions = Vec::with_capacity(256);

    {
        let bytes = first_line.as_bytes();
        for col in 0..cols {
            let value = bytes[col] - b'0';
            if value == 0 {
                starting_positions.push((0, col));
            }
            data[col] = value;
        }
    }

    for (row, line) in lines_iter.enumerate() {
        let row = row + 1;
        let bytes = line.as_bytes();

        for col in 0..cols {
            let value = bytes[col] - b'0';
            if value == 0 {
                starting_positions.push((row, col));
            }
            data[row * cols + col] = value;
        }
    }

    let map = Map { rows, cols, data };

    let mut score = 0;
    for start in starting_positions {
        score += find_trail_b(start, &map, 0);
    }

    score
}
