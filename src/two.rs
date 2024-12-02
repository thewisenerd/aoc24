use std::fs;

fn safe(m: i32, n: i32, asc: bool) -> bool {
    if asc {
        n >= m && (n - m) >= 1 && (n - m) <= 3
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
