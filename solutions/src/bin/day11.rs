use std::collections::HashMap;

fn part1() -> i64 {
    let input = include_str!("input11.txt").trim();
    let result;
    let mut nums: Vec<i64> = input.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();

    for _ in 0..25 {
        let mut newnums = Vec::new();

        for n in nums {
            let s = n.to_string();
            if n == 0 {
                newnums.push(1);
            } else if s.len() % 2 == 0 {
                let n1 = s[0..s.len() / 2].parse::<i64>().unwrap();
                let n2 = s[s.len() / 2..].parse::<i64>().unwrap();
                newnums.push(n1);
                newnums.push(n2);
            } else {
                newnums.push(n * 2024);
            }
        }

        nums = newnums;
    }

    result = nums.len() as i64;
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input11.txt").trim();
    let mut result = 0;
    let nums: Vec<i64> = input.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut map: HashMap<i64, i64> = HashMap::new();

    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut newmap = HashMap::new();

        for (key, val) in map {
            let s = key.to_string();
            if key == 0 {
                *newmap.entry(1).or_insert(0) += val;
            } else if s.len() % 2 == 0 {
                let n1 = s[0..s.len() / 2].parse::<i64>().unwrap();
                let n2 = s[s.len() / 2..].parse::<i64>().unwrap();
                *newmap.entry(n1).or_insert(0) += val;
                *newmap.entry(n2).or_insert(0) += val;
            } else {
                *newmap.entry(key * 2024).or_insert(0) += val;
            }
        }

        map = newmap;
    }

    for (_, v) in map {
        result += v;
    }
    

    return result;
}

fn main() {
    println!("Day 11!");
    println!("{}", part1());
    println!("{}", part2());
}