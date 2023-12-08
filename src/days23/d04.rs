use std::{cmp::min, collections::HashSet};

use crate::{get_input, Result};
/*

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

list of winning | your numbers
4 matches: 1 point for match 1, double for each of the next (8 points total)

sum your points

pt 2

# of matches is equal to # of following cards you can copy.

so dynamic programming:
         0  1  2  3  4  5  6
we have [0, 1, 1, 1, 1, 1, 1]

1. start with 1 copy of 1.
1. find # of matches for 1. for each following card, multiply cards by # of current cards
2.
*/

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(4)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Result<i32> {
    Ok(get_games(input).fold(0, |sum, acc| sum + calc_points(acc as i32)))
}

fn part2(input: &str) -> Result<i32> {
    let n = input.lines().count();
    let mut cache = vec![1; n];

    for (i, matches) in get_games(input).enumerate() {
        for j in i + 1..min(matches + i + 1, n) {
            cache[j] += cache[i];
        }
    }

    Ok(cache.iter().sum())
}

fn get_games(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|l| {
        let (_, game) = l.split_once(": ").unwrap();
        let (winning, yours) = game.split_once(" | ").unwrap();

        let winning: HashSet<i32> = winning
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        yours
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .filter(|n| winning.contains(n))
            .count()
    })
}

fn solution(input: &str) -> Result<i32> {
    let games = input.lines().fold(0, |acc, l| {
        let (_, game) = l.split_once(": ").unwrap();
        let (winning, yours) = game.split_once(" | ").unwrap();

        let winning: HashSet<i32> = winning
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let matches = yours
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .filter(|n| winning.contains(n))
            .count();

        acc + calc_points(matches as i32)
    });
    Ok(games)
}

fn calc_points(matches: i32) -> i32 {
    if matches == 0 {
        return 0;
    }
    2_i32.pow((matches - 1) as u32)
}

const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(13, part1(INPUT).unwrap());
        assert_eq!(30, part2(INPUT).unwrap());
    }
}
