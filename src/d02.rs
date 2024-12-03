use std::fs;

fn safe(m: i32, n: i32, asc: bool) -> bool {
    if asc {
        n > m && (n - m) >= 1 && (n - m) <= 3
    } else {
        m > n && (m - n) >= 1 && (m - n) <= 3
    }
}

pub fn one() {
    let mut safe_count = 0;
    fs::read_to_string("inputs/2")
        .expect("failed to read file")
        .lines()
        .for_each(|line| {
            let mut i = 0;
            let mut asc = false;
            let mut last = None;
            let mut line_safe = true;
            for word in line.split(" ") {
                let n = word.parse::<i32>().unwrap();
                if i == 0 {
                    // nothing
                } else if i == 1 {
                    let m = last.unwrap();
                    asc = n >= m;
                    if !safe(m, n, asc) {
                        line_safe = false;
                        break;
                    }
                } else {
                    // i > 1
                    let m = last.unwrap();
                    if !safe(m, n, asc) {
                        line_safe = false;
                        break;
                    }
                }
                i += 1;
                last = Some(n);
            }
            if line_safe {
                safe_count += 1;
            }
        });
    println!("{}", safe_count);
}

fn is_line_safe_one(line: &str) -> bool {
    let readings = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .enumerate();

    let mut asc = None;
    let mut last = None;
    for (_, n) in readings {
        if last.is_some() {
            let m = last.unwrap();
            if asc.is_none() {
                asc = Some(n > m);
            }

            if !safe(m, n, asc.unwrap()) {
                return false;
            }
        }
        last = Some(n)
    }

    true
}

fn is_line_safe_damp(line: &str, damp: Option<usize>) -> bool {
    let readings = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .enumerate()
        .filter(|(idx, _)| {
            if damp.is_none() {
                true
            } else {
                idx != &damp.unwrap()
            }
        });

    let mut delta_array = vec![];
    let mut delta_last = None;
    for (_, n) in readings.clone() {
        if delta_last.is_some() {
            let m = delta_last.unwrap();
            delta_array.push(n - m);
        }
        delta_last = Some(n);
    }
    println!("[d={:?}] {:?} {}", damp, delta_array, line);

    let mut asc = None;
    let mut last = None;

    for (idx, n) in readings.clone() {
        if last.is_some() {
            let m = last.unwrap();
            if asc.is_none() {
                asc = Some(n > m);
            }

            if !safe(m, n, asc.unwrap()) {
                return if damp.is_none() {
                    if idx >= 2 && is_line_safe_damp(line, Some(idx - 2)) {
                        println!("safe by removing n2 {}", idx - 1);
                        true
                    } else if is_line_safe_damp(line, Some(idx - 1)) {
                        println!("safe by removing n1 {}", idx - 1);
                        true
                    } else if is_line_safe_damp(line, Some(idx)) {
                        println!("safe by removing n0 {}", idx);
                        true
                    } else {
                        println!("fail: {} d=null", false);
                        false
                    }
                } else {
                    println!("fail: {} d=?", false);
                    false
                };
            }
        }
        last = Some(n)
    }

    println!("{}", true);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damp() {
        assert!(is_line_safe_damp("7 6 4 2 1", None));
        assert!(!is_line_safe_damp("1 2 7 8 9", None));
        assert!(!is_line_safe_damp("9 7 6 2 1", None));
        assert!(is_line_safe_damp("1 3 2 4 5", None));
        assert!(is_line_safe_damp("8 6 4 4 1", None));
        assert!(is_line_safe_damp("1 3 6 7 9", None));
    }

    // https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    #[test]
    fn test_damp2() {
        assert!(is_line_safe_damp("48 46 47 49 51 54 56", None));
        assert!(is_line_safe_damp("1 1 2 3 4 5", None));
        assert!(is_line_safe_damp("1 2 3 4 5 5", None));
        assert!(is_line_safe_damp("5 1 2 3 4 5", None));
        assert!(is_line_safe_damp("1 4 3 2 1", None));
        assert!(is_line_safe_damp("1 6 7 8 9", None));
        assert!(is_line_safe_damp("1 2 3 4 3", None));
        assert!(is_line_safe_damp("9 8 7 6 7", None));
        assert!(is_line_safe_damp("7 10 8 10 11", None));
        assert!(is_line_safe_damp("29 28 27 25 26 25 22 20", None));

        // https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/m01kqum/
        assert!(is_line_safe_damp("7 10 8 10 11", None));
        assert!(is_line_safe_damp("29 28 27 25 26 25 22 20", None));
    }
}

pub fn two() {
    let mut safe_count = 0;
    let input = fs::read_to_string("inputs/2").expect("failed to read file");
    let lines = input.lines();
    for line in lines {
        println!("---");
        if is_line_safe_damp(line, None) {
            safe_count += 1
        }
    }
    println!("{}", safe_count);
}
