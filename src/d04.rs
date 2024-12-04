use std::fs;

// https://stackoverflow.com/a/54035801
fn index_math(n: usize, dn: i32) -> Option<usize> {
    if dn.is_negative() {
        if n > 0 {
            // we know dn will never be > 1
            n.checked_sub(dn.wrapping_abs() as u32 as usize)
        } else {
            None
        }
    } else {
        n.checked_add(dn as usize)
    }
}

fn find(grid: Vec<Vec<i32>>, x: usize, y: usize, current: i32, dx: i32, dy: i32) -> bool {
    let nx = index_math(x, dx);
    let ny = index_math(y, dy);
    if nx.is_none() || ny.is_none() {
        return false;
    }
    let line_t = grid.get(nx.unwrap());
    if line_t.is_none() {
        return false;
    }
    let line = line_t.unwrap();
    let cell: Option<&i32> = line.get(ny.unwrap());
    if cell.is_none() {
        return false;
    }

    let n = cell.unwrap();
    if *n == current + 1 {
        if *n == 4 {
            true
        } else {
            find(grid.clone(), nx.unwrap(), ny.unwrap(), *n, dx, dy)
        }
    } else {
        false
    }
}

fn scan_one(input: String) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|ch| match ch {
                    'X' => 1,
                    'M' => 2,
                    'A' => 3,
                    'S' => 4,
                    _ => 0,
                })
                .collect(),
        );
    }

    let mut found = 0;
    for x in 0..grid.len() {
        let line = grid.get(x).unwrap();
        for y in 0..line.len() {
            let char = line.get(y).unwrap();
            if *char == 1 {
                if find(grid.clone(), x, y, *char, 0, 1) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, 0, -1) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, 1, 0) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, -1, 0) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, 1, 1) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, 1, -1) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, -1, 1) {
                    found += 1;
                }
                if find(grid.clone(), x, y, *char, -1, -1) {
                    found += 1;
                }
            }
        }
    }

    found
}

pub fn one() {
    let input = fs::read_to_string("inputs/4").unwrap();
    let result = scan_one(input);
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = fs::read_to_string("inputs/4.test").unwrap();
        assert_eq!(scan_one(input), 18);
    }
}