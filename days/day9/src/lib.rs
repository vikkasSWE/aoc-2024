use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

enum Space {
    File(usize),
    Free,
}

pub fn a() -> usize {
    let mut seq = INPUT
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    seq.push(0);

    let mut files = VecDeque::with_capacity(65_535);
    for (id, chunk) in seq.chunks(2).enumerate() {
        let file_blocks = chunk[0];
        let free_space = chunk[1];

        for _ in 0..file_blocks {
            files.push_back(Space::File(id));
        }

        for _ in 0..free_space {
            files.push_back(Space::Free);
        }
    }

    let mut index = 0;
    let mut count = 0;

    loop {
        match files.pop_front() {
            Some(Space::File(id)) => {
                count += id * index;
                index += 1;

                continue;
            }
            Some(Space::Free) => {
                // Loop rev until file is found
                loop {
                    match files.pop_back() {
                        Some(Space::File(id)) => {
                            count += id * index;
                            break;
                        }
                        Some(_) => continue,
                        None => break,
                    }
                }
                index += 1;
            }

            None => break,
        }
    }

    count
}

struct File {
    size: usize,
    free: usize,
}

pub fn b() -> usize {
    let mut seq = INPUT
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    seq.push(0);
    let max_id = seq.len();

    let mut entries = HashMap::with_capacity(65_535);
    for (id, chunk) in seq.chunks(2).enumerate() {
        let file_blocks = chunk[0] as usize;
        let free_space = chunk[1] as usize;

        entries.insert(
            id,
            File {
                size: file_blocks,
                free: free_space,
            },
        );
    }

    let mut file_id = 0;
    let mut index = 0;
    let mut count = 0;
    let mut moved_ids = HashSet::new();

    while let Some(file) = entries.remove(&file_id) {
        let mut free_space = file.free;

        if !moved_ids.contains(&file_id) {
            for _ in 0..file.size {
                count += file_id * index;
                index += 1;
            }
        } else {
            for _ in 0..file.size {
                index += 1;
            }
        }

        while let Some((id, removed)) =
            find_entry_that_fits(&entries, free_space, max_id, file_id, &moved_ids)
        {
            moved_ids.insert(id);

            for _ in 0..removed.size {
                count += id * index;
                index += 1;
            }

            free_space -= removed.size;
        }

        index += free_space;
        file_id += 1;
    }

    assert!(count == 6467290479134);

    count
}

fn find_entry_that_fits<'a>(
    entries: &'a HashMap<usize, File>,
    free_space: usize,
    max_id: usize,
    min_id: usize,
    moved_ids: &HashSet<usize>,
) -> Option<(usize, &'a File)> {
    let mut file_id = max_id;

    while file_id != min_id {
        if moved_ids.contains(&file_id) {
            file_id -= 1;
            continue;
        }

        let Some(entry) = entries.get(&file_id) else {
            file_id -= 1;
            continue;
        };

        if entry.size <= free_space {
            return Some((file_id, entry));
        }

        file_id -= 1;
    }

    None
}
