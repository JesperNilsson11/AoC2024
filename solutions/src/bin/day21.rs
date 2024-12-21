use std::{collections::HashMap, result};

fn calc2(code: Vec<char>, level: i64, memo: &mut HashMap<(Vec<char>, i64), i64>) -> i64 {
    let mut result = 0;

    if code.len() == 0 {
        return i64::MAX / 2;
    }

    if memo.contains_key(&((code.clone(), level))) {
        return *memo.get(&(code, level)).unwrap();
    }

    let mut keypad = HashMap::new();
    keypad.insert('^', (1, 0));
    keypad.insert('A', (2, 0));
    keypad.insert('<', (0, 1));
    keypad.insert('v', (1, 1));
    keypad.insert('>', (2, 1));

    let mut pos = (2, 0);
    
    for &c in &code {
        let mut newpresses1 = Vec::new();
        let mut newpresses2 = Vec::new();
        let newpos = keypad.get(&c).unwrap();
        let dx: i32 = newpos.0 - pos.0;
        let dy: i32 = newpos.1 - pos.1;

        if pos.0 != 0 || dy >= 0 {
            for _ in 0..dy.abs() {
                if dy > 0 {
                    newpresses1.push('v');
                } else {
                    newpresses1.push('^');
                }
            }
            for _ in 0..dx.abs() {
                if dx > 0 {
                    newpresses1.push('>');
                } else {
                    newpresses1.push('<');
                }
            }
            newpresses1.push('A');
        }

        if newpos.0 != 0 || dx >= 0{
            for _ in 0..dx.abs() {
                if dx > 0 {
                    newpresses2.push('>');
                } else {
                    newpresses2.push('<');
                }
            }
            for _ in 0..dy.abs() {
                if dy > 0 {
                    newpresses2.push('v');
                } else {
                    newpresses2.push('^');
                }
            }
            newpresses2.push('A');
        }


        if level > 0 {
            let mut tmp = calc2(newpresses1, level - 1, memo);
            let t = calc2(newpresses2, level - 1, memo);
            if tmp > t {
                tmp = t;
            }
            result += tmp;
        } else {
            let mut tmp = newpresses1.len();
            if newpresses2.len() > tmp {
                tmp = newpresses2.len();
            }
            if newpresses1.len() != newpresses2.len() {
                if newpresses1.len() > 0 && newpresses2.len() > 0 {
                    println!("WARNING");
                }
            }
            result += tmp as i64;
        }
        pos = *newpos;
    }

    memo.insert((code, level), result);

    return result;
}

fn calc(code: Vec<char>, level: i64) -> i64 {
    let mut result = 0;
    let mut pos = (2, 3);
    let mut numpad = HashMap::new();
    numpad.insert('7', (0, 0));
    numpad.insert('8', (1, 0));
    numpad.insert('9', (2, 0));
    numpad.insert('4', (0, 1));
    numpad.insert('5', (1, 1));
    numpad.insert('6', (2, 1));
    numpad.insert('1', (0, 2));
    numpad.insert('2', (1, 2));
    numpad.insert('3', (2, 2));
    numpad.insert('0', (1, 3));
    numpad.insert('A', (2, 3));

    for c in code {
        let mut moves1 = Vec::new();
        let mut moves2 = Vec::new();
        let newpos = numpad.get(&c).unwrap();
        let dx: i32 = newpos.0 - pos.0;
        let dy: i32 = newpos.1 - pos.1;

        if newpos.1 != 3 || pos.0 != 0 || dy <= 0 {
            for _ in 0..dy.abs() {
                if dy > 0 {
                    moves1.push('v');
                } else {
                    moves1.push('^');
                }
            }

            for _ in 0..dx.abs() {
                if dx > 0 {
                    moves1.push('>');
                } else {
                    moves1.push('<');
                }
            }
            moves1.push('A');
        }

        if pos.1 != 3 || newpos.0 != 0 || dx >= 0 {
            for _ in 0..dx.abs() {
                if dx > 0 {
                    moves2.push('>');
                } else {
                    moves2.push('<');
                }
            }

            for _ in 0..dy.abs() {
                if dy > 0 {
                    moves2.push('v');
                } else {
                    moves2.push('^');
                }
            }
            moves2.push('A');
        }

        let mut memo = HashMap::new();
        let mut tmp = calc2(moves1, level, &mut memo);
        let t = calc2(moves2, level, &mut memo);
        if tmp > t {
            tmp = t;
        }
        result += tmp;
        pos = *newpos;
    }

    return result;
}

fn part1() -> i64 {
    let input = include_str!("input21.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let num: i64 = line.trim()[0..3].parse().unwrap();
        let tmp = calc(line.trim().chars().collect(), 1);
        result += tmp * num;
    }

    return result;
}

fn part2() -> i64 {
    let input = include_str!("input21.txt").trim();
    let mut result = 0;

    for line in input.split('\n') {
        let num: i64 = line.trim()[0..3].parse().unwrap();
        let tmp = calc(line.trim().chars().collect(), 24);
        result += tmp * num;
    }

    return result;
}

fn main() {
    println!("Day 21!");
    println!("{}", part1());
    println!("{}", part2());
}
