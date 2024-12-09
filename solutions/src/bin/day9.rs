
fn part1() -> i64 {
    let input = include_str!("input9.txt").trim();
    let mut result = 0;
    let mut nums: Vec<i64> = input.chars().map(|x| x.to_digit(10).expect("!") as i64).collect();

    let mut idx = 0;
    let mut pos = 0;
    let mut backidx = nums.len() - 1;

    loop {
        let even = idx % 2 == 0;
        
        if !even {
            while backidx > 1 {
                if nums[backidx] > 0 {
                    let id = backidx as i64 / 2;
                    result += pos * id;
                    nums[backidx] -= 1;
                    break;
                }
                backidx -= 2;
            }
        } else {
            let id = idx as i64 / 2;
            result += pos * id;
        }
        
        nums[idx] -= 1;
        while idx < nums.len() && nums[idx] <= 0 {
            idx += 1;
        }
        pos += 1;

        if backidx < idx {
            break;
        }

        if idx == nums.len() {
            break;
        }
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input9.txt").trim();
    let mut result = 0;
    let nums: Vec<i64> = input.chars().map(|x| x.to_digit(10).expect("!") as i64).collect();
    
    let mut map = Vec::new();
    let mut startidx = 0;
    for i in 0..nums.len() {
        let size = nums[i];
        map.push((startidx, size));
        startidx += size;
    }

    let mut idx = map.len() - 1;
    while idx > 1 {
        let id = idx as i64 / 2;
        let mut spaceidx = 1;
        while spaceidx < map.len() && map[spaceidx].1 < map[idx].1 {
            spaceidx += 2;
        }

        if spaceidx > idx {
            
        } else {
            map[spaceidx].1 -= map[idx].1;
            map[idx].0 = map[spaceidx].0;
            map[spaceidx].0 += map[idx].1;
        }

        for i in 0..map[idx].1 {
            let pos = i + map[idx].0;
            result += pos * id;
        }

        idx -= 2;
    }

    return result;
}

fn main() {
    println!("Day 9!");
    println!("{}", part1());
    println!("{}", part2());
}