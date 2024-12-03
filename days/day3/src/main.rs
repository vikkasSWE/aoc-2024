use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Mul,
    Do,
    Dont,
    None,
}

fn main() {
    let start = Instant::now();
    let mut p1 = 0;
    let mut p2 = 0;

    let mut count = 0;
    let mut a = 0;
    let mut b = 0;

    let mut enable = true;
    let mut token = Token::None;

    for c in INPUT.chars() {
        match token {
            Token::None => {
                #[allow(clippy::if_same_then_else)]
                if (c == 'm' || c == 'd') && count == 0 {
                    count += 1;
                } else if (c == 'u' || c == 'o') && count == 1 {
                    count += 1;
                }
                // TOKEN
                else if c == 'l' && count == 2 {
                    count += 1;
                    token = Token::Mul;
                }
                // DO
                else if c == '(' && count == 2 {
                    count += 1;
                    token = Token::Do;
                }
                // DONT
                else if c == 'n' && count == 2 {
                    count += 1;
                    token = Token::Dont;
                } else {
                    count = 0;
                }
            }
            Token::Mul => {
                if c == '(' && count == 3 {
                    count += 1;
                } else if c.is_numeric() && count == 4 {
                    a = c.to_digit(10).unwrap() as i32;
                    count += 1;
                } else if count == 5 && c.is_numeric() {
                    a = a * 10 + c.to_digit(10).unwrap() as i32
                } else if c == ',' && count == 5 {
                    count += 1;
                } else if c.is_numeric() && count == 6 {
                    b = c.to_digit(10).unwrap() as i32;
                    count += 1;
                } else if count == 7 && c.is_numeric() {
                    b = b * 10 + c.to_digit(10).unwrap() as i32
                } else if c == ')' && count == 7 {
                    if enable {
                        p2 += a * b;
                    }
                    p1 += a * b;

                    count = 0;
                    token = Token::None;
                } else {
                    count = 0;
                    token = Token::None;
                }
            }
            Token::Do => {
                if c == ')' && count == 3 {
                    enable = true;
                    count = 0;
                    token = Token::None;
                } else {
                    count = 0;
                    token = Token::None;
                }
            }
            Token::Dont =>
            {
                #[allow(clippy::if_same_then_else)]
                if c == '\'' && count == 3 {
                    count += 1;
                } else if c == 't' && count == 4 {
                    count += 1;
                } else if c == '(' && count == 5 {
                    count += 1;
                } else if c == ')' && count == 6 {
                    enable = false;

                    count = 0;
                    token = Token::None;
                } else {
                    count = 0;
                    token = Token::None;
                }
            }
        }
    }

    println!("{}", start.elapsed().as_micros());

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
