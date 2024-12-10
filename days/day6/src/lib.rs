use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

struct Map<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T> Map<T> {
    #[inline(always)]
    fn get(&self, pos: &(usize, usize)) -> &T {
        unsafe { self.data.get_unchecked(pos.0 * self.cols + pos.1) }
    }

    #[inline(always)]
    fn get_mut(&mut self, pos: &(usize, usize)) -> &mut T {
        unsafe { self.data.get_unchecked_mut(pos.0 * self.cols + pos.1) }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Dir {
    const DIR_VECTORS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    #[inline(always)]
    fn turn_right(self) -> Dir {
        unsafe { std::mem::transmute(((self as u8) + 1) & 3) }
    }

    #[inline(always)]
    fn vector(self) -> (isize, isize) {
        Self::DIR_VECTORS[self as usize]
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
    (pos.0 * cols + pos.1) * 4 + dir as usize
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

    let mut visited_locations = HashSet::with_capacity(8192);

    {
        let mut dir = Dir::Up;
        let mut pos = start;

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
            } else {
                pos = (next.0, next.1);
                visited_locations.insert(pos);
            }
        }
    }

    let mut count = 0;
    let mut visited = vec![0; rows * cols * 4];
    let mut current_generation = 1;

    for xy in visited_locations {
        current_generation += 1;

        let original = *map.get(&xy);
        *map.get_mut(&xy) = '#';

        let mut new_start = start;
        let mut dir = Dir::Up;

        'inner: loop {
            let index = visited_index(cols, &new_start, dir);
            if visited[index] == current_generation {
                count += 1;
                break 'inner;
            } else {
                visited[index] = current_generation;
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
    // }

    count
}
