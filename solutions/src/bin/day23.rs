use std::collections::{HashMap, HashSet};

fn part1() -> i64 {
    let input = include_str!("input23.txt").trim();
    let mut result = 0;
    let mut hmap: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.split('\n') {
        let (l, r) = line.trim().split_once('-').unwrap();

        hmap.entry(l).or_insert(Vec::new()).push(r);
        hmap.entry(r).or_insert(Vec::new()).push(l);
    }

    for (&k, v) in &hmap {
        for i in 0..v.len() {
            for j in i+1..v.len() {
                if hmap.get(v[i]).unwrap().contains(&v[j]) {
                    if k[0..1] == *"t" || v[i][0..1] == *"t" || v[j][0..1] == *"t" {
                        result += 1;
                    }
                }
            }

        }
    }
    
    return result / 3;
}

fn part2() -> i64 {
    let input = include_str!("input23.txt").trim();
    let mut result = 0;
    let mut hmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut computers = HashSet::new();

    for line in input.split('\n') {
        let (l, r) = line.trim().split_once('-').unwrap();

        hmap.entry(l).or_insert(Vec::new()).push(r);
        hmap.entry(r).or_insert(Vec::new()).push(l);
        computers.insert(l);
        computers.insert(r);
    }

    let mut lans: Vec<Vec<&str>> = Vec::new();
    for c in computers {
      let mut found = false;
      let connections = hmap.get(c).unwrap();

        for i in 0..lans.len() {
            found = true;
            for j in 0..lans[i].len() {
                if connections.contains(&lans[i][j]) == false {
                    found = false;
                    break;
                }
            }
            if found {
                lans[i].push(c);
            }
        }

        if found == false {
            let newlan = vec![c];
            lans.push(newlan);
        }
    }

    let mut ans = Vec::new();
    for l in lans {
        if l.len() > result {
            result = l.len();
            ans = l.clone();
        }
    }

    ans.sort();
    for a in ans {
        print!("{},", a);
    }
    println!("");
    
    return result as i64;
}

fn main() {
    println!("Day 23!");
    println!("{}", part1());
    println!("{}", part2());
}