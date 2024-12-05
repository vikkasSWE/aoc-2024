use std::time::Instant;

fn main() {
    profile(day5::a);
    profile(day5::b);
    // day2::a();
    // day3::a();
    // day4::a();
    // day5::a();

    // day1::b();
    // day2::b();
    // day3::b();
    // day4::b();
    // day5::b();
}

fn profile(a: fn() -> i32) {
    println!("----------------------------");
    println!("Day 1 Part a:");
    let start = Instant::now();
    let res = a();
    println!("Time {}", start.elapsed().as_micros());
    println!("Res {}", res);
    println!("----------------------------")
}
