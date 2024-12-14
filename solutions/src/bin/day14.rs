use std::collections::HashSet;

use regex::Regex;

fn part1() -> i64 {
    let input = include_str!("input14.txt").trim();
    let result;
    let mut robots: Vec<(i64, i64, i64, i64)> = Vec::new();
    let h = 103;
    let w = 101;

    for line in input.split('\n') {
        let r = Regex::new(r".*=(-?[0-9]+).*,(-?[0-9]+).*=(-?[0-9]+).*,(-?[0-9]+)").unwrap();
        let Some(caps) = r.captures(line) else {
            println!("Parse failed");
            return -1;
        };

        let (x, y, dx, dy) = (caps[1].parse::<i64>().unwrap(),caps[2].parse::<i64>().unwrap(),caps[3].parse::<i64>().unwrap(),caps[4].parse::<i64>().unwrap());
        robots.push((x, y, dx, dy));
    }

    for _ in 0..100 {
        for r in &mut robots {
            r.0 += r.2;
            if r.0 < 0 {
                r.0 += w;
            }
            if r.0 >= w {
                r.0 -= w;
            }
            r.1 += r.3;
            if r.1 < 0 {
                r.1 += h;
            }
            if r.1 >= h {
                r.1 -= h;
            }
        }
    }

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;
    let mx = w / 2;
    let my = h / 2;
    for r in robots {
        if r.0 < mx {
            if r.1 < my {
                tl += 1;
            } else if r.1 > my {
                bl += 1;
            }
        } else if r.0 > mx {
            if r.1 < my {
                tr += 1;
            } else if r.1 > my {
                br += 1;
            }
        }
    }

    result = tl * tr * bl * br;
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input14.txt").trim();
    let result = 0;
    let mut robots: Vec<(i64, i64, i64, i64, usize)> = Vec::new();
    let h = 103;
    let w = 101;

    for (i, line) in input.split('\n').enumerate() {
        let r = Regex::new(r".*=(-?[0-9]+).*,(-?[0-9]+).*=(-?[0-9]+).*,(-?[0-9]+)").unwrap();
        let Some(caps) = r.captures(line) else {
            println!("Parse failed");
            return -1;
        };

        let (x, y, dx, dy) = (caps[1].parse::<i64>().unwrap(),caps[2].parse::<i64>().unwrap(),caps[3].parse::<i64>().unwrap(),caps[4].parse::<i64>().unwrap());
        robots.push((x, y, dx, dy, i));
    }

    for i in 1..1000000 {
        let mut found = false;
        let mut set = HashSet::new();
        for r in &mut robots {
            r.0 += r.2;
            if r.0 < 0 {
                r.0 += w;
            }
            if r.0 >= w {
                r.0 -= w;
            }
            r.1 += r.3;
            if r.1 < 0 {
                r.1 += h;
            }
            if r.1 >= h {
                r.1 -= h;
            }

            set.insert((r.0, r.1));
        }

        for (xx, yy) in &set {
            let mut counter = 0;
            let mut x = *xx;
            let mut y = *yy;
            while set.contains(&(x,y)) {
                counter += 1;
                x += 1;
                y += 1;
            }

            if counter > 10 {
                found = true;
                break;
            }
        }

        if found {
            for y in 0..h {
                for x in 0..w {
                    let mut count = 0;
                    for r in &robots {
                        if r.0 == x && r.1 == y {
                            count += 1;
                        }
                    }
                    if count == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
                println!(" ");
            }
            return i;
        }
    }

    return result;
}

fn main() {
    println!("Day 14!");
    println!("{}", part1());
    println!("{}", part2());
}