use crate::{get_input, Result};
use itertools::Itertools;

fn deterministic(mut p1: u64, mut p2: u64) -> u64 {
    let (mut p1_s, mut p2_s) = (0, 0);
    let mut current = &mut p1;
    let mut score = &mut p1_s;
    let mut rolls = 1;

    loop {
        *current = ((3 * rolls + 3 + *current - 1) % 10) + 1;
        *score += *current;
        rolls += 3;
        if *score > 999 {
            break;
        }
        current = if *current == p1 { &mut p2 } else { &mut p1 };
        score = if *score == p1_s { &mut p2_s } else { &mut p1_s };
    }
    rolls -= 1;

    let loser = if *score == p1_s { p2_s } else { p1_s };
    println!("loser:{},rolls:{}", loser, rolls);
    loser * rolls
}

fn read(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|line| line.chars().last().unwrap())
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect_tuple()
        .unwrap()
}
pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<u64> {
    let (p1, p2) = read(input);
    Ok(deterministic(p1, p2))
}

fn part2(input: &str) -> Result<usize> {
    Ok(0)
}

const INPUT: &str = "p1 start: 7\np2 start: 9";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(739785, part1("p1 start: 4\np2 start: 8").unwrap());
        // assert_eq!(444356092776315, part2("p1 start: 4\np2 start: 8").unwrap());
        println!("{}", part1(INPUT).unwrap());
    }
}
