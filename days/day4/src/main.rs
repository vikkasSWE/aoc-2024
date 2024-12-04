const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.trim_end_matches("\n");

    let mut count = 0;

    let array = input
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();

    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    println!("{},{}", num_rows, num_cols);

    let mut row = 0;
    for _ in 0..num_rows {
        if row == num_rows - 3 {
            break;
        }

        let mut col = 0;
        let mut window = [['.'; 4]; 4];
        for _ in 0..num_cols {
            if col == num_cols - 3 {
                break;
            }

            window = {
                [
                    [
                        array[(row + 0) * num_cols + col + 0],
                        array[(row + 0) * num_cols + col + 1],
                        array[(row + 0) * num_cols + col + 2],
                        array[(row + 0) * num_cols + col + 3],
                    ],
                    [
                        array[(row + 1) * num_cols + col + 0],
                        array[(row + 1) * num_cols + col + 1],
                        array[(row + 1) * num_cols + col + 2],
                        array[(row + 1) * num_cols + col + 3],
                    ],
                    [
                        array[(row + 2) * num_cols + col + 0],
                        array[(row + 2) * num_cols + col + 1],
                        array[(row + 2) * num_cols + col + 2],
                        array[(row + 2) * num_cols + col + 3],
                    ],
                    [
                        array[(row + 3) * num_cols + col + 0],
                        array[(row + 3) * num_cols + col + 1],
                        array[(row + 3) * num_cols + col + 2],
                        array[(row + 3) * num_cols + col + 3],
                    ],
                ]
            };

            // Diagonals
            {
                if window[0][0] == 'x'
                    && window[1][1] == 'm'
                    && window[2][2] == 'a'
                    && window[3][3] == 's'
                {
                    count += 1;
                }

                if window[3][3] == 'x'
                    && window[2][2] == 'm'
                    && window[1][1] == 'a'
                    && window[0][0] == 's'
                {
                    count += 1;
                }

                if window[0][3] == 'x'
                    && window[1][2] == 'm'
                    && window[2][1] == 'a'
                    && window[3][0] == 's'
                {
                    count += 1;
                }

                if window[3][0] == 'x'
                    && window[2][1] == 'm'
                    && window[1][2] == 'a'
                    && window[0][3] == 's'
                {
                    count += 1;
                }
            }

            // Verticals
            if window[0][0] == 'x'
                && window[1][0] == 'm'
                && window[2][0] == 'a'
                && window[3][0] == 's'
            {
                count += 1;
            }

            if window[3][0] == 'x'
                && window[2][0] == 'm'
                && window[1][0] == 'a'
                && window[0][0] == 's'
            {
                count += 1;
            }

            col += 1;
        }

        // last Verticals
        for i in 1..4 {
            if window[0][i] == 'x'
                && window[1][i] == 'm'
                && window[2][i] == 'a'
                && window[3][i] == 's'
            {
                count += 1;
            }

            if window[3][i] == 'x'
                && window[2][i] == 'm'
                && window[1][i] == 'a'
                && window[0][i] == 's'
            {
                count += 1;
            }
        }

        row += 1;
    }

    for line in input.lines() {
        for a in line.chars().collect::<Vec<_>>().windows(4) {
            if a[0] == 'X' && a[1] == 'M' && a[2] == 'A' && a[3] == 'S' {
                count += 1;
            }

            if a[3] == 'X' && a[2] == 'M' && a[1] == 'A' && a[0] == 'S' {
                count += 1;
            }
        }
    }

    println!("{}", count)
}
