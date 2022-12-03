use crate::{get_input, Result};
use std::collections::HashSet;

type Grid = HashSet<(u32, u32)>;
type Folds<'a> = Vec<(&'a str, u32)>;

fn read_input(input: &str) -> (Grid, Folds) {
    let mut lines = input.lines();
    let mut grid = HashSet::<(u32, u32)>::new();
    let mut folds = Vec::<(&str, u32)>::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (x, y) = line
            .split_once(',')
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
            .expect("malformed input");

        grid.insert((x, y));
    }

    for line in lines {
        let (coord, count) = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .split_once('=')
            .map(|(ch, c)| (ch, c.parse().unwrap()))
            .unwrap();

        folds.push((coord, count));
    }
    (grid, folds)
}

fn fold_grid(grid: Grid, coord: &str, count: u32) -> Grid {
    let mut new_grid = HashSet::<(u32, u32)>::new();
    for (x, y) in grid {
        // where count = 5 and x OR y = 10, simply solve for index = 0.
        if coord == "x" {
            if count < x {
                new_grid.insert((2 * count - x, y));
            } else {
                new_grid.insert((x, y));
            }
        } else if count < y {
            new_grid.insert((x, 2 * count - y));
        } else {
            new_grid.insert((x, y));
        }
    }

    new_grid
}

fn print_grid(grid: Grid) {
    use std::cmp::max;
    let (m, n): (u32, u32) = grid
        .iter()
        .fold((0, 0), |(x, y), &(x2, y2)| (max(x, x2), max(y, y2)));

    let mut v = vec![vec![' '; 1 + n as usize]; 1 + m as usize];

    for (x, y) in grid {
        v[x as usize][y as usize] = '0';
    }

    for j in 0..v[0].len() {
        for i in &v {
            print!("{}", i[j]);
        }
        println!();
    }
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<u32> {
    let (mut grid, folds) = read_input(input);
    if let Some(&(coord, count)) = folds.get(0) {
        grid = fold_grid(grid, coord, count);
    }

    Ok(grid.len() as u32)
}

fn part2(input: &str) -> Result<i32> {
    let (mut grid, folds) = read_input(input);
    for (coord, count) in folds {
        grid = fold_grid(grid, coord, count);
    }
    print_grid(grid);

    Ok(0)
}

const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(17, part1(INPUT).unwrap());
        assert_eq!(0, part2(INPUT).unwrap());
    }
}
