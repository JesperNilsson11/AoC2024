use std::collections::{HashMap, HashSet};

fn magic_num(mut num: i64) -> i64 {
    let tmp = num * 64;
    num ^= tmp;
    num %= 16777216;

    let tmp = num / 32;
    num ^= tmp;
    num %= 16777216;

    let tmp = num * 2048;
    num ^= tmp;
    num %= 16777216;

    return num;
}

fn part1() -> i64 {
    let input = include_str!("input22.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let mut num: i64 = line.trim().parse().unwrap();

        for _ in 0..2000 {
            num = magic_num(num);
        }

        result += num;
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input22.txt").trim();
    let mut result = 0;
    let mut hmap = HashMap::new();

    for line in input.split('\n') {
        let mut num: i64 = line.trim().parse().unwrap();
        let mut seq = Vec::new();
        let mut visited = HashSet::new();

        for _ in 0..3 {
            let oldnum = num;
            num = magic_num(num);
            seq.push((num % 10) - (oldnum % 10));
        }

        for _ in 3..2000 {
            let oldnum = num;
            num = magic_num(num);
            let diff = (num % 10) - (oldnum % 10);
            seq.push(diff);
            let idx = seq.len() - 1;
            let key = (seq[idx - 3], seq[idx - 2], seq[idx - 1], seq[idx]);
            if visited.contains(&key) {
                continue;
            }
            if num < 0 {
                println!("Warning");
            }

            visited.insert(key);
            let e = hmap.entry(key).or_insert(0); 
            *e += num % 10;
            if *e > result {
                result = *e;
            }
        }
    }

    return result;
}

fn main() {
    println!("Day 22!");
    println!("{}", part1());
    println!("{}", part2());
}
