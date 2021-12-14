use crate::Result;
use std::collections::HashMap;

type Cache = HashMap<(char, char), i64>;
type Instructions = HashMap<(char, char), char>;

fn read(input: &str) -> (Cache, Instructions) {
    let (template, instructions) = input.split_once("\n\n").unwrap();

    let mut cache = HashMap::new();
    let instructions = instructions.lines().fold(HashMap::new(), |mut map, line| {
        let (pair, to_insert) = line.split_once(" -> ").unwrap();
        let (f, s) = (pair.chars().next().unwrap(), pair.chars().last().unwrap());
        map.insert((f, s), to_insert.chars().next().unwrap());
        map
    });

    for (i, ch) in template.chars().enumerate() {
        if let Some(next) = template.chars().nth(i + 1) {
            *cache.entry((ch, next)).or_default() += 1;
        }
    }

    (cache, instructions)
}

fn get_max_min(cache: Cache) -> (i64, i64) {
    use std::cmp::{max, min};
    let mut counts = HashMap::<char, i64>::new();
    for (pair, count) in cache {
        let (first, second) = pair;
        *counts.entry(first).or_default() += count / 2;
        *counts.entry(second).or_default() += count / 2;
    }
    counts
        .into_iter()
        .fold((0, i64::MAX), |(m, n), (pair, count)| {
            (max(m, count), min(n, count))
        })
}

/// Each pair will result in the following iteration including (pair.0, next) AND (next, pair.1)
/// where next = the character to insert based on the given instructions.
fn solve(input: &str, iterations: usize) -> i64 {
    let (mut cache, instructions) = read(input);

    for _ in 0..iterations {
        let mut new_cache = HashMap::new();
        for (pair, count) in cache.iter_mut() {
            let &(first, second) = pair;
            if let Some(next) = instructions.get(pair) {
                *new_cache.entry((first, *next)).or_default() += *count;
                *new_cache.entry((*next, second)).or_default() += *count;
            }
        }
        cache = new_cache;
    }

    let (max, min) = get_max_min(cache);

    max - min
}

pub fn part1(input: &str) -> Result<i64> {
    Ok(solve(input, 10))
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(solve(input, 40))
}

const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(1588, part1(INPUT).unwrap());
    }
    #[test]
    fn test2() {
        assert_eq!(2188189693529, part2(INPUT).unwrap());
    }
}
