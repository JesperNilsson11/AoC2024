use std::collections::HashSet;


fn part1() -> i64 {
    let input = include_str!("input6.txt").trim();
    let mut result = 0;
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.split('\n') {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let mut dx = 0;
    let mut dy = -1;
    let mut x = 0;
    let mut y = 0;

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for yy in 0..height {
        for xx in 0..width {
            if map[yy as usize][xx as usize] == '^' {
                x = xx as i32;
                y = yy as i32;
                map[y as usize][x as usize] = 'X';
                break;
            }
        }
    }

    while x >= 0 && x < width && y >= 0 && y < height {
        let newx = x + dx;
        let newy = y + dy;

        if newx < 0 || newx >= width || newy < 0 || newy >= height {
            break;
        }

        if map[newy as usize][newx as usize] == '#' {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
            continue;
        }

        x = newx;
        y = newy;
        map[y as usize][x as usize] = 'X';
    }

    for yy in 0..height {
        for xx in 0..width {
            if map[yy as usize][xx as usize] == 'X' {
                result += 1;
            }
        }
    }

    
    return result;
}

fn is_loop(map: &Vec<Vec<char>>, mut x: i32, mut y: i32) -> bool {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut dy = -1;
    let mut dx = 0;
    let mut set: HashSet<i32>  = HashSet::new();

    let item = x | y << 8 | dx << 16 | dy << 24;
    set.insert(item);

    while x >= 0 && x < width && y >= 0 && y < height {
        let newx = x + dx;
        let newy = y + dy;

        if newx < 0 || newx >= width || newy < 0 || newy >= height {
            return false;
        }

        if map[newy as usize][newx as usize] == '#' {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
            continue;
        }

        x = newx;
        y = newy;

        let item = x | y << 8 | dx << 16 | dy << 24;
        if set.contains(&item) {
            return true;
        }
        set.insert(item);
    }

    return false;
}

fn part2() -> i64 {
    let input = include_str!("input6.txt").trim();
    let mut result = 0;
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.split('\n') {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let mut x = 0;
    let mut y = 0;

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for yy in 0..height {
        for xx in 0..width {
            if map[yy as usize][xx as usize] == '^' {
                x = xx as i32;
                y = yy as i32;
                map[y as usize][x as usize] = 'X';
                break;
            }
        }
    }

    for yy in 0..height {
        for xx in 0..width {
            if xx == x && yy == y {
                continue;
            }
            if map[yy as usize][xx as usize] == '#' {
                continue;
            }

            map[yy as usize][xx as usize] = '#';
            if is_loop(&map, x, y) {
                result += 1;
            }
            map[yy as usize][xx as usize] = '.';
        }
    }
    

    return result;
}

fn main() {
    println!("Day 6!");
    println!("{}", part1());
    println!("{}", part2());
}