use std::fs;

pub fn one() {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];
    fs::read_to_string("inputs/1")
        .expect("failed to read file")
        .lines()
        .for_each(|line| {
            let mut words = line.split_whitespace();
            let a = words.next();
            let b = words.next();
            let an = a
                .unwrap()
                .parse::<i32>()
                .expect("failed to parse first int");
            let bn = b
                .unwrap()
                .parse::<i32>()
                .expect("failed to parse second int");
            l1.push(an);
            l2.push(bn);
        });
    l1.sort();
    l2.sort();
    let mut delta = 0;
    for i in 0..l1.len() {
        let a = l1.get(i).unwrap();
        let b = l2.get(i).unwrap();
        if a >= b {
            delta += a - b;
        } else {
            delta += b - a;
        }
    }
    println!("{}", delta)
}

use std::collections::HashMap;
use std::io::BufRead;

pub fn two() {
    let mut l1: Vec<i32> = vec![];
    let mut counter: HashMap<i32, i32> = HashMap::new();
    fs::read_to_string("inputs/1")
        .expect("failed to read file")
        .lines()
        .for_each(|line| {
            let mut words = line.split_whitespace();
            let a = words.next();
            let b = words.next();
            let an = a
                .unwrap()
                .parse::<i32>()
                .expect("failed to parse first int");
            let bn = b
                .unwrap()
                .parse::<i32>()
                .expect("failed to parse second int");
            counter.insert(bn, counter.get(&bn).unwrap_or_else(|| &0) + 1);
            l1.push(an);
        });
    let mut score = 0;
    for k in l1 {
        let count = counter.get(&k).unwrap_or_else(|| &0);
        score += k * count
    }
    println!("{}", score)
}
