use std::i64;

fn getcombo(combo: i64, reg: &Vec<i64>) -> i64 {
    if combo <= 3 {
        return combo;
    }
    if combo <= 6 {
        return reg[(combo - 4) as usize];
    }
    println!("Fail");
    return -1;
}

fn part1() -> i64 {
    let input = include_str!("input17.txt").trim();
    let mut result = 0;
    let mut reg = Vec::new();
    let mut tmp = input.split('\n');
    for i in 0..3 {
        let line = tmp.next().unwrap();
        let num = line
            .split(':')
            .next_back()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();
        reg.push(num);
    }
    let prog = tmp
        .next_back()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .trim();

    let program: Vec<i64> = prog.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut pc = 0;
    let base: i64 = 2;

    while pc < program.len() {
        let op = program[pc];
        pc += 1;
        let oper = program[pc];
        pc += 1;

        match op {
            0 => reg[0] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
            1 => reg[1] = reg[1] ^ oper,
            2 => reg[1] = getcombo(oper, &reg) % 8,
            3 => {
                if reg[0] != 0 {
                    pc = oper as usize;
                }
            }
            4 => reg[1] = reg[1] ^ reg[2],
            5 => print!("{},", getcombo(oper, &reg) % 8),
            6 => reg[1] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
            7 => reg[2] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
            _ => println!("Error"),
        }
    }
    println!("");

    return result;
}

fn find_a(program: &Vec<i64>, a: i64, idx: usize) -> i64 {
    let mut result = i64::MAX;

    if idx >= program.len() {
        return a;
    }

    let mut reg = vec![0; 3];
    for n in 0..8 {
        let newa = (a << 3) + n;
        reg[0] = newa;
        reg[1] = 0;
        reg[2] = 0;
        let mut pc = 0;
        let mut ans = Vec::new();

        while pc < program.len() {
            let op = program[pc];
            pc += 1;
            let oper = program[pc];
            pc += 1;
            let base: i64 = 2;

            match op {
                0 => reg[0] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                1 => reg[1] = reg[1] ^ oper,
                2 => reg[1] = getcombo(oper, &reg) % 8,
                3 => {
                    if reg[0] != 0 {
                        pc = oper as usize;
                    }
                }
                4 => reg[1] = reg[1] ^ reg[2],
                5 => {
                    ans.push(getcombo(oper, &reg) % 8);
                }
                6 => reg[1] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                7 => reg[2] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                _ => println!("Error"),
            }
        }

        let mut found = true;
        for i in 0..ans.len() {
            if program[program.len() - 1 - i] != ans[ans.len() - 1 - i] {
                found = false;
                break;
            }
        }
        if ans.len() != idx + 1 {
            found = false;
        }

        if found {
            let tmp = find_a(program, newa, idx+1);
            if tmp < result {
                result = tmp;
            }
        }
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input17.txt").trim();
    let mut result = 0;
    let mut reg = Vec::new();
    let mut tmp = input.split('\n');
    for i in 0..3 {
        let line = tmp.next().unwrap();
        let num = line
            .split(':')
            .next_back()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();
        reg.push(num);
    }
    let prog = tmp
        .next_back()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .trim();

    let program: Vec<i64> = prog.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    result = find_a(&program, 0, 0);

    return result;
}

fn main() {
    println!("Day 17!");
    part1();
    println!("{}", part2());
}
