const INPUT: &str = include_str!("input.txt");

struct Map<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T> Map<T> {
    fn get(&self, pos: &(usize, usize)) -> &T {
        &self.data[pos.0 * self.cols + pos.1]
    }

    fn get_mut(&mut self, pos: &(usize, usize)) -> &mut T {
        &mut self.data[pos.0 * self.cols + pos.1]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn vector(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

fn visit_position(visited: &mut [bool], cols: usize, pos: (usize, usize), count: &mut i32) {
    let index = pos.0 * cols + pos.1;
    if !visited[index] {
        visited[index] = true;
        *count += 1;
    }
}

pub fn a() -> i32 {
    let lines: Vec<&str> = INPUT.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut data = Vec::with_capacity(rows * cols);
    let mut start = (0, 0);

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            data.push(c);
            if c == '^' {
                start = (row, col);
            }
        }
    }

    let map = Map { cols, data };

    let mut visited = vec![false; rows * cols];
    let mut visited_count = 0;

    let mut dir = Dir::Up;
    let mut pos = start;

    visit_position(&mut visited, cols, pos, &mut visited_count);

    loop {
        let dir_vec = dir.vector();
        let next = (
            (pos.0 as isize + dir_vec.0) as usize,
            (pos.1 as isize + dir_vec.1) as usize,
        );

        if next.0 >= rows || next.1 >= cols {
            break;
        }

        let ahead_value = *map.get(&next);
        if ahead_value == '#' {
            dir = dir.turn_right();
            visit_position(&mut visited, cols, pos, &mut visited_count);
        } else {
            pos = (next.0, next.1);
            visit_position(&mut visited, cols, pos, &mut visited_count);
        }
    }

    visited_count
}

fn visited_index(cols: usize, pos: &(usize, usize), dir: Dir) -> usize {
    let dir_index = match dir {
        Dir::Up => 0,
        Dir::Right => 1,
        Dir::Down => 2,
        Dir::Left => 3,
    };
    (pos.0 * cols + pos.1) * 4 + dir_index
}

pub fn b() -> i32 {
    let lines: Vec<&str> = INPUT.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut data = Vec::with_capacity(rows * cols);

    let mut start = (0, 0);
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            data.push(c);
            if c == '^' {
                start = (row, col);
            }
        }
    }

    let mut map = Map { cols, data };

    let mut count = 0;
    let mut visited = vec![false; rows * cols * 4];

    for row in 0..rows {
        for col in 0..cols {
            let xy = (row, col);

            let original = *map.get(&xy);
            *map.get_mut(&xy) = '#';

            let mut new_start = start;
            let mut dir = Dir::Up;

            visited.fill(false);

            'inner: loop {
                let idx = visited_index(cols, &new_start, dir);
                if visited[idx] {
                    count += 1;
                    break 'inner;
                } else {
                    visited[idx] = true;
                }

                let dir_vec = dir.vector();
                let ahead = (
                    (new_start.0 as isize + dir_vec.0) as usize,
                    (new_start.1 as isize + dir_vec.1) as usize,
                );

                if ahead.0 >= rows || ahead.1 >= cols {
                    break 'inner;
                }

                let ahead_value = *map.get(&ahead);
                if ahead_value == '#' {
                    dir = dir.turn_right();
                } else {
                    loop {
                        let dir_vec = dir.vector();
                        let next = (
                            (new_start.0 as isize + dir_vec.0) as usize,
                            (new_start.1 as isize + dir_vec.1) as usize,
                        );
                        if next.0 >= rows || next.1 >= cols {
                            break 'inner;
                        }

                        let map_value = *map.get(&next);
                        if map_value != '#' {
                            new_start = next;
                            break;
                        } else {
                            dir = dir.turn_right();
                        }
                    }
                }
            }

            *map.get_mut(&xy) = original;
        }
    }

    count
}
