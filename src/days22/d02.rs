use crate::{get_input, Result};
use phf::{phf_map, Map};

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

static MOVES: Map<&'static str, i32> = phf_map! {
    "A" => ROCK,
    "B" => PAPER,
    "C" => SCISSORS,
    "X" => ROCK,
    "Y" => PAPER,
    "Z" => SCISSORS,
};

static OUTCOMES: Map<&'static str, i32> = phf_map! {
    "X" => LOSE,
    "Y" => DRAW,
    "Z" => WIN,
};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(2)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn p1(input: &str) -> Option<i32> {
    let mut score = 0;

    for line in input.lines() {
        let (opponent, you) = line.split_once(' ')?;
        let (&op, &you) = (MOVES.get(opponent)?, MOVES.get(you)?);
        let mut round_score = you;

        let win = you - op == 1 || you - op == -2;
        let draw = you == op;

        round_score += match (win, draw) {
            (true, _) => WIN,
            (_, true) => DRAW,
            _ => LOSE,
        };
        score += round_score;
    }
    Some(score)
}

fn p2(input: &str) -> Option<i32> {
    let mut score = 0;

    for line in input.lines() {
        let (opponent, condition) = line.split_once(' ')?;
        let (&op, &cond) = (MOVES.get(opponent)?, OUTCOMES.get(condition)?);

        let you = match (op, cond) {
            (_, DRAW) => op,
            (ROCK, WIN) | (SCISSORS, LOSE) => PAPER,
            (ROCK, LOSE) | (PAPER, WIN) => SCISSORS,
            (PAPER, LOSE) | (SCISSORS, WIN) => ROCK,
            _ => unreachable!(),
        };
        score += you + cond;
    }

    Some(score)
}

pub fn part1(input: &str) -> Result<i32> {
    Ok(p1(input).unwrap())
}

pub fn part2(input: &str) -> Result<i32> {
    Ok(p2(input).unwrap())
}

const INPUT: &str = "A Y
B X
C Z";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(15, part1(INPUT).unwrap());
        assert_eq!(12, part2(INPUT).unwrap());
    }
}
