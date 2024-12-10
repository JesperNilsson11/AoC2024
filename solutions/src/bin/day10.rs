use std::collections::HashSet;

fn calc(pos: (usize, usize), level: i64, map: &Vec<Vec<i64>>, mut visited: &mut HashSet<(usize, usize)>) {
    if map[pos.1][pos.0] != level {
        return;
    }
    if level == 9 {
        visited.insert((pos.0,pos.1));
        return;
    }

    if pos.0 > 0 {
        calc((pos.0-1,pos.1), level + 1, &map, &mut visited);
    }
    if pos.1 > 0 {
        calc((pos.0,pos.1-1), level + 1, &map, &mut visited);
    }
    if pos.0 < map[pos.1].len() - 1 {
        calc((pos.0+1,pos.1), level + 1, &map, &mut visited);
    }
    if pos.1 < map.len() - 1 {
        calc((pos.0,pos.1+1), level + 1, &map, &mut visited);
    }
}

fn part1() -> i64 {
    let input = include_str!("input10.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<i64>> = input.split('\n').map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i64).collect()).collect();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut set = HashSet::new();
            if map[y][x] == 0 {
                calc((x, y), 0, &map, &mut set);
                result += set.len() as i64;
            }
        }
    }
    
    return result;
}

fn calc2(pos: (usize, usize), level: i64, map: &Vec<Vec<i64>>, mut visited: &mut HashSet<(usize, usize)>) -> i64 {
    let mut res = 0;
    if map[pos.1][pos.0] != level {
        return 0;
    }
    if level == 9 {
        visited.insert((pos.0,pos.1));
        return 1;
    }

    if pos.0 > 0 {
        res += calc2((pos.0-1,pos.1), level + 1, &map, &mut visited);
    }
    if pos.1 > 0 {
        res += calc2((pos.0,pos.1-1), level + 1, &map, &mut visited);
    }
    if pos.0 < map[pos.1].len() - 1 {
        res += calc2((pos.0+1,pos.1), level + 1, &map, &mut visited);
    }
    if pos.1 < map.len() - 1 {
        res += calc2((pos.0,pos.1+1), level + 1, &map, &mut visited);
    }

    return res;
}

fn part2() -> i64 {
    let input = include_str!("input10.txt").trim();
    let mut result = 0;
    let map: Vec<Vec<i64>> = input.split('\n').map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i64).collect()).collect();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let res = calc2((x, y), 0, &map, &mut HashSet::new());
                result += res;
            }
        }
    }
    

    return result;
}

fn main() {
    println!("Day 10!");
    println!("{}", part1());
    println!("{}", part2());
}