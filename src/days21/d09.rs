use crate::Result;

fn in_bounds(m: isize, n: isize, i: isize, j: isize) -> bool {
    i > -1 && i < m && j > -1 && j < n
}

fn is_lowest(map: &[Vec<u32>], x: isize, y: isize) -> bool {
    let (m, n) = (map.len() as isize, map[0].len() as isize);

    for (i, j) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
        if in_bounds(m, n, i, j) && map[i as usize][j as usize] <= map[x as usize][y as usize] {
            return false;
        }
    }
    true
}
pub fn part1(input: &str) -> Result<u32> {
    let mut res = 0;

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    for (x, row) in map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if is_lowest(&map, x as isize, y as isize) {
                res += cell + 1;
            }
        }
    }

    Ok(res)
}

use std::collections::{HashSet, VecDeque};

pub fn part2(input: &str) -> Result<u32> {
    let mut basins: Vec<u32> = vec![];

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (m, n) = (map.len() as isize, map[0].len() as isize);

    let mut bfs = |x: usize, y: usize| {
        let mut size = 0;
        let mut stack = VecDeque::<(usize, usize)>::from([(x, y)]);
        let mut visited = HashSet::<(usize, usize)>::new();

        while let Some((i, j)) = stack.pop_front() {
            if map[i][j] == 9 || visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            size += 1;

            let (i, j) = (i as isize, j as isize);
            for (i2, j2) in [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)] {
                if in_bounds(m, n, i2, j2) {
                    if map[i2 as usize][j2 as usize] < map[x][y] {
                        return;
                    }
                    stack.push_back((i2 as usize, j2 as usize));
                }
            }
        }
        if size > 0 {
            basins.push(size);
        }
    };

    for (x, row) in map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            bfs(x, y);
        }
    }
    basins.sort_by(|a, b| b.cmp(a));

    Ok(basins.iter().take(3).product())
}

const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(15, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(1134, part2(INPUT).unwrap());
    }
}
