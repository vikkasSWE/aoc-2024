use std::time::Instant;

fn profile(day: &str, a: fn() -> i32, b: fn() -> i32) {
    println!("{day}");

    let p1t = Instant::now();
    let p1 = a();
    let p1_time = p1t.elapsed().as_micros();

    let p2t = Instant::now();
    let p2 = b();
    let p2_time = p2t.elapsed().as_micros();

    println!("Part 1 {}, time: {}us", p1, p1_time);
    println!("Part 2 {}, time: {}us", p2, p2_time);

    println!("----------------------------");
}

fn main() {
    println!("----------------------------");
    profile("Day 1", day1::a, day1::b);
    profile("Day 2", day2::a, day2::b);
    profile("Day 3", day3::a, day3::b);
    profile("Day 4", day4::a, day4::b);
    profile("Day 5", day5::a, day5::b);
    profile("Day 6", day6::a, day6::b);
}
