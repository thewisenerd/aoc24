use std::{fs, io};
use std::io::Write;

struct Input {
    answer: u64,
    numbers: Vec<u64>,
}

fn parse_input(input: String) -> Vec<Input> {
    let mut lines: Vec<Input> = vec![];

    for line in input.lines() {
        println!("parsing {}", line);
        let pair = line.split_once(": ");
        if let Some((a, b)) = pair {
            let answer = a.parse::<u64>().expect("failed to parse number");
            let numbers = b
                .split(" ")
                .map(|ns| ns.parse::<u64>().expect("failed to parse number"))
                .collect();
            lines.push(Input { answer, numbers });
        }
    }

    lines
}

fn permutations(numbers: &Vec<u64>) -> Vec<u64> {
    println!(
        "permutations {:?} {}",
        numbers,
        2u64.pow((numbers.len() - 1) as u32)
    );
    let mut permutation_list: Vec<u64> = vec![];
    for idx in 0..2u64.pow((numbers.len() - 1) as u32) {
        let mut result = 0;
        for (nidx, n) in numbers.iter().enumerate() {
            if nidx == 0 {
                result = *n;
            } else {
                let mask = 1 << (nidx - 1);
                if (idx & mask) != 0 {
                    result = result * n;
                } else {
                    result = result + n;
                }
            }
        }
        permutation_list.push(result);
    }
    permutation_list
}

fn possible_op(input: &Input) -> bool {
    for answer in permutations(&input.numbers) {
        if answer == input.answer {
            return true;
        }
    }

    false
}

fn sum_possible_op(lines: Vec<Input>) -> u64 {
    let mut result = 0;
    for line in lines.iter() {
        if possible_op(line) {
            println!("possible");
            result += line.answer;
        } else {
            println!("not possible");
        }
    }
    result
}

fn base3_op(mut perm: u64, idx: usize) -> u64 {
    let mut ret: Vec<u64> = vec![];
    while perm > 0 {
        ret.push(perm % 3);
        perm /= 3;
    }
    *ret.get(idx).unwrap_or(&0)
}

fn permutations3(numbers: &Vec<u64>, answer: u64) -> bool {
    println!(
        "permutations3 {} {:?} {}",
        answer,
        numbers,
        3u64.pow((numbers.len() - 1) as u32)
    );
    for perm in 0..3u64.pow((numbers.len() - 1) as u32) {
        let mut result = 0;
        for (nidx, n) in numbers.iter().enumerate() {
            if nidx == 0 {
                result = *n;
            } else {
                match base3_op(perm, nidx - 1) {
                    0 => {
                        result *= n;
                    }
                    1 => {
                        result += n;
                    }
                    2 => {
                        result = format!("{}{}", result, n)
                            .parse::<u64>()
                            .expect("failed to concat numbers");
                    }
                    _ => {
                        println!("unknown operation!")
                    }
                }
            }
        }
        if result == answer {
            return true;
        }
    }

    false
}

fn sum_possible_op3(lines: Vec<Input>) -> u64 {
    let mut result = 0;
    for line in lines.iter() {
        println!("{} {:?}", line.answer, line.numbers);
        io::stdout().flush().expect("failed flush");
        if permutations3(&line.numbers, line.answer) {
            println!("possible");
            result += line.answer;
        } else {
            println!("not possible");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        assert!(possible_op(&Input {
            answer: 190,
            numbers: vec![10, 19]
        }));
    }

    #[test]
    fn test3() {
        assert!(permutations3(&vec![15, 6], 156));
        assert!(permutations3(&vec![6, 8, 6, 15], 7290));
    }

    #[test]
    fn one() {
        let input = fs::read_to_string("inputs/7.test").expect("failed to read input");
        let lines = parse_input(input);
        assert_eq!(sum_possible_op(lines), 3749);
    }

    #[test]
    fn one_impl() {
        let input = fs::read_to_string("inputs/7").expect("failed to read input");
        let lines = parse_input(input);
        let result = sum_possible_op(lines);
        println!("{}", result);
    }

    #[test]
    fn two() {
        let input = fs::read_to_string("inputs/7.test").expect("failed to read input");
        let lines = parse_input(input);
        assert_eq!(sum_possible_op3(lines), 11387);
    }

    #[test]
    fn two_impl() {
        let input = fs::read_to_string("inputs/7").expect("failed to read input");
        let lines = parse_input(input);
        let result = sum_possible_op3(lines);
        println!("{}", result);
    }
}

pub fn two() {
    let input = fs::read_to_string("inputs/7").expect("failed to read input");
    let lines = parse_input(input);
    let result = sum_possible_op3(lines);
    println!("{}", result);
}