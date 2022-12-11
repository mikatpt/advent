use itertools::Itertools;
use std::collections::VecDeque;

use crate::{get_input, Result};

pub fn solve() -> Result<(i64, i64)> {
    let input = get_input(11)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Option<i64> {
    solution(input, 20)
}

fn part2(input: &str) -> Option<i64> {
    solution(input, 10000)
}

fn solution(input: &str, rounds: i32) -> Option<i64> {
    let mut monkeys = parse(input)?;
    let n = monkeys.len();
    let modulus: i64 = monkeys.iter().map(|m| m.divisor).product();

    for round in 0..rounds {
        for i in 0..n {
            while let Some(item) = monkeys[i].items.pop_front() {
                let mut relief = (monkeys[i].operation)(item);
                monkeys[i].inspections += 1;
                if rounds == 20 {
                    relief /= 3;
                }
                relief %= modulus;
                let next_monkey = (monkeys[i].test)(relief);

                monkeys[next_monkey].items.push_back(relief);
            }
        }
    }
    let res = monkeys
        .into_iter()
        .map(|m| m.inspections as i64)
        .sorted()
        .rev()
        .take(2)
        .product();
    Some(res)
}

fn parse(input: &str) -> Option<Vec<Monkey>> {
    let mut res = vec![];

    for monkey in input.split("\n\n") {
        let mut details = monkey.lines().skip(1);
        let items: VecDeque<_> = details.next()?.trim()["Starting items: ".len()..]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let (op, num) = details.next()?.trim()["Operation: new = old ".len()..].split_once(' ')?;
        let mut number = 0;
        if num.chars().all(|c| c.is_numeric()) {
            number = num.parse().unwrap();
        }

        let operation: Box<dyn Fn(i64) -> i64> = match (op, number) {
            ("*", 0) => Box::new(|old: i64| old * old),
            ("*", n) => Box::new(move |old: i64| old * n),
            ("+", 0) => Box::new(|old: i64| old + old),
            ("+", n) => Box::new(move |old: i64| old + n),
            _ => unreachable!(),
        };

        let divisor = details.next()?.trim()["Test: divisible by ".len()..]
            .parse()
            .unwrap();

        let monkey1 = details.next()?.trim()["If true: throw to monkey ".len()..]
            .parse()
            .unwrap();
        let monkey2 = details.next()?.trim()["If false: throw to monkey ".len()..]
            .parse()
            .unwrap();

        let test = Box::new(move |item: i64| {
            if item % divisor == 0 {
                monkey1
            } else {
                monkey2
            }
        });

        res.push(Monkey {
            items,
            operation,
            test,
            divisor,
            inspections: 0,
        });
    }

    Some(res)
}

struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> usize>,
    divisor: i64,
    inspections: usize,
}

const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        crate::trace::init();
        assert_eq!(10605, part1(INPUT).unwrap());
        assert_eq!(2713310158, part2(INPUT).unwrap());
    }
}
