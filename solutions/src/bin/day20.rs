use std::collections::{HashMap, HashSet};


fn part1() -> i64 {
    let input = include_str!("input20.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<char>> = input.split('\n').map(|x| x.trim().chars().collect()).collect();
    let mut start: (i64, i64) = (0,0);
    let mut cost = Vec::new();

    let h = map.len() as i64;
    let w = map[0].len() as i64;
    for y in 0..h {
        let mut row = Vec::new();
        for x in 0..w {
            row.push(i64::MAX);
            if map[y as usize][x as usize] == 'S' {
                start = (x as i64, y as i64);
            }
        }
        cost.push(row);
    }

    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((start.0, start.1));
    visited.insert((start.0, start.1));

    let mut found = false;
    while queue.len() > 0 && !found {
        let mut newqueue = Vec::new();
        for (x, y) in queue {
            if map[y as usize][x as usize] == '#' {
                continue;
            }
            cost[y as usize][x as usize] = result;
            for (dx, dy) in [(1,0), (-1,0),(0,1),(0,-1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }
                if map[ny as usize][nx as usize] == 'E' {
                    found = true;
                    cost[ny as usize][nx as usize] = result + 1;
                    break;
                }
                if map[ny as usize][nx as usize] == '#' {
                    continue;
                }
                if !visited.contains(&(nx,ny)) {
                    visited.insert((nx,ny));
                    newqueue.push((nx,ny));
                }
            }
        }
        
        result += 1;
        queue = newqueue;
    }

    let mut hmap = HashMap::new();
    for y in 1..h-1 {
        for x in 1..w-1 {
            if map[y as usize][x as usize] == '#' {
                let mut cs = Vec::new();
                for (dx, dy) in [(1,0), (-1,0),(0,1),(0,-1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    let c = cost[ny as usize][nx as usize];
                    if c != i64::MAX {
                        cs.push(c);
                    }
                }

                for i in 0..cs.len() {
                    for j in i+1..cs.len() {
                        let c1 = cs[i];
                        let c2 = cs[j];

                        if c1 < c2 {
                            *hmap.entry(c2-c1-2).or_insert(0) += 1;
                        } else {
                            *hmap.entry(c1-c2-2).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    
    result = 0;
    for (k,v) in hmap {
        if k >= 100 {
            result += v;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input20.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<char>> = input.split('\n').map(|x| x.trim().chars().collect()).collect();
    let mut start: (i64, i64) = (0,0);
    let mut cost = Vec::new();

    let h = map.len() as i64;
    let w = map[0].len() as i64;
    for y in 0..h {
        let mut row = Vec::new();
        for x in 0..w {
            row.push(i64::MAX);
            if map[y as usize][x as usize] == 'S' {
                start = (x as i64, y as i64);
            }
        }
        cost.push(row);
    }

    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((start.0, start.1));
    visited.insert((start.0, start.1));

    let mut found = false;
    while queue.len() > 0 && !found {
        let mut newqueue = Vec::new();
        for (x, y) in queue {
            if map[y as usize][x as usize] == '#' {
                continue;
            }
            cost[y as usize][x as usize] = result;
            for (dx, dy) in [(1,0), (-1,0),(0,1),(0,-1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }
                if map[ny as usize][nx as usize] == 'E' {
                    found = true;
                    cost[ny as usize][nx as usize] = result + 1;
                    break;
                }
                if map[ny as usize][nx as usize] == '#' {
                    continue;
                }
                if !visited.contains(&(nx,ny)) {
                    visited.insert((nx,ny));
                    newqueue.push((nx,ny));
                }
            }
        }
        
        result += 1;
        queue = newqueue;
    }

    let mut hmap = HashMap::new();
    let max_dist = 20;
    for y in 1..h-1 {
        for x in 1..w-1 {
                let c = cost[y as usize][x as usize];
                if c == i64::MAX {
                    continue;
                }

                for dy in -(max_dist)..max_dist+1 as i64 {
                    for dx in -(max_dist)..max_dist+1 as i64 {
                        let dist = dx.abs() + dy.abs();
                        if dist > max_dist {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;

                        if nx < 0 || nx >= w || ny < 0 || ny >= h {
                            continue;
                        }
                        let nc = cost[ny as usize][nx as usize];
                        if nc == i64::MAX {
                            continue;
                        }
                        if c + dist - 1 < nc {
                            *hmap.entry(nc-(c+dist)).or_insert(0) += 1;
                        }
                    }
                }
        }
    }
    
    result = 0;
    for (k,v) in hmap {
        if k >= 100 {
            result += v;
        }
    }

    return result;
}

fn main() {
    println!("Day 20!");
    println!("{}", part1());
    println!("{}", part2());
}
