use regex::Regex;
use std::fs;

fn calc_one(input: &str) -> i32 {
    let mut result = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for m in re.captures_iter(input) {
        let x = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = m.get(2).unwrap().as_str().parse::<i32>().unwrap();
        result += x * y;
    }

    result
}

pub fn one() {
    let input = fs::read_to_string("inputs/3").expect("failed to read input");
    let result = calc_one(input.as_str());
    println!("{}", result);
}

fn calc_two(input: &str) -> i32 {
    let mut result = 0;
    let mut do_mul = true;

    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do|don't)\(\)").unwrap();
    for m in re.captures_iter(input) {
        let group = m.get(0).unwrap().as_str();
        if group.starts_with("mul") {
            if do_mul {
                let x = m.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let y = m.get(3).unwrap().as_str().parse::<i32>().unwrap();
                result += x * y;
            }
        } else if group.starts_with("do(") {
            do_mul = true;
        } else if group.starts_with("don't(") {
            do_mul = false;
        } else {
            println!("should not happen! {:?}", m);
        }
    }

    result
}

pub fn two() {
    let input = fs::read_to_string("inputs/3").expect("failed to read input");
    println!("{}", calc_two(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(
            calc_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        )
    }

    #[test]
    fn test_two() {
        assert_eq!(
            calc_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
