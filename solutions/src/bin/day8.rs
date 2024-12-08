use std::collections::HashSet;
use num_integer;

fn part1() -> i64 {
    let input = include_str!("input8.txt").trim();
    let result;
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.split('\n') {
        map.push(line.trim().chars().collect());
    }

    let height = map.len();
    let width = map[0].len();
    let mut set = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '.' {
                continue;
            }
            for y2 in y..height {
                let mut startx = 0;
                if y == y2 {
                    startx = x+1;
                }
                for x2 in startx..width {
                    if map[y][x] != map[y2][x2] {
                        continue;
                    }

                    let dx = x as i64 - x2 as i64;
                    let dy = y as i64 - y2 as i64;
                    let x3 = x as i64 + dx;
                    let y3 = y as i64 + dy;
                    if x3 >= 0 && x3 < width as i64 && y3 >= 0 && y3 < height as i64 {
                        set.insert((x3, y3));
                    }
                    let x4 = x2 as i64 - dx;
                    let y4 = y2 as i64 - dy;
                    if x4 >= 0 && x4 < width as i64 && y4 >= 0 && y4 < height as i64 {
                        set.insert((x4, y4));
                    }
                } 
            }
        }
    }
    result = set.len() as i64;

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input8.txt").trim();
    let result;
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.split('\n') {
        map.push(line.trim().chars().collect());
    }

    let height = map.len();
    let width = map[0].len();
    let mut set = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '.' {
                continue;
            }
            for y2 in y..height {
                let mut startx = 0;
                if y == y2 {
                    startx = x+1;
                }
                for x2 in startx..width {
                    if map[y][x] != map[y2][x2] {
                        continue;
                    }

                    let mut dx = x as i64 - x2 as i64;
                    let mut dy = y as i64 - y2 as i64;
                    let gcd = num_integer::gcd(dx, dy);
                    dx /= gcd;
                    dy /= gcd;

                    let mut xx = x as i64;
                    let mut yy = y as i64;
                    while xx >= 0 && xx < width as i64 && yy >= 0 && yy < height as i64 {
                        set.insert((xx,yy));
                        xx += dx;
                        yy += dy;
                    }
                    xx = x as i64;
                    yy = y as i64;
                    while xx >= 0 && xx < width as i64 && yy >= 0 && yy < height as i64 {
                        set.insert((xx,yy));
                        xx -= dx;
                        yy -= dy;
                    }
                } 
            }
        }
    }

    result = set.len() as i64;    

    return result;
}

fn main() {
    println!("Day 8!");
    println!("{}", part1());
    println!("{}", part2());
}