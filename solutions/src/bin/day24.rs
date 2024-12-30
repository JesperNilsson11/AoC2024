use std::collections::HashMap;

use regex::Regex;

fn part1() -> i64 {
    let input = include_str!("input24.txt").trim();
    let mut result = 0;
    let (w, c) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    let mut connections = HashMap::new();
    
    for line in w.split('\n') {
        let (l, r) = line.split_once(": ").unwrap();
        let n: i64 = r.parse().unwrap();

        wires.insert(l.to_string(), n);
        //println!("{} {}", l, n);
    }
    for line in c.split('\n') {
        let r = Regex::new(r"(\w+).(\w+).(\w+) -> (\w+)").unwrap();
        let cap = r.captures(line).unwrap();

        let w1 = cap[1].to_string();
        let w2 = cap[3].to_string();
        let op = cap[2].to_string();
        let w3 = cap[4].to_string();
        //println!("{} {} {} {}", w1, op, w2, w3);
        wires.entry(w1.clone()).or_insert(-1);
        wires.entry(w2.clone()).or_insert(-1);
        wires.entry(w3.clone()).or_insert(-1);
        connections.insert(w3, (w1, w2, op));
    }

    let mut found = false;
    while found == false {
        found = true;
        let mut newvals = Vec::new();

            for k in wires.keys() {            
                if *wires.get(k).unwrap() != -1 {
                continue;
            }
            found = false;
            let (w1, w2, op) = connections.get(k).unwrap();
            let v1 = *wires.get(w1).unwrap();
            let v2 = *wires.get(w2).unwrap();
            if v1 == -1 || v2 == -1 {
                continue;
            }
            
            let val;
            if op == "AND" {
                val = v1 & v2;
            } else if op == "OR" {
                val = v1 | v2;
            } else if op == "XOR" {
                val = v1 ^ v2;
            } else {
                val = -1;
                println!("ERROR");
            }
            
            //println!("{} {}", k, val);
            newvals.push((k.clone(), val));
        }

        for (k, v) in newvals {
            *wires.get_mut(&k).unwrap() = v;
        }
    }

    let ans = ["z00", "z01", "z02", "z03", "z04", "z05",
                          "z06", "z07", "z08", "z09",
                          "z10", "z11", "z12", "z13", "z14", "z15", "z16", "z17", "z18", "z19",
                          "z20", "z21", "z22", "z23", "z24", "z25", "z26", "z27", "z28", "z29",
                          "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38", "z39",
                          "z40", "z41", "z42", "z43", "z44", "z45", "z46", "z47", "z48", "z49",
                          "z50", "z51", "z52", "z53", "z54", "z55", "z56", "z57", "z58", "z59"];
    for i in 0..ans.len() {
        if let Some(val) = wires.get(ans[i]) {
            print!("{}", val);
            result |= (val << i);
        }
    }
    println!("");
    
    return result;
}

fn part2() -> i64 {
    let input = include_str!("input24.txt").trim();
    let mut result = 0;
    let (w, c) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    let mut connections = HashMap::new();
    
    for line in w.split('\n') {
        let (l, r) = line.split_once(": ").unwrap();
        let n: i64 = r.parse().unwrap();

        wires.insert(l.to_string(), n);
        //println!("{} {}", l, n);
    }
    for line in c.split('\n') {
        let r = Regex::new(r"(\w+).(\w+).(\w+) -> (\w+)").unwrap();
        let cap = r.captures(line).unwrap();

        let w1 = cap[1].to_string();
        let w2 = cap[3].to_string();
        let op = cap[2].to_string();
        let w3 = cap[4].to_string();
        //println!("{} {} {} {}", w1, op, w2, w3);
        wires.entry(w1.clone()).or_insert(-1);
        wires.entry(w2.clone()).or_insert(-1);
        wires.entry(w3.clone()).or_insert(-1);
        connections.insert(w3, (w1, w2, op));
    }

    let mut x = 0;
    let mut y = 0;
    for i in 0..64 {
        let mut keyx = "x".to_string();
        let mut keyy = "y".to_string();
        if i < 10 {
            keyx += "0";
            keyy += "0";
        }
        keyx += &i.to_string();
        keyy += &i.to_string();
        if let Some(val) = wires.get(&keyx) {
            x |= val << i;
        }
        if let Some(val) = wires.get(&keyy) {
            y |= val << i;
        }
    }
    println!("{} {}", x, y);

    let mut carry = HashMap::new();
    carry.insert("carry00", "wrs");

    for i in 1..45 {
        let mut keyx = "x".to_string();
        let mut keyy = "y".to_string();
        let mut keyz = "z".to_string();
        if i < 10 {
            keyx += "0";
            keyy += "0";
            keyz += "0";
        }
        keyx += &i.to_string();
        keyy += &i.to_string();
        keyz += &i.to_string();

        let mut found = false;
        let mut key = "";
        for (k, (i1, i2, op)) in &connections {
            if (*i1 == keyx && *i2 == keyy) || (*i1 == keyy && *i2 == keyx) {
                if op == "XOR" {
                    key = k;
                    found = true;
                    break;
                }
            }
        }

        if found == false {
            println!("Wrong {} or {}", keyx, keyy);
            continue;
        }

        if connections.contains_key(&keyz) == false {
            println!("wrong {}", keyz);
            continue;
        }
        let (i1, i2, op) = connections.get(&keyz).unwrap();
        if i1 == key || i2 == key {
            if op != "XOR" {
                println!("op not xor: wrong {}", keyz);
            } else if i1 == key {
                key = i2;
            } else {
                key = i1;
            }
        } else {
            println!("Wire wrong {}: i1 {} i2 {} op {} {}", key, i1, i2, op, keyz);
        }
    }

    return result;
}

fn main() {
    println!("Day 24!");
    //println!("{}", part1());
    println!("{}", part2());
}
