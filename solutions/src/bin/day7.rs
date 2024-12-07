fn valid(res: i64, nums: &Vec<i64>, idx: usize, sum: i64) -> bool {
    if idx >= nums.len() {
        return sum == res;
    }

    let mulsum = sum * nums[idx];
    if valid(res, nums, idx+1, mulsum) {
        return true;
    }
    let sumsum = sum + nums[idx];
    if valid(res, nums, idx+1, sumsum) {
        return true;
    }

    return false;
}

fn valid2(res: i64, nums: &Vec<i64>, idx: usize, sum: i64) -> bool {
    if idx >= nums.len() {
        return sum == res;
    }

    let mulsum = sum * nums[idx];
    if valid2(res, nums, idx+1, mulsum) {
        return true;
    }
    let sumsum = sum + nums[idx];
    if valid2(res, nums, idx+1, sumsum) {
        return true;
    }
    let n = nums[idx];
    let mut combsum = sum;
    let mut m = 10;
    while m < n {
        m *= 10;
    }
    combsum *= m;
    combsum += n;
    if valid2(res, nums, idx+1, combsum) {
        return true;
    }

    return false;
}

fn part1() -> i64 {
    let input = include_str!("input7.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let s: Vec<&str> = line.split(':').collect();
        let num = s[0].parse::<i64>().expect("msg");
        let nums: Vec<i64> = s[1].split_whitespace().map(|x| x.parse::<i64>().expect("msg")).collect();
        if valid(num, &nums, 1, nums[0]) {
            result += num;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input7.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let s: Vec<&str> = line.split(':').collect();
        let num = s[0].parse::<i64>().expect("msg");
        let nums: Vec<i64> = s[1].split_whitespace().map(|x| x.parse::<i64>().expect("msg")).collect();
        if valid2(num, &nums, 1, nums[0]) {
            result += num;
        }
    }

    return result;
}

fn main() {
    println!("Day 7!");
    println!("{}", part1());
    println!("{}", part2());
}