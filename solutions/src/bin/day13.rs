use regex::Regex;

fn part1() -> i64 {
    let input = include_str!("input13.txt").trim();
    let mut result = 0;
    let data: Vec<&str> = input.split("\n\n").collect();

    for d in data {
        let rows: Vec<&str> = d.split('\n').collect();
        let r1 = Regex::new(r"Button .: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let Some(caps) = r1.captures(rows[0]) else {
            println!("Fail");
            return -1;
        };
        let (x1, y1) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());
        let Some(caps) = r1.captures(rows[1]) else {
            println!("Fail");
            return -1;
        };
        let (x2, y2) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());

        let r2 = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
        let Some(caps) = r2.captures(rows[2]) else {
            println!("Fail");
            return -1;
        };
        let (x3, y3) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());

        let mut cost = 0;
        for a in 0..100 {
            for b in 0..100 {
                if a * x1 + b * x2 == x3 && a * y1 + b * y2 == y3 {
                    let c = a * 3 + b;
                    if cost == 0 || c < cost {
                        cost = c;
                    }
                }
            }
        }
        result += cost;
    }
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input13.txt").trim();
    let mut result = 0;
    let data: Vec<&str> = input.split("\n\n").collect();

    for d in data {
        let rows: Vec<&str> = d.split('\n').collect();
        let r1 = Regex::new(r"Button .: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let Some(caps) = r1.captures(rows[0]) else {
            println!("Fail");
            return -1;
        };
        let (x1, y1) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());
        let Some(caps) = r1.captures(rows[1]) else {
            println!("Fail");
            return -1;
        };
        let (x2, y2) = (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap());

        let r2 = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
        let Some(caps) = r2.captures(rows[2]) else {
            println!("Fail");
            return -1;
        };
        let (x3, y3) = (caps[1].parse::<i64>().unwrap() + 10000000000000, caps[2].parse::<i64>().unwrap() + 10000000000000);

        let fx1 = x1 as f64;
        let fy1 = y1 as f64;
        let fx2 = x2 as f64;
        let fy2 = y2 as f64;
        let fx3 = x3 as f64;
        let fy3 = y3 as f64;

        let fb = (fy3 - fy1 * fx3 / fx1) / (fy2 - fy1 * fx2 / fx1);
        let fa = (fx3 - fb * fx2) / fx1;

        let b = fb.round() as i64;
        let a = fa.round() as i64;

        if a * x1 + b * x2 == x3 && a * y1 + b * y2 == y3 {
            result += a * 3 + b;
        }
    }
    

    return result;
}

fn main() {
    println!("Day 13!");
    println!("{}", part1());
    println!("{}", part2());
}