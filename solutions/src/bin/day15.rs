use std::usize;

fn part1() -> i64 {
    let input = include_str!("input15.txt").trim();
    let mut result = 0;

    //let mut tmp = input.split("\r\n\r\n");
    let mut tmp = input.split("\n\n");
    let mut map: Vec<Vec<char>> = tmp
        .next()
        .unwrap()
        .split('\n')
        .map(|x| x.trim().chars().collect())
        .collect();
    let moves = tmp.next().unwrap();
    let mut pos = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                pos = (x, y);
                map[y][x] = '.';
            }
        }
    }

    let w = map[0].len();
    let h = map.len();

    for l in moves.split('\n') {
        for m in l.trim().chars() {
            match m {
                '<' => {
                    if pos.0 == 0 {
                        continue;
                    }
                    let mut newpos = pos;
                    let mut valid_move = false;
                    let package = map[pos.1][pos.0 - 1] == 'O';
                    while newpos.0 > 0 {
                        newpos.0 -= 1;
                        if map[newpos.1][newpos.0] == '#' {
                            break;
                        }
                        if map[newpos.1][newpos.0] == '.' {
                            valid_move = true;
                            break;
                        }
                    }
                    if valid_move {
                        pos.0 -= 1;
                        if package {
                            map[pos.1][pos.0] = '.';
                            map[newpos.1][newpos.0] = 'O';
                        }
                    }
                }
                '>' => {
                    if pos.0 == w - 1 {
                        continue;
                    }
                    let mut newpos = pos;
                    let mut valid_move = false;
                    let package = map[pos.1][pos.0 + 1] == 'O';
                    while newpos.0 < w {
                        newpos.0 += 1;
                        if map[newpos.1][newpos.0] == '#' {
                            break;
                        }
                        if map[newpos.1][newpos.0] == '.' {
                            valid_move = true;
                            break;
                        }
                    }
                    if valid_move {
                        pos.0 += 1;
                        if package {
                            map[pos.1][pos.0] = '.';
                            map[newpos.1][newpos.0] = 'O';
                        }
                    }
                }
                '^' => {
                    if pos.1 == 0 {
                        continue;
                    }
                    let mut newpos = pos;
                    let mut valid_move = false;
                    let package = map[pos.1-1][pos.0] == 'O';
                    while newpos.1 > 0 {
                        newpos.1 -= 1;
                        if map[newpos.1][newpos.0] == '#' {
                            break;
                        }
                        if map[newpos.1][newpos.0] == '.' {
                            valid_move = true;
                            break;
                        }
                    }
                    if valid_move {
                        pos.1 -= 1;
                        if package {
                            map[pos.1][pos.0] = '.';
                            map[newpos.1][newpos.0] = 'O';
                        }
                    }
                },
                'v' => {
                    if pos.1 == h-1 {
                        continue;
                    }
                    let mut newpos = pos;
                    let mut valid_move = false;
                    let package = map[pos.1 + 1][pos.0] == 'O';
                    while newpos.1 < h {
                        newpos.1 += 1;
                        if map[newpos.1][newpos.0] == '#' {
                            break;
                        }
                        if map[newpos.1][newpos.0] == '.' {
                            valid_move = true;
                            break;
                        }
                    }
                    if valid_move {
                        pos.1 += 1;
                        if package {
                            map[pos.1][pos.0] = '.';
                            map[newpos.1][newpos.0] = 'O';
                        }
                    }
                },
                _ => println!("Error: {}", m),
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if map[y][x] == 'O' {
                result += 100 * (y as i64) + x as i64;
            }
        }
    }

    return result;
}

fn move_box(b: (usize, usize), dir: char, map: &Vec<Vec<char>>) -> Vec<(usize,usize)> {
    match dir {
        '<' => {
            if map[b.1][b.0 - 1] == '.' {
                return vec![(b.0 - 1, b.1)];
            }
            if map[b.1][b.0 - 1] == ']' {
                let mut boxes = move_box((b.0-2,b.1), dir, map);
                if boxes.len() > 0 {
                    boxes.push((b.0 - 1, b.1));
                    return boxes;
                }
            }
            if map[b.1][b.0 - 1] == '[' {
                println!("Error");
            }
            return vec![];
        }
        '>' => {
            if map[b.1][b.0 + 2] == '.' {
                return vec![(b.0 + 1, b.1)];
            }
            if map[b.1][b.0 + 2] == '[' {
                let mut boxes = move_box((b.0+2,b.1), dir, map);
                if boxes.len() > 0 {
                    boxes.push((b.0 + 1, b.1));
                    return boxes;
                }
            }
            return vec![];
        }
        '^' => {
            if map[b.1 - 1][b.0] == '.' &&  map[b.1 - 1][b.0 + 1] == '.' {
                return vec![(b.0, b.1 - 1)];
            }
            if map[b.1 - 1][b.0] == '[' {
                let mut boxes = move_box((b.0, b.1 - 1), dir, map);
                if boxes.len() > 0 {
                    boxes.push((b.0, b.1 - 1));
                    return boxes;
                }
            }
            let mut boxes = vec![];
            if map[b.1 - 1][b.0] == ']' && map[b.1 - 1][b.0 + 1] != '#' {
                let mut tmp = move_box((b.0 - 1, b.1 - 1), dir, map);
                if tmp.len() == 0 {
                    return vec![];
                }
                boxes.append(&mut tmp);
            }
            if map[b.1 - 1][b.0 + 1] == '[' && map[b.1 - 1][b.0] != '#' {
                let mut tmp = move_box((b.0 + 1, b.1 - 1), dir, map);
                if tmp.len() == 0 {
                    return vec![];
                }
                boxes.append(&mut tmp);
            }

            if boxes.len() > 0 {
                boxes.push((b.0, b.1 - 1));
            }

            return boxes;
        },
        'v' => {
            if map[b.1 + 1][b.0] == '.' && map[b.1 + 1][b.0 + 1] == '.' {
                return vec![(b.0, b.1 + 1)];
            }
            if map[b.1 + 1][b.0] == '[' {
                let mut boxes = move_box((b.0, b.1 + 1), dir, map);
                if boxes.len() > 0 {
                    boxes.push((b.0, b.1 + 1));
                    return boxes;
                }
            }
            let mut boxes = vec![];
            if map[b.1 + 1][b.0] == ']' && map[b.1 + 1][b.0 + 1] != '#' {
                let mut tmp = move_box((b.0 - 1, b.1 + 1), dir, map);
                if tmp.len() == 0 {
                    return vec![];
                }
                boxes.append(&mut tmp);
            }
            if map[b.1 + 1][b.0 + 1] == '[' && map[b.1 + 1][b.0] != '#' {
                let mut tmp = move_box((b.0 + 1, b.1 + 1), dir, map);
                if tmp.len() == 0 {
                    return vec![];
                }
                boxes.append(&mut tmp);
            }

            if boxes.len() > 0 {
                boxes.push((b.0, b.1 + 1));
            }

            return boxes;
        },
        _ => println!("Error: {}", dir),
    }

    return vec![];
}

fn clear_boxes(boxes: Vec<(usize,usize)>, dir: char, map: &mut Vec<Vec<char>>) {
    let mut dx: i64 = 0;
    let mut dy: i64 = 0;

    match dir {
        '<' => dx = 1,
        '>' => dx = -1,
        '^' => dy = 1,
        'v' => dy = -1,
        _ => println!("error: {}", dir),
    }

    for b in &boxes {
        let oldpos = ((b.0 as i64 + dx) as usize, (b.1 as i64 + dy) as usize);
        map[oldpos.1][oldpos.0] = '.';
        map[oldpos.1][oldpos.0+1] = '.';
    }
    for b in boxes {
        map[b.1][b.0] = '[';
        map[b.1][b.0+1] = ']';
    }
}

fn move_robot(pos: (usize, usize), dir: char, map: &mut Vec<Vec<char>>) -> (usize, usize) {
    match dir {
        '<' => {
            if map[pos.1][pos.0 - 1] == '#' {
                return pos;
            }
            if map[pos.1][pos.0 - 1] == '.' {
                return (pos.0 - 1, pos.1);
            }

            let boxes = move_box((pos.0 - 2, pos.1), dir, map);
            if boxes.len() == 0 {
                return pos;
            }
            clear_boxes(boxes, dir, map);
            return (pos.0 - 1, pos.1);
        },
        '>' => {
            if map[pos.1][pos.0 + 1] == '#' {
                return pos;
            }
            if map[pos.1][pos.0 + 1] == '.' {
                return (pos.0 + 1, pos.1);
            }

            let boxes = move_box((pos.0 + 1, pos.1), dir, map);
            if boxes.len() == 0 {
                return pos;
            }
            clear_boxes(boxes, dir, map);
            return (pos.0 + 1, pos.1);
        },
        '^' => {
            if map[pos.1 - 1][pos.0] == '#' {
                return pos;
            }
            if map[pos.1 - 1][pos.0] == '.' {
                return (pos.0, pos.1 - 1);
            }

            if map[pos.1 - 1][pos.0] == ']' {
                let boxes = move_box((pos.0 - 1, pos.1 - 1), dir, map);
                if boxes.len() == 0 {
                    return pos;
                }
                clear_boxes(boxes, dir, map);
                return (pos.0, pos.1 - 1);
            }
            if map[pos.1 - 1][pos.0] == '[' {
                let boxes = move_box((pos.0, pos.1 - 1), dir, map);
                if boxes.len() == 0 {
                    return pos;
                }
                clear_boxes(boxes, dir, map);
                return (pos.0, pos.1 - 1);
            }
        },
        'v' => {
            if map[pos.1 + 1][pos.0] == '#' {
                return pos;
            }
            if map[pos.1 + 1][pos.0] == '.' {
                return (pos.0, pos.1 + 1);
            }

            if map[pos.1 + 1][pos.0] == ']' {
                let boxes = move_box((pos.0 - 1, pos.1 + 1), dir, map);
                if boxes.len() == 0 {
                    return pos;
                }
                clear_boxes(boxes, dir, map);
                return (pos.0, pos.1 + 1);
            }
            if map[pos.1 + 1][pos.0] == '[' {
                let boxes = move_box((pos.0, pos.1 + 1), dir, map);
                if boxes.len() == 0 {
                    return pos;
                }
                clear_boxes(boxes, dir, map);
                return (pos.0, pos.1 + 1);
            }
        },
        _ => println!("error: {}", dir),
    }
    return pos;
}

fn printmap(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!("  {}", map[y].len());
    }
}

fn part2() -> i64 {
    let input = include_str!("input15.txt").trim();
    let mut result = 0;
    let mut tmp = input.split("\n\n");
    //let mut tmp = input.split("\r\n\r\n");
    let mut tmpmap: Vec<Vec<char>> = tmp
        .next()
        .unwrap()
        .split('\n')
        .map(|x| x.trim().chars().collect())
        .collect();
    let mut moves = tmp.next().unwrap();
    let mut pos = (0, 0);
    let mut map = Vec::new();
    for y in 0..tmpmap.len() {
        let mut row = Vec::new();
        for x in 0..tmpmap[y].len() {
            if tmpmap[y][x] == '#' {
                row.push('#');
                row.push('#');
            }
            if tmpmap[y][x] == 'O' {
                row.push('[');
                row.push(']');
            }
            if tmpmap[y][x] == '.' {
                row.push('.');
                row.push('.');
            }
            if tmpmap[y][x] == '@' {
                row.push('@');
                row.push('.');
            }
        }
        map.push(row);
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                pos = (x, y);
            }
        }
    }

    let w = map[0].len();
    let h = map.len();

    for l in moves.split('\n') {
        for m in l.trim().chars() {
            map[pos.1][pos.0] = '.';
            pos = move_robot(pos, m, &mut map);
            map[pos.1][pos.0] = '@';
            //printmap(&map);
        }
    }

    for y in 0..h {
        for x in 0..w {
            if map[y][x] == '[' {
                result += 100 * (y as i64) + x as i64;
            }
        }
    }

    //printmap(&map);

    return result;
}

fn main() {
    println!("Day 15!");
    println!("{}", part1());
    println!("{}", part2());
}
