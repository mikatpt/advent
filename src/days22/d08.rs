use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(8)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}
// 12:15
/*
how many trees are visible from outside?
check visibility from left, right, top, and bottom.

[{3, left: true, right: false, top: true, bottom: false},0,3,7,3]


*/

#[derive(Debug, Default)]
struct Item {
    digit: i32,
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

fn parse(input: &str) -> Vec<Vec<Item>> {
    let mut res: Vec<Vec<Item>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .map(|c| Item {
                    digit: c as i32,
                    ..Default::default()
                })
                .collect()
        })
        .collect();

    let (m, n) = (res.len(), res[0].len());
    for row in res.iter_mut() {
        row[0].left = true;
        row[n - 1].right = true;
    }
    for j in 0..n {
        res[0][j].top = true;
        res[m - 1][j].bottom = true;
    }

    res
}
use std::cmp::max;

fn part1(input: &str) -> Result<i32> {
    let mut digits = parse(input);

    let (m, n) = (digits.len(), digits[0].len());

    #[allow(clippy::needless_range_loop)]
    for i in 1..m - 1 {
        let mut max_left = 0;
        let mut max_right = 0;
        for j in 1..n - 1 {
            let left_cell = digits[i][j].digit;
            let to_left = digits[i][j - 1].digit;
            max_left = max(max_left, to_left);

            // bitwise NOT lets us iterate backwards simultaneously.
            let back = (n as i32 + !j as i32) as usize;
            let right_cell = digits[i][back].digit;
            let to_right = digits[i][back + 1].digit;
            max_right = max(max_right, to_right);

            if max_left < left_cell {
                digits[i][j].left = true;
            }
            if max_right < right_cell {
                digits[i][back].right = true;
            }
        }
    }

    for j in 1..n - 1 {
        let mut max_top = 0;
        let mut max_bot = 0;
        for i in 1..m - 1 {
            let top_cell = digits[i][j].digit;
            let to_top = digits[i - 1][j].digit;
            max_top = max(max_top, to_top);

            let back = (m as i32 + !i as i32) as usize;
            let bot_cell = digits[back][j].digit;
            let to_bot = digits[back + 1][j].digit;
            max_bot = max(max_bot, to_bot);

            if max_top < top_cell {
                digits[i][j].top = true;
            }

            if max_bot < bot_cell {
                digits[back][j].bottom = true;
            }
        }
    }

    let res = digits.into_iter().fold(0, |sum, arr| {
        sum + arr.into_iter().fold(0, |mut s, item| {
            if item.left || item.right || item.top || item.bottom {
                s += 1;
            }
            s
        })
    });

    Ok(res)
}

fn part2(input: &str) -> Result<i32> {
    let digits = parse(input);
    let mut max_score = 0;
    let (m, n) = (digits.len(), digits[0].len());

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            max_score = max(max_score, find_max(i, j, &digits));
        }
    }

    Ok(max_score as i32)
}

fn calc_dir<T: Iterator<Item = usize>>(
    iter: T,
    dir: &str,
    digits: &[Vec<Item>],
    x: usize,
    y: usize,
) -> usize {
    use std::cmp::Ordering::{Equal, Greater};

    let mut res = 0;
    for j in iter {
        res += 1;

        let cmp = match dir {
            "hori" => digits[x][j].digit.cmp(&digits[x][y].digit),
            _ => digits[j][y].digit.cmp(&digits[x][y].digit),
        };
        match cmp {
            Equal | Greater => break,
            _ => {}
        }
    }

    res
}

fn find_max(x: usize, y: usize, digits: &Vec<Vec<Item>>) -> usize {
    let (m, n) = (digits.len(), digits[0].len());

    let left = calc_dir((0..y).rev(), "hori", digits, x, y);
    let right = calc_dir(y + 1..n, "hori", digits, x, y);
    let top = calc_dir((0..x).rev(), "vert", digits, x, y);
    let bot = calc_dir(x + 1..m, "vert", digits, x, y);

    left * right * top * bot
}

const INPUT: &str = "30373
25512
65332
33549
35390";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(21, part1(INPUT).unwrap());
        assert_eq!(8, part2(INPUT).unwrap());
    }
}
