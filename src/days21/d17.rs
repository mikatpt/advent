use crate::{eyre, Result};

struct Target {
    x: (i32, i32),
    y: (i32, i32),
}

fn read(input: &str) -> Option<Target> {
    let comma = input.find(',')?;
    let convert_str = |(a, b): (&str, &str)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap());

    let mut x = input[15..comma].split_once("..").map(convert_str)?;
    let mut y = input[comma + 4..].split_once("..").map(convert_str)?;

    if x.0 > x.1 {
        std::mem::swap(&mut x.0, &mut x.1);
    }
    if y.0 > y.1 {
        std::mem::swap(&mut y.0, &mut y.1);
    }

    Some(Target { x, y })
}

use std::cmp::max;

fn get_valid_max(t: &Target, mut xvel: i32, mut yvel: i32) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;

    while y > t.y.0 {
        x += xvel;
        y += yvel;
        yvel -= 1;
        max_y = max(max_y, y);

        if xvel != 0 {
            xvel += if xvel < 0 { 1 } else { -1 };
        }

        if x >= t.x.0 && x <= t.x.1 && y >= t.y.0 && y <= t.y.1 {
            return Some(max_y);
        }
    }
    None
}

fn find_valid(t: &Target) -> (i32, i32) {
    let mut highest = i32::MIN;
    let mut count = 0;

    for i in 0..t.x.1 + 1 {
        for j in t.y.0..1000 {
            if let Some(high) = get_valid_max(t, i, j) {
                count += 1;
                highest = max(highest, high);
            }
        }
    }

    (count, highest)
}

pub fn part1(input: &str) -> Result<i32> {
    let target = read(input).ok_or_else(|| eyre!("malformed input"))?;
    let (_, highest) = find_valid(&target);

    Ok(highest)
}

pub fn part2(input: &str) -> Result<i32> {
    let target = read(input).ok_or_else(|| eyre!("malformed input"))?;
    let (count, _) = find_valid(&target);
    Ok(count)
}

const INPUT: &str = "target area: x=20..30, y=-10..-5";
const PUZZLE_INPUT: &str = "target area: x=32..65, y=-225..-177";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(45, part1(INPUT).unwrap());
    }

    #[test]
    fn solve() {
        dbg!(part2(PUZZLE_INPUT).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(0, part2(INPUT).unwrap());
    }
}
