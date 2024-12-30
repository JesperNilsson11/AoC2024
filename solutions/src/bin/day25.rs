
fn part1() -> i64 {
    let input = include_str!("input25.txt").trim();
    let mut result = 0;
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for grid in input.split("\n\n") {
        let map: Vec<Vec<char>> = grid.split('\n').map(|x| x.chars().collect()).collect();
        let mut nums = Vec::new();
        let key = map[0][0] == '#';
        for x in 0..map[0].len() {
            let mut count = 0;
            for y in 0..map.len() {
                if map[y][x] == '#' {
                    count += 1;
                }
            }
            nums.push(count);
        }

        if key {
            keys.push(nums);
        } else {
            locks.push(nums);
        }
    }

    let height = 7;
    for l in locks {
        for k in &keys {
            let mut okay = true;
            for i in 0..l.len() {
                if l[i] + k[i] > height {
                    okay = false;
                    break;
                }
            }
            if okay {
                result += 1;
            }
        }
    }
    
    return result;
}

fn main() {
    println!("Day 25!");
    println!("{}", part1());
}