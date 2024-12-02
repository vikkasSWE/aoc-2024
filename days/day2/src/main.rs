fn main() {
    let input = include_str!("input.txt");

    let mut safe_count = 0;
    for line_str in input.lines() {
        let line = line_str
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (is_safe, unsafe_count) = check_line(line.clone());

        if is_safe {
            safe_count += 1;
            continue;
        } else {
            println!("{:?}", line);
        }

        for i in 0..line.len() {
            let new_line: Vec<_> = line
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &item)| item)
                .collect();

            let (is_safe, unsafe_count) = check_line(new_line.clone());
            if is_safe {
                safe_count += 1;
                break;
            }
        }
    }

    println!("{}", safe_count);

    fn check_line(line: Vec<i32>) -> (bool, i32) {
        let mut increasing = Vec::new();
        let mut safe = true;
        let mut unsafe_count = 0;
        for pair in line.windows(2) {
            let a = pair[0];
            let b = pair[1];

            let diff = a - b;

            if diff.is_positive() {
                increasing.push(true);
            } else {
                increasing.push(false);
            }

            if diff == 0 || diff.abs() > 3 {
                safe = false;
                unsafe_count += 1;
            }
        }

        let all_increasing = increasing.iter().all(|v| *v);
        let all_decreasing = increasing.iter().all(|v| !*v);

        let dir = if all_increasing && !all_decreasing {
            true
        } else {
            all_decreasing && !all_increasing
        };

        (safe && dir, unsafe_count)
    }

    // let mut safe_count = 0;

    // for line in input.lines() {
    //     let mut safe = true;

    //     let line = line
    //         .split_whitespace()
    //         .map(|v| v.parse::<i32>().unwrap())
    //         .collect::<Vec<_>>();

    //     let mut last = None;
    //     let dir = (line[0] - line[1]).is_positive();

    //     let mut decreasing = false;
    //     let mut increasing = false;

    //     for pair in line.windows(2) {
    //         let a = pair[0];
    //         let b = pair[1];

    //         let diff = a - b;

    //         if diff < 0 {
    //             decreasing = true;
    //         } else if diff > 0 {
    //             increasing = true;
    //         }
    //     }

    //     if decreasing && increasing || !decreasing && !increasing {
    //         continue;
    //     }

    //     for curr in &line {
    //         let Some(last) = last.as_mut() else {
    //             last = Some(curr);
    //             continue;
    //         };

    //         if dir != (*last - curr).is_positive() {
    //             safe = false;
    //         }

    //         if (*last - curr).abs() > 3 || *last - curr == 0 {
    //             println!("Non safe: {} -> {}", last, curr);
    //             safe = false;
    //         }

    //         *last = curr;
    //     }

    //     if safe {
    //         safe_count += 1;
    //     }
    // }

    // println!("{}", safe_count)
}
