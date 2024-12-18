use std::collections::HashSet;

fn part1() -> i64 {
    let input = include_str!("input18.txt").trim();
    let mut result = 0;
    let mut bytes: Vec<Vec<i64>> = input
        .split('\n')
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect())
        .collect();
    let mut map = HashSet::new();

    let w = 70;
    let h = 70;
    for i in 0..1024 {
        map.insert((bytes[i][0], bytes[i][1]));
    }

    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((0, 0));
    visited.insert((0, 0));
    while queue.len() > 0 {
        let mut newqueue = Vec::new();
        let mut found = false;
        for (x, y) in queue {
            if x == w && y == h {
                found = true;
                break;
            }

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx > w || ny < 0 || ny > h {
                    continue;
                }
                if visited.contains(&(nx, ny)) {
                    continue;
                }
                if map.contains(&(nx, ny)) {
                    continue;
                }

                newqueue.push((nx, ny));
                visited.insert((nx, ny));
            }
        }

        if found {
            break;
        }
        queue = newqueue;
        result += 1;
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input18.txt").trim();
    let mut result = 0;
    let mut bytes: Vec<Vec<i64>> = input
        .split('\n')
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect())
        .collect();

    let mut min = 0;
    let mut max = bytes.len() - 1;

    while min < max {
        let mut map = HashSet::new();
        let w = 70;
        let h = 70;

        let num_bytes = (max+min + 1) / 2;
        for i in 0..num_bytes {
            map.insert((bytes[i][0], bytes[i][1]));
        }
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push((0, 0));
        visited.insert((0, 0));
        let mut found = false;
        while queue.len() > 0 {
            let mut newqueue = Vec::new();
            found = false;
            for (x, y) in queue {
                if x == w && y == h {
                    found = true;
                    break;
                }

                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx > w || ny < 0 || ny > h {
                        continue;
                    }
                    if visited.contains(&(nx, ny)) {
                        continue;
                    }
                    if map.contains(&(nx, ny)) {
                        continue;
                    }

                    newqueue.push((nx, ny));
                    visited.insert((nx, ny));
                }
            }

            if found {
                break;
            }
            queue = newqueue;
            result += 1;
        }

        if found {
            if min == num_bytes {
                min += 1;
            } else {
                min = num_bytes;
            }
            result = min;
        } else {
            if max == num_bytes {
                max -= 1;
            } else {
                max = num_bytes;
            }
        }
    }

    result = min;
    println!("{},{}", bytes[min][0], bytes[min][1]);

    return result as i64;
}

fn main() {
    println!("Day 18!");
    println!("{}", part1());
    println!("{}", part2());
}
