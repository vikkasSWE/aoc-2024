const INPUT: &str = include_str!("input.txt");

fn recurse_a(correct_value: u64, acc_in: u64, values: &[u64], depth: usize) -> bool {
    if depth == values.len() {
        return acc_in == correct_value;
    }

    if acc_in > correct_value {
        return false;
    }

    ({
        let acc_mul = acc_in * values[depth];
        recurse_a(correct_value, acc_mul, values, depth + 1)
    } || {
        let acc_add = acc_in + values[depth];
        recurse_a(correct_value, acc_add, values, depth + 1)
    })
}

pub fn a() -> u64 {
    let mut map = Vec::new();
    for line in INPUT.lines() {
        let mut split = line.split(":");

        let correct_value = split.next().unwrap().parse::<u64>().unwrap();

        let values = split
            .next()
            .unwrap()
            .trim_start()
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        map.push((correct_value, values));
    }

    let mut count: u64 = 0;

    for (correct_value, values) in map {
        let start = values[0];

        if recurse_a(correct_value, start, &values, 1) {
            count += correct_value;
        }
    }

    count
}

fn count_digits(mut x: u64) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut digits = 0;
    while x > 0 {
        digits += 1;
        x /= 10;
    }
    digits
}

fn recurse_b(correct_value: u64, acc_in: u64, values: &[u64], depth: usize) -> bool {
    if depth == values.len() {
        return acc_in == correct_value;
    }

    if acc_in > correct_value {
        return false;
    }

    ({
        let acc_mul = acc_in * values[depth];
        recurse_b(correct_value, acc_mul, values, depth + 1)
    } || {
        let acc_add = acc_in + values[depth];
        recurse_b(correct_value, acc_add, values, depth + 1)
    } || {
        let digits = count_digits(values[depth]);
        let pow10 = 10u64.pow(digits);
        let acc_con = acc_in * pow10 + values[depth];
        recurse_b(correct_value, acc_con, values, depth + 1)
    })
}

pub fn b() -> u64 {
    let mut map = Vec::new();
    for line in INPUT.lines() {
        let mut split = line.split(":");

        let correct_value = split.next().unwrap().parse::<u64>().unwrap();

        let values = split
            .next()
            .unwrap()
            .trim_start()
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        map.push((correct_value, values));
    }

    let mut count: u64 = 0;

    for (correct_value, values) in map {
        let start = values[0];

        if recurse_b(correct_value, start, &values, 1) {
            count += correct_value;
        }
    }

    count
}
