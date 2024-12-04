fn solve(map: &Vec<Vec<char>>, dx: i64, dy: i64) -> i64 {
    let mut result = 0;

    if dy != 0 {
        for xx in 0..map[0].len() {
            let mut y: i64 = 0;
            let mut x: i64 = xx as i64;
            let mut xidx = 0;
            let mut sidx = 0;

            while y < (map.len() as i64) && x < map[0].len() as i64 && x >= 0 && y >= 0 {
                let c = map[y as usize][x as usize];
                if c == 'X' {
                    xidx = 1;
                    if sidx == 3 {
                        result += 1;
                        //println!("samx {} {}", x, y);
                    }
                    sidx = 0;
                } else if c == 'M' {
                    if xidx == 1 {
                        xidx += 1;
                    } else {
                        xidx = 0;
                    }
                    if sidx == 2 {
                        sidx += 1;
                    } else {
                        sidx = 0;
                    }
                } else if c == 'A' {
                    if xidx == 2 {
                        xidx += 1;
                    } else {
                        xidx = 0;
                    }
                    if sidx == 1 {
                        sidx += 1;
                    } else {
                        sidx = 0;
                    }
                } else if c == 'S' {
                    if xidx == 3 {
                        result += 1;
                        //println!("xmas {} {}", x, y);
                    }
                    xidx = 0;
                    sidx = 1;
                }

                x += dx;
                y += dy;
            }
        }
    }

    if dx != 0 {
        let mut start = 0;
        if dy != 0 {
            start = 1;
        }
        for yy in start..map.len() {
            let mut y: i64 = yy as i64;
            let mut x: i64 = 0;
            if dx < 0 {
                x = map[0].len() as i64 - 1;
            }
            let mut xidx = 0;
            let mut sidx = 0;

            while y < (map.len() as i64) && x < map[0].len() as i64 && x >= 0 && y >= 0 {
                let c = map[y as usize][x as usize];
                if c == 'X' {
                    xidx = 1;
                    if sidx == 3 {
                        result += 1;
                        //println!("samx {} {}", x, y);
                    }
                    sidx = 0;
                } else if c == 'M' {
                    if xidx == 1 {
                        xidx += 1;
                    } else {
                        xidx = 0;
                    }
                    if sidx == 2 {
                        sidx += 1;
                    } else {
                        sidx = 0;
                    }
                } else if c == 'A' {
                    if xidx == 2 {
                        xidx += 1;
                    } else {
                        xidx = 0;
                    }
                    if sidx == 1 {
                        sidx += 1;
                    } else {
                        sidx = 0;
                    }
                } else if c == 'S' {
                    if xidx == 3 {
                        result += 1;
                        //println!("xmas {} {}", x, y);
                    }
                    xidx = 0;
                    sidx = 1;
                }

                x += dx;
                y += dy;
            }
        }
    }

    return result;
}

fn part1() -> i64 {
    let input = include_str!("input4.txt").trim();
    let mut result = 0;

    let mut map: Vec<Vec<char>> = vec![];
    for l in input.split('\n') {
        let mut cs: Vec<char> = vec![];
        for c in l.chars() {
            cs.push(c);
        }
        map.push(cs);
    }

    result += solve(&map, 1, 0);
    result += solve(&map, 0, 1);
    result += solve(&map, 1, 1);
    result += solve(&map, -1, 1);

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input4.txt").trim();
    let mut result = 0;

    let mut map: Vec<Vec<char>> = vec![];
    for l in input.split('\n') {
        let mut cs: Vec<char> = vec![];
        for c in l.chars() {
            cs.push(c);
        }
        map.push(cs);
    }

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] != 'A' {
                continue;
            }

            if map[y - 1][x - 1] == 'M' {
                if map[y + 1][x + 1] != 'S' {
                    continue;
                }

                if map[y + 1][x - 1] == 'M' {
                    if map[y - 1][x + 1] == 'S' {
                        result += 1;
                    } else {
                        continue;
                    }
                }
                if map[y + 1][x - 1] == 'S' {
                    if map[y - 1][x + 1] == 'M' {
                        result += 1;
                    } else {
                        continue;
                    }
                }
            }

            if map[y - 1][x - 1] == 'S' {
                if map[y + 1][x + 1] != 'M' {
                    continue;
                }

                if map[y + 1][x - 1] == 'M' {
                    if map[y - 1][x + 1] == 'S' {
                        result += 1;
                    } else {
                        continue;
                    }
                }
                if map[y + 1][x - 1] == 'S' {
                    if map[y - 1][x + 1] == 'M' {
                        result += 1;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    return result;
}

fn main() {
    println!("Day 4!");
    println!("{}", part1());
    println!("{}", part2());
}
