use std::collections::HashSet;

fn part1() -> i64 {
    let input = include_str!("input12.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<char>> = input.split('\n').map(|x| x.chars().collect()).collect();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let h = map.len() as i64;
    let w = map[0].len() as i64;

    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut positions: HashSet<(i64, i64)> = HashSet::new();
            let mut area = 0;
            let mut perimiter = 0;
            positions.insert((x, y));
            let mut newpos = true;
            while newpos {
                newpos = false;
                let mut nposis = Vec::new();
                for (px, py) in &positions {
                    if visited.contains(&(*px, *py)) {
                        continue;
                    }
                    newpos = true;
                    visited.insert((*px, *py));
                    area += 1;

                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nx = *px + dx;
                        let ny = *py + dy;
                        if nx < 0 || nx >= w || ny < 0 || ny >= h || map[ny as usize][nx as usize] != map[*py as usize][*px as usize] {
                            perimiter += 1;
                        } else {
                            nposis.push((nx, ny));
                        }
                    }
                }

                for (px, py) in nposis {
                    positions.insert((px, py));
                }
            }

            result += perimiter * area;
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input12.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<char>> = input.split('\n').map(|x| x.chars().collect()).collect();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let h = map.len() as i64;
    let w = map[0].len() as i64;

    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut positions: HashSet<(i64, i64)> = HashSet::new();
            let mut area = 0;
            let mut perimiter = HashSet::new();
            positions.insert((x, y));
            let mut newpos = true;
            while newpos {
                newpos = false;
                let mut nposis = Vec::new();
                for (px, py) in &positions {
                    if visited.contains(&(*px, *py)) {
                        continue;
                    }
                    newpos = true;
                    visited.insert((*px, *py));
                    area += 1;

                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nx = *px + dx;
                        let ny = *py + dy;
                        if nx < 0 || nx >= w || ny < 0 || ny >= h || map[ny as usize][nx as usize] != map[*py as usize][*px as usize] {
                            perimiter.insert((nx, ny, dx, dy));
                        } else {
                            nposis.push((nx, ny));
                        }
                    }
                }

                for (px, py) in nposis {
                    positions.insert((px, py));
                }
            }

            let mut sides = 0;
            let mut sides_visited: HashSet<(i64, i64, i64, i64)> = HashSet::new();
            for (px, py, dx, dy) in &perimiter {
                if sides_visited.contains(&(*px, *py, *dx, *dy)) {
                    continue;
                }
                sides += 1;
                sides_visited.insert((*px, *py, *dx, *dy));

                for (ddx, ddy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let mut nx = px + ddx;
                    let mut ny = py + ddy;

                    while perimiter.contains(&(nx, ny, *dx, *dy)) {
                        sides_visited.insert((nx, ny, *dx, *dy));
                        nx += ddx;
                        ny += ddy;
                    }
                }
            }
            result += sides * area;
        }
    }

    return result;
}

fn main() {
    println!("Day 12!");
    println!("{}", part1());
    println!("{}", part2());
}