use crate::{get_input, Result};

use std::collections::HashMap;

/*
add up part numbers to get missing part.

numbers adjacent to symbols (even diagonal) are part numbers.
periods aren't symbols.

in example, 114 and 58 are not part numbers.

1. find number
2. if is adjacent to symbol, insert into hashmap of those symbol locations.
3. sum all symbols
4. part two, sum * symbols with two numbers (multiplied)
*/

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(3)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Result<i32> {
    solution(input, false)
}

fn part2(input: &str) -> Result<i32> {
    solution(input, true)
}

fn solution(input: &str, p2: bool) -> Result<i32> {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut map = HashMap::new();

    for i in 0..matrix.len() {
        let mut symbol_location: Option<(usize, usize)> = None;
        let mut buf = String::new();

        for j in 0..matrix[0].len() {
            let char = matrix[i][j];
            if char.is_ascii_digit() {
                buf.push(char);
                matrix[i][j] = '.';
                if symbol_location.is_none() {
                    symbol_location = find_symbol(&matrix, (i as isize, j as isize), p2);
                }
            }
            if char == '.' || is_symbol(char, p2) || j == matrix[0].len() - 1 {
                if let Some(loc) = symbol_location {
                    let symbol = matrix[loc.0][loc.1];
                    let n = buf.parse::<i32>()?;
                    map.entry(loc).or_insert(vec![]).push((n, symbol));
                }
                symbol_location = None;
                buf.clear();
            }
        }
    }

    Ok(map.values().fold(0, |sum, vector| {
        if p2 {
            if vector.len() == 2 && vector[0].1 == '*' {
                return sum + vector.iter().map(|t| t.0).product::<i32>();
            }
            return sum;
        }
        sum + vector.iter().map(|t| t.0).sum::<i32>()
    }))
}

fn find_symbol(matrix: &Vec<Vec<char>>, loc: (isize, isize), p2: bool) -> Option<(usize, usize)> {
    let (m, n) = (matrix.len() as isize, matrix[0].len() as isize);
    let dirs = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    for (x, y) in dirs {
        let dx = loc.0 + x;
        let dy = loc.1 + y;
        if dx < 0 || dx >= m || dy < 0 || dy >= n {
            continue; // out of bounds
        }
        if is_symbol(matrix[dx as usize][dy as usize], p2) {
            return Some((dx as usize, dy as usize));
        }
    }

    None
}

fn is_symbol(char: char, p2: bool) -> bool {
    if p2 {
        return char == '*';
    }
    !char.is_ascii_digit() && char != '.'
}

const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(4361, part1(INPUT).unwrap());
        assert_eq!(467835, part2(INPUT).unwrap());
    }
}
