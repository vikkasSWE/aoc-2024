use std::{collections::HashMap, error::Error};

const INPUT: &str = include_str!("input.txt");

fn test_values(correct_value: u64, values: Vec<u64>) -> Result<u64, Box<dyn Error>> {
    let start = values[0];

    if recurse(correct_value, start, &values, 1) {
        Ok(correct_value)
    } else {
        Err("()".into())
    }
}

fn recurse(correct_value: u64, acc_in: u64, values: &[u64], depth: usize) -> bool {
    if depth == values.len() {
        return acc_in == correct_value;
    }

    if acc_in > correct_value {
        return false;
    }

    let acc_add = acc_in + values[depth];
    let acc_mul = acc_in * values[depth];

    recurse(correct_value, acc_add, values, depth + 1)
        || recurse(correct_value, acc_mul, values, depth + 1)
}

pub fn a() -> u64 {
    let mut map = HashMap::new();
    for line in INPUT.lines() {
        let mut split = line.split(":");

        let correct_value = split.next().unwrap().parse::<u64>().unwrap();

        let val = split.next().unwrap().trim_ascii_start().split(" ");

        let mut values = Vec::new();
        for value in val {
            values.push(value.parse::<u64>().unwrap());
        }

        map.insert(correct_value, values);
    }

    let mut count: u64 = 0;

    for (correct_value, values) in map {
        if let Ok(res) = test_values(correct_value, values) {
            count += res;
        }
    }

    count
}

fn test_values_2(correct_value: u64, values: Vec<u64>) -> Result<u64, Box<dyn Error>> {
    let start = values[0];

    if recurse_2(correct_value, start, &values, 1) {
        Ok(correct_value)
    } else {
        Err("()".into())
    }
}

fn recurse_2(correct_value: u64, acc_in: u64, values: &[u64], depth: usize) -> bool {
    if depth == values.len() {
        return acc_in == correct_value;
    }

    if acc_in > correct_value {
        return false;
    }

    let acc_add = acc_in + values[depth];
    let acc_mul = acc_in * values[depth];
    let acc_con = format!("{}{}", acc_in, values[depth])
        .parse::<u64>()
        .unwrap();

    recurse_2(correct_value, acc_add, values, depth + 1)
        || recurse_2(correct_value, acc_mul, values, depth + 1)
        || recurse_2(correct_value, acc_con, values, depth + 1)
}

pub fn b() -> u64 {
    let mut map = HashMap::new();
    for line in INPUT.lines() {
        let mut split = line.split(":");

        let correct_value = split.next().unwrap().parse::<u64>().unwrap();

        let val = split.next().unwrap().trim_ascii_start().split(" ");

        let mut values = Vec::new();
        for value in val {
            values.push(value.parse::<u64>().unwrap());
        }

        map.insert(correct_value, values);
    }

    let mut count: u64 = 0;

    for (correct_value, values) in map {
        if let Ok(res) = test_values_2(correct_value, values) {
            count += res;
        }
    }

    count
}
