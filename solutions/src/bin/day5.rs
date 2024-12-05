use std::collections::HashSet;

fn valid(map: &HashSet<i64>, nums: &Vec<i64>) -> bool {
    for i in 1..nums.len() {
        let val = (nums[i] << 32) | nums[i - 1];
        if map.contains(&val) {
            return false;
        }
    }

    return true;
}

fn part1() -> i64 {
    let input: Vec<&str> = include_str!("input5.txt").trim().split("\n\n").collect();
    let mut result = 0;
    let mut map: HashSet<i64> = HashSet::new();

    for line in input[0].split('\n') {
        let pair = line
            .split('|')
            .map(|x| x.parse::<i64>().expect("!"))
            .collect::<Vec<i64>>();
        let val = (pair[0] << 32) | pair[1];
        map.insert(val);
    }

    for line in input[1].split('\n') {
        let nums = line
            .split(',')
            .map(|x| x.parse::<i64>().expect("!"))
            .collect::<Vec<i64>>();

        if valid(&map, &nums) {
            result += nums[nums.len() / 2];
        }
    }

    return result;
}

fn part2() -> i64 {
    let input: Vec<&str> = include_str!("input5.txt").trim().split("\n\n").collect();
    let mut result = 0;
    let mut map: HashSet<i64> = HashSet::new();

    for line in input[0].split('\n') {
        let pair = line
            .split('|')
            .map(|x| x.parse::<i64>().expect("!"))
            .collect::<Vec<i64>>();
        let val = (pair[0] << 32) | pair[1];
        map.insert(val);
    }

    for line in input[1].split('\n') {
        let mut nums = line
            .split(',')
            .map(|x| x.parse::<i64>().expect("!"))
            .collect::<Vec<i64>>();
        if valid(&map, &nums) == false {
            let mut valid = false;
            while valid == false {
                valid = true;
                for i in 1..nums.len() {
                    let val = (nums[i] << 32) | nums[i - 1];
                    if map.contains(&val) {
                        valid = false;
                        let tmp = nums[i];
                        nums[i] = nums[i-1];
                        nums[i-1] = tmp;
                    }
                }
            }

            result += nums[nums.len() / 2];
        }
    }

    return result;
}

fn main() {
    println!("Day 5!");
    println!("{}", part1());
    println!("{}", part2());
}
