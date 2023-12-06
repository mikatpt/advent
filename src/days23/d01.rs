use crate::{get_input, Result};

/*
each line, calibration value can be found by combining first digit and last digit to form single two digit number.

so 12 + 38 + 15 + 77 = 142
*/

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .fold(0, |prev, acc| {
            prev + if acc.is_empty() {
                0
            } else {
                String::from_iter([acc[0], acc[acc.len() - 1]])
                    .parse()
                    .unwrap()
            }
        }))
}

use std::collections::HashMap;
#[derive(Default)]
struct Trie {
    value: Option<u32>,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn insert<S: AsRef<str>>(&mut self, word: S, value: u32) {
        let mut pointer = self;

        for char in word.as_ref().chars() {
            pointer = pointer.children.entry(char).or_default();
        }
        pointer.value = Some(value);
    }

    fn find_first_number(&mut self, chars: Vec<char>) -> Option<char> {
        let len = chars.len();

        for i in 0..len {
            let mut pointer = &mut *self;
            for c in chars.iter().take(len).skip(i) {
                if let Some(value) = pointer.value {
                    return std::char::from_digit(value, 10);
                }

                if c.is_ascii_digit() {
                    return Some(*c);
                }

                if pointer.children.contains_key(c) {
                    pointer = pointer.children.get_mut(c).unwrap();
                } else {
                    break;
                }
            }
        }

        None
    }
}

fn make_tries() -> (Trie, Trie) {
    let n = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let rev: Vec<String> = n
        .clone()
        .into_iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    let mut forwards = Trie::default();
    let mut backwards = Trie::default();

    for (i, num) in n.iter().enumerate() {
        forwards.insert(num, i as u32 + 1);
        backwards.insert(&rev[i], i as u32 + 1);
    }
    (forwards, backwards)
}

fn part2(input: &str) -> Result<i32> {
    let (mut forwards, mut backwards) = make_tries();

    let mut sum = 0;
    for line in input.lines() {
        let first = forwards.find_first_number(line.chars().collect());
        let last = backwards.find_first_number(line.chars().rev().collect());
        sum += match (first, last) {
            (Some(f), Some(l)) => String::from_iter([f, l]).parse().unwrap(),
            _ => 0,
        }
    }
    Ok(sum)
}

const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(142, part1(INPUT).unwrap());
        assert_eq!(281, part2(INPUT2).unwrap());
    }
}
