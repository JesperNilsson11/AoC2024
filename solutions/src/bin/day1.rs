
fn part1() -> i64 {
    let mut result = 0;
    let input = include_str!("input1.txt");
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    

    for line in input.trim().split('\n') {
        let split = line.split_whitespace().map(|x| x.parse::<i64>().expect("!")).collect::<Vec<i64>>();
        left.push(split[0]);
        right.push(split[1]);
    }

    left.sort();
    right.sort();

    for (i,l) in left.iter().enumerate() {
        result += (l - right[i]).abs();
    }

    return result;
}

fn part2() -> i64 {
    let mut result = 0;
    let input = include_str!("input1.txt");
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    

    for line in input.trim().split('\n') {
        let split = line.split_whitespace().map(|x| x.parse::<i64>().expect("!")).collect::<Vec<i64>>();
        left.push(split[0]);
        right.push(split[1]);
    }

    for l in &left {
        let mut num_times = 0;
        for r in &right {
            if r == l {
                num_times += 1;
            }
        }
        result += l * num_times;
    }

    return result;
}

fn main() {
    println!("Day 1!");
    println!("{}", part1());
    println!("{}", part2());
}