use crate::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn read_input(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().filter_map(|line| {
        let mut nums = line
            .split(&[' ', ',', '-', '>'][..])
            .filter(|char| !char.is_empty())
            .map(|n| n.parse::<i32>().unwrap());

        let (x1, y1) = (nums.next()?, nums.next()?);
        let (x2, y2) = (nums.next()?, nums.next()?);

        Some(Line {
            start: Coordinate { x: x1, y: y1 },
            end: Coordinate { x: x2, y: y2 },
        })
    })
}

fn mutate(grid: &mut HashMap<(i32, i32), u8>, count: &mut i32, entry: (i32, i32)) {
    let coordinate = grid.entry(entry).or_insert(0);
    *coordinate += 1;
    if *coordinate == 2 {
        *count += 1;
    }
}

fn solve(line: &Line, grid: &mut HashMap<(i32, i32), u8>, count: &mut i32, x: bool) {
    let (mut start, mut end) = match x {
        true => (line.start.y, line.end.y),
        false => (line.start.x, line.end.x),
    };

    if end < start {
        std::mem::swap(&mut start, &mut end);
    }

    for j in start..=end {
        let entry = match x {
            true => (line.start.x, j),
            false => (j, line.start.y),
        };
        mutate(grid, count, entry);
    }
}

fn solve_diag(line: &Line, grid: &mut HashMap<(i32, i32), u8>, count: &mut i32) {
    let (mut start_x, mut start_y) = (line.start.x, line.start.y);
    let (end_x, end_y) = (line.end.x, line.end.y);

    let x_dir = if start_x < end_x { 1 } else { -1 };
    let y_dir = if start_y < end_y { 1 } else { -1 };

    loop {
        mutate(grid, count, (start_x, start_y));
        start_x += x_dir;
        start_y += y_dir;

        if start_x == end_x + x_dir {
            break;
        }
    }
}

pub fn part1(input: &str) -> Result<i32> {
    let mut count: i32 = 0;
    let mut grid: HashMap<(i32, i32), u8> = HashMap::new();

    let lines = read_input(input);

    for line in lines {
        if line.start.x == line.end.x {
            solve(&line, &mut grid, &mut count, true);
        } else if line.start.y == line.end.y {
            solve(&line, &mut grid, &mut count, false);
        }
    }

    Ok(count)
}

pub fn part2(input: &str) -> Result<i32> {
    let mut count: i32 = 0;
    let mut grid: HashMap<(i32, i32), u8> = HashMap::new();

    let lines = read_input(input);

    for line in lines {
        if line.start.x == line.end.x {
            solve(&line, &mut grid, &mut count, true);
        } else if line.start.y == line.end.y {
            solve(&line, &mut grid, &mut count, false);
        } else {
            solve_diag(&line, &mut grid, &mut count);
        }
    }

    Ok(count)
}

const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(5, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(12, part2(INPUT).unwrap());
    }
}
