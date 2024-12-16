use std::collections::{HashMap, HashSet};


fn part1() -> i64 {
    let input = include_str!("input16.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<char>> = input.split('\n').map(|x| x.chars().collect()).collect();
    let mut pos: (i64,i64) = (0,0);
    let mut hmap = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                pos = (x as i64, y as i64);
            }
        }
    }

    hmap.insert((pos.0, pos.1, 1, 0), 1);
    hmap.insert((pos.0, pos.1, 0, 1), 1001);
    hmap.insert((pos.0, pos.1, 0, -1), 1001);
    loop {
        let mut found = false;
        result += 1;
        let mut queue = Vec::new();
        for (k, v) in &mut hmap {
            if *v > 0 {
                *v -= 1;
                if *v == 0 {
                    queue.push(*k);
                }
            }
        }

        for (x, y, dx, dy) in queue {
            let newx = x + dx;
            let newy = y + dy;

            if map[newy as usize][newx as usize] == '#' {
                continue;
            }
            if map[newy as usize][newx as usize] == 'E' {
                found = true;
                break;
            }

            let e = hmap.entry((newx, newy, dx, dy)).or_insert(1);
            if *e > 0 {
                *e = 1;
            }

            let ddx = dy;
            let ddy = -dx;
            hmap.entry((newx, newy, ddx, ddy)).or_insert(1001);
            let ddx = -dy;
            let ddy = dx;
            hmap.entry((newx, newy, ddx, ddy)).or_insert(1001);
        }

        if found || result > 10000000 {
            break;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input16.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<char>> = input.split('\n').map(|x| x.chars().collect()).collect();
    let mut pos: (i64,i64) = (0,0);
    let mut hmap = HashMap::new();
    let mut costs = HashMap::new();
    let mut end = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                pos = (x as i64, y as i64);
            }
        }
    }

    hmap.insert((pos.0, pos.1, 1, 0), 1);
    hmap.insert((pos.0, pos.1, 0, 1), 1001);
    hmap.insert((pos.0, pos.1, 0, -1), 1001);
    loop {
        let mut found = false;
        result += 1;
        let mut queue = Vec::new();
        for (k, v) in &mut hmap {
            if *v > 0 {
                *v -= 1;
                if *v == 0 {
                    queue.push(*k);
                }
            }
        }

        for (x, y, dx, dy) in queue {
            let newx = x + dx;
            let newy = y + dy;

            if map[newy as usize][newx as usize] == '#' {
                continue;
            }
            if map[newy as usize][newx as usize] == 'E' {
                found = true;
                end = (newx, newy);
            }
            costs.entry((newx, newy, dx, dy)).or_insert(result);
            costs.entry((x, y, dx, dy)).or_insert(result - 1);

            let e = hmap.entry((newx, newy, dx, dy)).or_insert(1);
            if *e > 0 {
                *e = 1;
            }

            let ddx = dy;
            let ddy = -dx;
            hmap.entry((newx, newy, ddx, ddy)).or_insert(1001);
            let ddx = -dy;
            let ddy = dx;
            hmap.entry((newx, newy, ddx, ddy)).or_insert(1001);
        }

        if found || result > 10000000 {
            break;
        }
    }

    let mut set = HashSet::new();
    set.insert(end);
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((end.0, end.1, 1, 0, result));
    queue.push((end.0, end.1, -1, 0, result));
    queue.push((end.0, end.1, 0, 1, result));
    queue.push((end.0, end.1, 0, -1, result));

    visited.insert((end.0, end.1, 1, 0));
    visited.insert((end.0, end.1, -1, 0));
    visited.insert((end.0, end.1, 0, 1));
    visited.insert((end.0, end.1, 0, -1));

    while queue.len() > 0 {
        let mut newqueue = Vec::new();
        for idx in 0..queue.len() {
            let p = queue[idx];
            let nx = p.0 - p.2;
            let ny = p.1 - p.3;

            if nx == 13 && ny == 13 {
                print!("");
            }

            if costs.contains_key(&(nx, ny, p.2, p.3)) {
                let c = costs.get(&(nx, ny, p.2, p.3)).unwrap();
                if c + 1 == p.4 {
                    if visited.contains(&(nx, ny, p.2, p.3)) == false {
                        set.insert((nx, ny));
                        newqueue.push((nx, ny, p.2, p.3, *c));
                        newqueue.push((nx, ny, -p.3, p.2, *c-1000));
                        newqueue.push((nx, ny, p.3, -p.2, *c-1000));
                    }
                }
            }
        }
        queue = newqueue;

    }

    return set.len() as i64;
}

fn main() {
    println!("Day 16!");
    println!("{}", part1());
    println!("{}", part2());
}