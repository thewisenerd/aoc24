use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    before: HashSet<i32>,
    after: HashSet<i32>,
}

fn safe(rule_map: &HashMap<i32, Rule>, numbers: Vec<i32>) -> i32 {
    for (i, n) in numbers.iter().enumerate() {
        let rule = rule_map.get(n);
        if rule.is_none() {
            continue;
        }
        let rule = rule.unwrap();

        for (j, m) in numbers.iter().enumerate() {
            if j == i {
                continue;
            } else if j < i {
                if rule.after.contains(m) {
                    println!("violation {} {}", n, m);
                    return 0;
                }
            } else if j > i {
                if rule.before.contains(m) {
                    println!("violation {} {}", n, m);
                    return 0;
                }
            }
        }
    }

    *numbers.get(numbers.len() / 2).unwrap()
}

fn reorder(rule_map: &HashMap<i32, Rule>, numbers: Vec<i32>, depth: usize) -> i32 {
    if depth > (numbers.len() * numbers.len()) {
        panic!("something is wrong!");
    }
    for (i, n) in numbers.iter().enumerate() {
        let rule = rule_map.get(n);
        if rule.is_none() {
            continue;
        }
        let rule = rule.unwrap();
        for (j, m) in numbers.iter().enumerate() {
            if j == i {
                continue;
            } else if j < i {
                if rule.after.contains(m) {
                    let mut next = numbers.clone();
                    next.swap(i, j);
                    println!("violation {} {} {:?} {:?}", i, j, numbers, next);
                    return reorder(rule_map, next, depth + 1);
                }
            } else if j > i {
                if rule.before.contains(m) {
                    let mut next = numbers.clone();
                    next.swap(i, j);
                    println!("violation {} {} {:?} {:?}", i, j, numbers, next);
                    return reorder(rule_map, next, depth + 1);
                }
            }
        }
    }

    *numbers.get(numbers.len() / 2).unwrap()
}

pub fn parse(input: String, part_two: bool) -> i32 {
    let mut rule_map: HashMap<i32, Rule> = HashMap::new();
    let mut result = 0;
    let mut unsafe_result = 0;

    let mut phase = 0;
    for line in input.lines() {
        if line.is_empty() {
            if phase == 0 {
                for (m, rule) in rule_map.iter() {
                    println!("{} {:?}", m, rule);
                }

                phase = 1;
            } else {
                // parsing complete?
                continue;
            }
        } else {
            if phase == 0 {
                let mut iter = line.split("|");
                let a = iter.next().unwrap().parse::<i32>().unwrap();
                let b = iter.next().unwrap().parse::<i32>().unwrap();

                rule_map
                    .entry(a)
                    .or_insert(Rule {
                        before: HashSet::new(),
                        after: HashSet::new(),
                    })
                    .after
                    .insert(b);

                rule_map
                    .entry(b)
                    .or_insert(Rule {
                        before: HashSet::new(),
                        after: HashSet::new(),
                    })
                    .before
                    .insert(a);

                println!("{} {}", a, b);
            } else if phase == 1 {
                println!("safe {}", line);
                let numbers: Vec<i32> = line
                    .split(",")
                    .map(|ns| ns.parse::<i32>().unwrap())
                    .collect();
                let ll = safe(&rule_map, numbers.clone());
                result += ll;
                if ll == 0 {
                    unsafe_result += reorder(&rule_map, numbers.clone(), 0);
                }
            } else {
                // ??
                continue;
            }
        }
    }

    if part_two {
        unsafe_result
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::d05::parse;
    use std::fs;

    #[test]
    fn one() {
        let input = fs::read_to_string("inputs/5.test").expect("failed to read input");
        let result = parse(input, false);
        assert_eq!(result, 143);
    }

    #[test]
    fn one_actual() {
        let input = fs::read_to_string("inputs/5").expect("failed to read input");
        let result = parse(input, false);
        println!("{}", result);
    }

    #[test]
    fn two() {
        let input = fs::read_to_string("inputs/5.test").expect("failed to read input");
        let result = parse(input, true);
        assert_eq!(result, 123);
    }

    #[test]
    fn two_actual() {
        let input = fs::read_to_string("inputs/5").expect("failed to read input");
        let result = parse(input, true);
        println!("{}", result);
    }
}
