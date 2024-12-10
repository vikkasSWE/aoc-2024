use std::{fmt::Display, time::Instant};

fn profile<T: Display, V: Display>(day: &str, a: fn() -> T, b: fn() -> V) -> u128 {
    println!("{day}");

    let p1t = Instant::now();
    let p1 = a();
    let p1_time = p1t.elapsed().as_micros();

    let p2t = Instant::now();
    let p2 = b();
    let p2_time = p2t.elapsed().as_micros();
    let res = p1t.elapsed().as_micros();

    println!(
        "Part 1 {}, time: {}ms, {}us",
        p1,
        p1_time / 1000,
        p1_time % 1000
    );
    println!(
        "Part 2 {}, time: {}ms, {}us",
        p2,
        p2_time / 1000,
        p2_time % 1000
    );

    println!("----------------------------");

    res
}

fn main() {
    println!("----------------------------");
    let mut res = 0;
    // res += profile("Day 1", day1::a, day1::b);
    // res += profile("Day 2", day2::a, day2::b);
    // res += profile("Day 3", day3::a, day3::b);
    // res += profile("Day 4", day4::a, day4::b);
    // res += profile("Day 5", day5::a, day5::b);
    res += profile("Day 6", day6::a, day6::b);
    // res += profile("Day 7", day7::a, day7::b);
    // res += profile("Day 8", day8::a, day8::b);
    // res += profile("Day 9", day9::a, day9::b);
    // res += profile("Day 10", day10::a, day10::b);

    println!(
        "Total ALL Days(A+B): {}s, {}ms, {}us",
        res / 1_000_000,
        res / 1_000,
        res % 1000
    );
    println!("----------------------------");
}
