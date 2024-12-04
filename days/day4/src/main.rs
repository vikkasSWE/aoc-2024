use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let startup = Instant::now();
    let input = INPUT.trim_end_matches("\n");

    let mut p1 = 0;
    let mut p2 = 0;

    let array = input
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();

    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let startup_end = startup.elapsed().as_micros();
    let p1_start = Instant::now();

    for (row, _) in (0..num_rows).enumerate() {
        if row == num_rows - 3 {
            break;
        }

        let mut window = [['.'; 4]; 4];
        for (col, _) in (0..num_cols).enumerate() {
            if col == num_cols - 3 {
                break;
            }
            window = {
                [
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 0) * num_cols + col + 0],
                        array[(row + 0) * num_cols + col + 1],
                        array[(row + 0) * num_cols + col + 2],
                        array[(row + 0) * num_cols + col + 3],
                    ],
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 1) * num_cols + col + 0],
                        array[(row + 1) * num_cols + col + 1],
                        array[(row + 1) * num_cols + col + 2],
                        array[(row + 1) * num_cols + col + 3],
                    ],
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 2) * num_cols + col + 0],
                        array[(row + 2) * num_cols + col + 1],
                        array[(row + 2) * num_cols + col + 2],
                        array[(row + 2) * num_cols + col + 3],
                    ],
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 3) * num_cols + col + 0],
                        array[(row + 3) * num_cols + col + 1],
                        array[(row + 3) * num_cols + col + 2],
                        array[(row + 3) * num_cols + col + 3],
                    ],
                ]
            };

            // Diagonals

            if window[0][0] == 'x'
                && window[1][1] == 'm'
                && window[2][2] == 'a'
                && window[3][3] == 's'
            {
                p1 += 1;
            }

            if window[3][3] == 'x'
                && window[2][2] == 'm'
                && window[1][1] == 'a'
                && window[0][0] == 's'
            {
                p1 += 1;
            }

            if window[0][3] == 'x'
                && window[1][2] == 'm'
                && window[2][1] == 'a'
                && window[3][0] == 's'
            {
                p1 += 1;
            }

            if window[3][0] == 'x'
                && window[2][1] == 'm'
                && window[1][2] == 'a'
                && window[0][3] == 's'
            {
                p1 += 1;
            }

            // Verticals
            if window[0][0] == 'x'
                && window[1][0] == 'm'
                && window[2][0] == 'a'
                && window[3][0] == 's'
            {
                p1 += 1;
            }

            if window[3][0] == 'x'
                && window[2][0] == 'm'
                && window[1][0] == 'a'
                && window[0][0] == 's'
            {
                p1 += 1;
            }
        }

        // last verticals
        for i in 1..4 {
            if window[0][i] == 'x'
                && window[1][i] == 'm'
                && window[2][i] == 'a'
                && window[3][i] == 's'
            {
                p1 += 1;
            }

            if window[3][i] == 'x'
                && window[2][i] == 'm'
                && window[1][i] == 'a'
                && window[0][i] == 's'
            {
                p1 += 1;
            }
        }
    }

    for line in input.lines() {
        for a in line.chars().collect::<Vec<_>>().windows(4) {
            if a[0] == 'X' && a[1] == 'M' && a[2] == 'A' && a[3] == 'S' {
                p1 += 1;
            }

            if a[3] == 'X' && a[2] == 'M' && a[1] == 'A' && a[0] == 'S' {
                p1 += 1;
            }
        }
    }

    let p1_end = p1_start.elapsed().as_micros();

    let p2_start = Instant::now();

    for (row, _) in (0..num_rows).enumerate() {
        if row == num_rows - 2 {
            break;
        }

        for (col, _) in (0..num_cols).enumerate() {
            if col == num_cols - 2 {
                break;
            }

            let window = {
                [
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 0) * num_cols + col + 0],
                        array[(row + 0) * num_cols + col + 1],
                        array[(row + 0) * num_cols + col + 2],
                    ],
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 1) * num_cols + col + 0],
                        array[(row + 1) * num_cols + col + 1],
                        array[(row + 1) * num_cols + col + 2],
                    ],
                    #[allow(clippy::identity_op)]
                    [
                        array[(row + 2) * num_cols + col + 0],
                        array[(row + 2) * num_cols + col + 1],
                        array[(row + 2) * num_cols + col + 2],
                    ],
                ]
            };

            // Diagonals
            if ((window[0][0] == 'm' && window[1][1] == 'a' && window[2][2] == 's')
                || (window[2][2] == 'm' && window[1][1] == 'a' && window[0][0] == 's'))
                && ((window[0][2] == 'm' && window[1][1] == 'a' && window[2][0] == 's')
                    || (window[2][0] == 'm' && window[1][1] == 'a' && window[0][2] == 's'))
            {
                p2 += 1;
            }
        }
    }

    let p2_end = p2_start.elapsed().as_micros();

    println!("Part 1: {}, time: {}us", p1, p1_end + startup_end);
    println!("Part 2: {}, time: {}us", p2, p2_end + startup_end)
}
