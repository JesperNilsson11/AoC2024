use std::collections::HashMap;

fn valid(pattern: &str, stripes: &Vec<&str>) -> bool {
    if pattern == "" {
        return true;
    }
    for &s in stripes {
        if pattern.len() >= s.len() {
            if &pattern[0..s.len()] == s {
                if valid(&pattern[s.len()..], stripes) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn valid2(map: &mut HashMap<usize, i64>, pattern: &str, stripes: &Vec<&str>, idx: usize) -> i64 {
    let mut result = 0;
    if map.contains_key(&idx) {
        return *map.get(&idx).unwrap();
    }
    if pattern == "" {
        return 1;
    }

    for &s in stripes {
        if pattern.len() >= s.len() {
            if &pattern[0..s.len()] == s {
                result += valid2(map, &pattern[s.len()..], stripes, idx+s.len());
            }
        }
    }

    map.insert(idx, result);

    return result;
}

fn part1() -> i64 {
    let input = include_str!("input19.txt").trim();
    let mut result = 0;
    let mut iter = input.split("\n\n");
    let stripes: Vec<&str> = iter.next().unwrap().split(", ").collect();
    let patterns: Vec<&str> = iter.next().unwrap().split('\n').collect();

    for p in patterns {
        if valid(p, &stripes) {
            result += 1;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input19.txt").trim();
    let mut result = 0;
    let mut iter = input.split("\n\n");
    let stripes: Vec<&str> = iter.next().unwrap().split(", ").collect();
    let patterns: Vec<&str> = iter.next().unwrap().split('\n').collect();

    for p in patterns {
        let mut map: HashMap<usize, i64> = HashMap::new();
        let tmp = valid2(&mut map, p, &stripes, 0);
        result += tmp;
    }
    

    return result;
}

fn main() {
    println!("Day 19!");
    println!("{}", part1());
    println!("{}", part2());
}