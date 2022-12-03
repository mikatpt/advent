use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(3)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

use std::collections::{HashMap, HashSet};

fn get_values() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    let alpha = "abcdefghijklmnopqrstuvwxyz";

    for (i, char) in alpha.chars().enumerate() {
        let i = i as i32;
        map.insert(char, i + 1);
        map.insert(char.to_ascii_uppercase(), i + 1 + 26);
    }

    map
}

fn part1(input: &str) -> Option<i32> {
    let val_map = get_values();

    let mut prio_sum = 0;
    for line in input.lines() {
        let mut set = HashSet::new();
        let mut len = line.len() / 2;
        let mut chars = line.chars();
        while len > 0 && let Some(char) = chars.next() {
            len -= 1;
            set.insert(char);
        }

        for char in chars {
            if set.contains(&char) {
                prio_sum += val_map.get(&char)?;
                break;
            }
        }
    }

    Some(prio_sum)
}

fn part2(input: &str) -> Option<i32> {
    let val_map = get_values();

    let mut group_sum = 0;

    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let mut set = HashSet::new();
        let mut matches = HashSet::new();

        for char in lines.next()?.chars() {
            set.insert(char);
        }

        for char in lines.next()?.chars() {
            if set.contains(&char) {
                matches.insert(char);
            }
        }

        for char in lines.next()?.chars() {
            if matches.contains(&char) {
                group_sum += val_map.get(&char)?;
                break;
            }
        }
    }

    Some(group_sum)
}

const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(157, part1(INPUT).unwrap());
        assert_eq!(70, part2(INPUT).unwrap());
    }
}
