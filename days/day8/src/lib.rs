use std::collections::{hash_map::Entry, HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

pub fn a() -> i32 {
    let mut map = HashMap::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line
            .chars()
            .filter(|c| *c != '\n' || *c != '\r')
            .enumerate()
        {
            map.insert((row, col), c);
            if c != '.' {
                if let Some(antenna) = antennas.get_mut(&c) {
                    antenna.push((row, col));
                } else {
                    antennas.insert(c, vec![(row, col)]);
                }
            }
        }
    }

    let mut count = HashSet::new();
    for (_, positions) in antennas {
        for i in 0..positions.len() {
            let mut others = positions.clone();
            others.remove(i);
            let pos = positions[i];

            for other in others {
                let diff_row = pos.0 as i32 - other.0 as i32;
                let diff_col = pos.1 as i32 - other.1 as i32;

                let res_row = pos.0 as i32 + diff_row;
                let res_col = pos.1 as i32 + diff_col;

                if res_row >= 0 && res_col >= 0 {
                    let anti_1 = (res_row as usize, res_col as usize);
                    if let Entry::Occupied(mut e) = map.entry(anti_1) {
                        e.insert('#');
                        count.insert(anti_1);
                    }
                }
            }
        }
    }

    count.len() as i32
}

pub fn b() -> i32 {
    let mut map = HashMap::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row, line) in INPUT.lines().enumerate() {
        for (col, c) in line
            .chars()
            .filter(|c| *c != '\n' || *c != '\r')
            .enumerate()
        {
            map.insert((row, col), c);
            if c != '.' {
                if let Some(antenna) = antennas.get_mut(&c) {
                    antenna.push((row, col));
                } else {
                    antennas.insert(c, vec![(row, col)]);
                }
            }
        }
    }

    let mut count = HashSet::new();
    for (_, positions) in antennas {
        for i in 0..positions.len() {
            let mut others = positions.clone();
            others.remove(i);
            count.insert(positions[i]);

            for other in others {
                let mut pos = positions[i];
                let diff_row = pos.0 as i32 - other.0 as i32;
                let diff_col = pos.1 as i32 - other.1 as i32;
                loop {
                    let res_row = pos.0 as i32 + diff_row;
                    let res_col = pos.1 as i32 + diff_col;

                    if res_row >= 0 && res_col >= 0 {
                        let anti = (res_row as usize, res_col as usize);

                        #[allow(clippy::map_entry)]
                        if map.contains_key(&anti) {
                            map.insert(anti, '#');
                            count.insert(anti);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    pos = (res_row as usize, res_col as usize);
                }
            }
        }
    }

    count.len() as i32
}
