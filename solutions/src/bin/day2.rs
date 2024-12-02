fn valid(nums: &Vec<i64>) -> bool {
    let less = nums[0] < nums[1];
        for i in 0..nums.len()-1 {
            if ((nums[i] < nums[i+1]) == less) && (nums[i]-nums[i+1]).abs() < 4 && nums[i] != nums[i+1] {

            } else {
                return false;
            }
        }
    return true;
}

fn part1() -> i64 {
    let input = include_str!("input2.txt").trim();
    let mut result = 0;
    
    for line in input.split('\n') {
        let nums = line.split_whitespace().map(|x| x.parse::<i64>().expect("!")).collect::<Vec<i64>>();

        if valid(&nums) {
            result += 1;
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input2.txt").trim();
    let mut result = 0;
    
    for line in input.split('\n') {
        let nums = line.split_whitespace().map(|x| x.parse::<i64>().expect("!")).collect::<Vec<i64>>();

        if valid(&nums) {
            result += 1;
        } else {
            for i in 0..nums.len() {
                let mut newnums = nums.to_vec();
                newnums.remove(i);

                if valid(&newnums) {
                    result += 1;
                    break;
                }
            }
        }
    }

    return result;
}

fn main() {
    println!("Day 2!");
    println!("{}", part1());
    println!("{}", part2());
}