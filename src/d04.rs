use std::fs;
use crate::utils::index_math;

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

fn scan_two(input: String) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|ch| match ch {
                    'M' => 1,
                    'A' => 2,
                    'S' => 3,
                    _ => 0,
                })
                .collect(),
        )
    }

    let mut found = 0;

    for x in 1..grid.len() {
        let line = grid.get(x).unwrap();
        let prev = grid.get(x - 1);
        let next = grid.get(x + 1);
        if prev.is_some() && next.is_some() {
            for y in 1..line.len() {
                let char = line.get(y).unwrap();

                if *char == 2 {
                    let nw = prev.unwrap().get(y - 1);
                    let se = next.unwrap().get(y + 1);

                    let ne = prev.unwrap().get(y + 1);
                    let sw = next.unwrap().get(y - 1);

                    if nw.is_some() && ne.is_some() && sw.is_some() && se.is_some() {
                        let nwc = nw.unwrap();
                        let sec = se.unwrap();

                        let nec = ne.unwrap();
                        let swc = sw.unwrap();

                        if (*nwc == 1 && *sec == 3 || *sec == 1 && *nwc == 3)
                            && (*nec == 1 && *swc == 3 || *nec == 3 && *swc == 1) {
                            found += 1;
                        }

                        // this is bugged because i can have nwc && sec = 2
                        // leading to a higher number
                        // if (*nwc + *sec == 4) && (*nec + *swc == 4) {
                        //     found += 1;
                        // }
                    }
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

pub fn two() {
    let input = fs::read_to_string("inputs/4").unwrap();
    let result = scan_two(input);
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

    #[test]
    fn test_two() {
        let input = fs::read_to_string("inputs/4.test").unwrap();
        assert_eq!(scan_two(input), 9);
    }
}
