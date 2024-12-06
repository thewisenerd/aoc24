use crate::utils::index_math;
use std::cell::Cell;

#[derive(Clone)]
struct Guard {
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
}

#[derive(Clone)]
struct GridCell {
    x: usize,
    y: usize,
    open: Cell<bool>,
    obstacle: bool,
}

struct State {
    grid: Vec<Vec<GridCell>>,
    guard: Guard,
}

fn print_cell(cell: &GridCell, guard: &Guard) -> char {
    if cell.obstacle {
        '#'
    } else if cell.x == guard.x && cell.y == guard.y {
        match (guard.dx, guard.dy) {
            (0, -1) => '^',
            (0, 1) => 'v',
            (1, 0) => '>',
            (-1, 0) => '<',
            _ => panic!("not expected"),
        }
    } else {
        '.'
    }
}

fn parse_cell(ch: char, x: usize, y: usize) -> (GridCell, Option<Guard>) {
    match ch {
        '.' => (
            GridCell {
                x,
                y,
                open: Cell::new(false),
                obstacle: false,
            },
            None,
        ),
        '#' => (
            GridCell {
                x,
                y,
                open: Cell::new(false),
                obstacle: true,
            },
            None,
        ),
        '^' => (
            GridCell {
                x,
                y,
                open: Cell::new(true),
                obstacle: false,
            },
            Some(Guard {
                x,
                y,
                dx: 0,
                dy: -1,
            }),
        ),
        'v' => (
            GridCell {
                x,
                y,
                open: Cell::new(true),
                obstacle: false,
            },
            Some(Guard { x, y, dx: 0, dy: 1 }),
        ),
        '>' => (
            GridCell {
                x,
                y,
                open: Cell::new(true),
                obstacle: false,
            },
            Some(Guard { x, y, dx: 1, dy: 0 }),
        ),
        '<' => (
            GridCell {
                x,
                y,
                open: Cell::new(true),
                obstacle: false,
            },
            Some(Guard {
                x,
                y,
                dx: -1,
                dy: 0,
            }),
        ),
        _ => {
            panic!("unknown cell type");
        }
    }
}

fn parse_grid(input: String) -> State {
    let mut found = None;
    let grid: Vec<Vec<GridCell>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    let (cell, guard) = parse_cell(ch, x, y);
                    if guard.is_some() {
                        found = guard;
                    }
                    cell
                })
                .collect()
        })
        .collect();

    if found.is_none() {
        panic!("guard not found")
    }

    State {
        grid,
        guard: found.unwrap(),
    }
}

fn print_grid(grid: &State) {
    for line in grid.grid.iter() {
        for cell in line {
            print!("{}", print_cell(cell, &grid.guard));
        }
        print!("\n");
    }
    println!("{} {}", grid.guard.x, grid.guard.y);
}

fn next_cell<'a>(grid: &'a Vec<Vec<GridCell>>, guard: &Guard) -> Option<&'a GridCell> {
    let nx = index_math(guard.x, guard.dx)?;
    let ny = index_math(guard.y, guard.dy)?;

    let line: &Vec<GridCell> = grid.get(ny)?;
    let cell = line.get(nx)?;
    Some(cell)
}

fn walk(input: String) -> i32 {
    let mut state = parse_grid(input);
    let mut result = 1; // one guard cell is always open

    loop {
        // print_grid(&state);

        let next = next_cell(&state.grid, &state.guard);
        if next.is_none() {
            break;
        }
        let cell = next.unwrap();
        println!("{} {}", cell.x, cell.y);
        if cell.obstacle {
            let (ndx, ndy) = match (state.guard.dx, state.guard.dy) {
                (-1, 0) => (0, -1), // < : ^
                (0, -1) => (1, 0),  // ^ : >
                (1, 0) => (0, 1),   // > : v
                (0, 1) => (-1, 0),  // v : <
                _ => panic!("not expected"),
            };
            // cannot move _into_ obstacle
            state.guard = Guard {
                x: state.guard.x,
                y: state.guard.y,
                dx: ndx,
                dy: ndy,
            }
        } else {
            // can move into next cell
            if cell.open.get() {
                // do nothing
            } else {
                cell.open.set(true);
                result += 1;
            }
            state.guard = Guard {
                x: cell.x,
                y: cell.y,
                dx: state.guard.dx,
                dy: state.guard.dy,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn one() {
        let input = fs::read_to_string("inputs/6.test").expect("failed to read input");
        let result = walk(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn one_actual() {
        let input = fs::read_to_string("inputs/6").expect("failed to read input");
        let result = walk(input);
        println!("{}", result);
    }
}
