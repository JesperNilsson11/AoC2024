fn part1() -> i64 {
    let input = include_str!("input3.txt").trim();
    let mut result = 0;

    let mut left: i64 = 0;
    let mut right: i64 = 0;
    let mut state = 0;

    for c in input.chars() {
        if c == 'm' {
            state = 1;
            left = 0;
            right = 0;
        } else if c == 'u' {
            if state == 1 {
                state = 2;
            } else {
                state = 0;
            }
        } else if c == 'l' {
            if state == 2 {
                state = 3;
            } else {
                state = 0;
            }
        } else if c == '(' {
            if state == 3 {
                state = 4;
            } else {
                state = 0;
            }
        } else if c.is_digit(10) {
            if state >= 4 && state <= 6 {
                left *= 10;
                left += c.to_digit(10).unwrap() as i64;
                state += 1;
            } else if state >= 8 && state <= 10 {
                right *= 10;
                right += c.to_digit(10).unwrap() as i64;
                state += 1;
            }
        } else if c == ',' {
            if state >= 5 && state <= 7 {
                state = 8;
            } else {
                state = 0;
            }
        } else if c == ')' {
            if state >= 9 && state <= 11 {
                result += left * right;
            }
            state = 0;
        } else {
            state = 0;
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input3.txt").trim();
    let mut result = 0;

    let mut left: i64 = 0;
    let mut right: i64 = 0;
    let mut state = 0;
    let mut enabled = true;

    for c in input.chars() {
        if c == 'm' {
            state = 1;
            left = 0;
            right = 0;
        } else if c == 'u' {
            if state == 1 {
                state = 2;
            } else {
                state = 0;
            }
        } else if c == 'l' {
            if state == 2 {
                state = 3;
            } else {
                state = 0;
            }
        } else if c == '(' {
            if state == 3 {
                state = 4;
            } else if state == 13 {
                state = 14;
            } else if state == 16 {
                state = 17;
            } else {
                state = 0;
            }
        } else if c.is_digit(10) {
            if state >= 4 && state <= 6 {
                left *= 10;
                left += c.to_digit(10).unwrap() as i64;
                state += 1;
            } else if state >= 8 && state <= 10 {
                right *= 10;
                right += c.to_digit(10).unwrap() as i64;
                state += 1;
            }
        } else if c == ',' {
            if state >= 5 && state <= 7 {
                state = 8;
            } else {
                state = 0;
            }
        } else if c == ')' {
            if state >= 9 && state <= 11 {
                if enabled {
                    result += left * right;
                }
            } else if state == 14 {
                enabled = true;
            } else if state == 17 {
                enabled = false;
            }
            state = 0;
        } else if c == 'd' {
            state = 12;
        } else if c == 'o' {
            if state == 12 {
                state = 13;
            } else {
                state = 0;
            }
        } else if c == 'n' {
            if state == 13 {
                state = 14;
            } else {
                state = 0;
            }
        } else if c == '\'' {
            if state == 14 {
                state = 15;
            } else {
                state = 0;
            }
        } else if c == 't' {
            if state == 15 {
                state = 16;
            } else {
                state = 0;
            }
        } else {
            state = 0;
        }
    }

    return result;
}

fn main() {
    println!("Day 3!");
    println!("{}", part1());
    println!("{}", part2());
}
