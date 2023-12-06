use crate::{get_input, Result};

fn read_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn dfs(octopi: &mut Vec<Vec<u32>>, i: isize, j: isize) {
    let (m, n) = (octopi.len() as isize, octopi[0].len() as isize);
    let adjacent: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        let item = &mut octopi[x as usize][y as usize];

        if *item == 10 {
            // already flashed!
            continue;
        }

        if *item == 9 {
            stack.extend(
                adjacent
                    .iter()
                    .filter(|(a, b)| x + a >= 0 && x + a < m && y + b >= 0 && y + b < n)
                    .map(|(a, b)| (x + a, y + b)),
            );
        }
        *item += 1;
    }
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<u32> {
    let mut octopi = read_input(input);
    let mut res = 0;

    for _ in 0..100 {
        for i in 0..octopi.len() {
            for j in 0..octopi[0].len() {
                dfs(&mut octopi, i as isize, j as isize);
            }
        }
        for row in octopi.iter_mut() {
            for item in row.iter_mut() {
                if *item == 10 {
                    *item = 0;
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

fn part2(input: &str) -> Result<i32> {
    let mut octopi = read_input(input);

    let mut x = 1;
    loop {
        for i in 0..octopi.len() {
            for j in 0..octopi[0].len() {
                dfs(&mut octopi, i as isize, j as isize);
            }
        }
        let mut isflash = true;
        for row in octopi.iter_mut() {
            for item in row.iter_mut() {
                if *item == 10 {
                    *item = 0;
                } else {
                    isflash = false;
                }
            }
        }
        if isflash {
            return Ok(x);
        }
        x += 1;
    }
}

const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1656, part1(INPUT).unwrap());
        assert_eq!(195, part2(INPUT).unwrap());
    }
}
