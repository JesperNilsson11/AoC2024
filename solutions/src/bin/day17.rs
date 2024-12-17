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
        println!("{}", num);
    }
    let prog = tmp
        .next_back()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .trim();
    println!("{}", prog);
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
    println!("{}", prog);
    let mut program: Vec<i64> = prog.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let base: i64 = 2;
    let mut solves: Vec<Vec<i64>> = Vec::new();
    for _ in 0..program.len() {
        solves.push(Vec::new());
    }
    println!("");
    //result = 35184372088000;
    //while result < 3e14 as i64 {
    while result < 1000 {
        if result % 10000000000 == 0 {
            println!("{}", result);
        }
        reg[0] = result;
        print!("reg A: {} -  ", reg[0]);
        reg[1] = 0;
        reg[2] = 0;
        let mut ans = Vec::new();
        let mut pc = 0;
        let mut idx = 0;

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
                5 => {
                    ans.push(getcombo(oper, &reg) % 8);
                    //println!("ans {}", ans[idx]);
                    //if idx < program.len() && ans[idx] != program[idx] {
                    //    break;
                    //} else if idx >= program.len() {
                    //    break;
                    //}
                    idx += 1;
                }
                6 => reg[1] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                7 => reg[2] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                _ => println!("Error"),
            }
        }

        let mut found = false;
        //if ans.len() == program.len() {
            found = true;
            for i in 0..ans.len() {
                print!("{} ", ans[i]);
                if ans[i] != program[i] {
                    found = false;
                    //break;
                }
            }
            if ans.len() > 1 && ans[0] == 2 && ans[1] == 4 {
                println!("?");
            }
        //}
        if found {
            //println!("{}", result);
            //break;
        }
        result += 1;
        println!("");
    }
    println!("");

    /*println!("----- Solves ------- ");
    for i in 0..solves.len() {
        for x in  &solves[i] {
            print!("{} ", x);
        }
        println!("");
    }*/

    return result;
}

/*fn part3() -> i64 {
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
    println!("{}", prog);
    let mut program: Vec<i64> = prog.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut pc = 0;
    let base: i64 = 2;
    let mut solves: Vec<Vec<i64>> = Vec::new();
    for _ in 0..program.len() {
        solves.push(Vec::new());
    }

    let mut idx = program.len() - 1;
    loop {
        reg[0] = result;
        print!("reg A: {} -  ", reg[0]);
        reg[1] = 0;
        reg[2] = 0;
        pc = 0;
        let mut next_num = program[0];

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
                5 => {
                    reg[1] = program[idx];
                },
                6 => reg[1] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                7 => reg[2] = reg[0] / base.pow(getcombo(oper, &reg) as u32),
                _ => println!("Error"),
            }
        }

        if idx == 0 {
            break;
        }
        idx -= 1;
    }

    return result;
}*/

fn part3() -> i64 {
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
    println!("{}", prog);
    let program: Vec<i64> = prog.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut pc = 0;
    let base: i64 = 2;
    let mut solves: Vec<Vec<i64>> = Vec::new();
    for _ in 0..program.len() {
        solves.push(Vec::new());
    }

    let mut idx = program.len() - 1;
    let mut current_result = 0;
    for _ in 0..program.len() {
        for n in 0..8 {
            result = (current_result << 3) + n;
            reg[0] = result;
            reg[1] = 0;
            reg[2] = 0;
            pc = 0;
            let mut ans = Vec::new();

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
                if program[program.len() -1 - i] != ans[i] {
                    found = false;
                    break;
                }
            }

            if found {
                current_result = result;
                break;
            }
        }
    }

    return result;
}

fn main() {
    println!("Day X!");
    println!("{}", part1());
    println!("{}", part2());
    println!("{}", part3());
}
