use crate::{eyre, Result};
use std::collections::VecDeque;
type Snails = VecDeque<Snail>;

#[derive(Debug)]
enum Snail {
    Symbol(char),
    Num(u32),
}

fn increment_explosion<'a, I>(snails: I, val: u32)
where
    I: Iterator<Item = &'a mut Snail>,
{
    for snail in snails {
        if let Snail::Num(n) = snail {
            *n += val;
            break;
        }
    }
}

/// This should always get an array of two numbers.
fn get_until_closing_bracket(pairs: &mut Snails) -> Vec<u32> {
    let mut inner = vec![];
    while let Some(pair) = pairs.pop_front() {
        match pair {
            Snail::Num(n) => inner.push(n),
            Snail::Symbol(s) => {
                if s == ']' {
                    break;
                }
            }
        };
    }

    inner
}

fn explode(mut input: Snails) -> (Snails, bool) {
    let mut processed = VecDeque::new();
    let mut depth = 0;

    while let Some(pair) = input.pop_front() {
        match pair {
            Snail::Num(n) => processed.push_back(Snail::Num(n)),
            Snail::Symbol(bracket) => {
                if depth == 4 && bracket == '[' {
                    let inner = get_until_closing_bracket(&mut input);

                    // Check backwards through processed, and forwards through input to add exploded value.
                    increment_explosion(processed.iter_mut().rev(), inner[0]);
                    increment_explosion(input.iter_mut(), inner[1]);

                    processed.push_back(Snail::Num(0));
                    processed.extend(input.drain(0..));
                    return (processed, true);
                } else {
                    processed.push_back(pair);
                    depth += if bracket == '[' { 1 } else { -1 };
                }
            }
        }
    }
    (processed, false)
}

fn split(mut input: Snails) -> (Snails, bool) {
    let mut processed = VecDeque::new();

    while let Some(pair) = input.pop_front() {
        match pair {
            Snail::Num(n) => {
                if n > 9 {
                    processed.extend([
                        Snail::Symbol('['),
                        Snail::Num(n / 2),
                        Snail::Num(n / 2 + (n & 1)),
                        Snail::Symbol(']'),
                    ]);
                    processed.extend(input.drain(0..));

                    return (processed, true);
                } else {
                    processed.push_back(Snail::Num(n));
                }
            }
            Snail::Symbol(bracket) => processed.push_back(pair),
        }
    }
    (processed, false)
}

fn reduce(input: Snails) -> Snails {
    let (input, repeat) = explode(input);
    if repeat {
        return reduce(input);
    }

    let (input, repeat) = split(input);
    if repeat {
        return reduce(input);
    }

    input
}

fn find_match(input: &Snails, idx: usize, forwards: bool) -> usize {
    let iter: Box<dyn Iterator<Item = &Snail>> = if forwards {
        Box::new(input.iter().skip(idx))
    } else {
        Box::new(input.iter().rev().skip(input.len() - 1 - idx))
    };

    let mut count = 0;
    let increment = if forwards { 1 } else { -1 };
    let mut i = idx;

    for item in iter {
        if let Snail::Symbol(b) = item {
            if *b == '[' {
                count += increment;
            } else {
                count -= increment;
            }
        }
        if count == 0 {
            return i;
        }
        i = if forwards { i + 1 } else { i - 1 };
    }
    unreachable!("Malformed input")
}

fn count(input: &Snails, lo: usize, hi: usize) -> u32 {
    let left = if let Snail::Num(n) = input[lo + 1] {
        n
    } else {
        count(input, lo + 1, find_match(input, lo + 1, true))
    };

    let right = if let Snail::Num(n) = input[hi - 1] {
        n
    } else {
        count(input, find_match(input, hi - 1, false), hi - 1)
    };

    3 * left + 2 * right
}

fn read_line(input: &str) -> Snails {
    let mut queue = VecDeque::new();

    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            queue.push_back(Snail::Num(digit));
        } else if c == '[' || c == ']' {
            queue.push_back(Snail::Symbol(c));
        }
    }

    queue
}

fn add_pairs(mut first: Snails, second: Snails) -> Snails {
    first.push_front(Snail::Symbol('['));
    first.extend(second.into_iter());
    first.push_back(Snail::Symbol(']'));
    first
}

pub fn part1(input: &str) -> Result<u32> {
    let first = input
        .lines()
        .next()
        .ok_or_else(|| eyre!("malformed input!"))?;

    let res = input.lines().skip(1).fold(read_line(first), |prev, curr| {
        reduce(add_pairs(prev, read_line(curr)))
    });

    Ok(count(&res, 0, res.len() - 1))
}

pub fn part2(input: &str) -> Result<u32> {
    let mut max = 0;
    let lines: Vec<&str> = input.lines().collect();

    for &line in &lines {
        for &line2 in &lines {
            if line == line2 {
                continue;
            }
            let res = reduce(add_pairs(read_line(line), read_line(line2)));
            let c = count(&res, 0, res.len() - 1);
            max = std::cmp::max(max, c);
        }
    }

    Ok(max)
}

const INPUT: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
const INPUT2: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(143, part1("[[1,2],[[3,4],5]]").unwrap());
        assert_eq!(1384, part1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap());
        assert_eq!(445, part1("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(3993, part2(INPUT2).unwrap());
    }

    static FILE: &str = include_str!("../../input/21/18.txt");

    #[bench]
    fn benchmark(b: &mut Bencher) {
        b.iter(|| part2(FILE.trim()));
    }
}
