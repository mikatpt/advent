use std::collections::HashSet;

use crate::{get_input, Result};

pub fn solve() -> Result<(i32, String)> {
    let input = get_input(10)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Option<i32> {
    let targets: HashSet<i32> = HashSet::from_iter([20, 60, 100, 140, 180, 220]);
    let mut result = vec![];
    let (mut cycle, mut x) = (1, 1);

    let mut lines = input.lines().peekable();
    let mut amt: Option<&str> = None;

    while lines.peek().is_some() {
        if let Some(amount) = amt.take() {
            x += amount.parse::<i32>().expect("this should be an integer");
        } else if let Some(line) = lines.next() {
            amt = line.split_whitespace().nth(1);
        }
        cycle += 1;
        if targets.contains(&cycle) {
            result.push(cycle * x);
        }
    }

    Some(result.into_iter().sum())
}

/*
Sprite is 3 pixels wide.
Each cycle, draw 3 pixels centered at X.
If at least one pixel is visible, fill in this cycle's position with #, otherwise .
*/
fn part2(input: &str) -> Option<String> {
    let mut x = 1;
    let mut amt: Option<&str> = None;
    let mut buffer = String::with_capacity(240);

    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        for i in 0..40 {
            let to_draw = if is_visible(i, x) { '#' } else { '.' };
            buffer.push(to_draw);

            if let Some(amount) = amt.take() {
                x += amount.parse::<i32>().expect("this should be an integer");
            } else if let Some(line) = lines.next() {
                amt = line.split_whitespace().nth(1);
            }
        }
    }
    Some(buffer)
}

fn is_visible(cycle: i32, x: i32) -> bool {
    cycle == x - 1 || cycle == x || cycle == x + 1
}

const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const OUTPUT: &str = "\
##..##..##..##..##..##..##..##..##..##..\
###...###...###...###...###...###...###.\
####....####....####....####....####....\
#####.....#####.....#####.....#####.....\
######......######......######......####\
#######.......#######.......#######.....";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        crate::trace::init();
        assert_eq!(13140, part1(INPUT).unwrap());
        assert_eq!(OUTPUT, part2(INPUT).unwrap());
    }
}
