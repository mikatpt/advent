use crate::{get_input, Result};
use std::collections::VecDeque;

pub fn solve() -> Result<(String, String)> {
    let input = get_input(5)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

// amount, from, to
type Action = (usize, usize, usize);
type Input = (Vec<VecDeque<char>>, Vec<Action>);

fn parse(input: &str) -> Option<Input> {
    // we know how many stacks there are based on the amount of whitespace.
    let num_stacks = (input.lines().next()?.len() + 1) / 4;
    let mut stacks = Vec::from_iter((0..num_stacks).map(|_| VecDeque::new()));

    let mut lines = input.lines();
    while let Some(line) = lines.next() && !line.trim().chars().next()?.is_numeric() {
        for (idx, char) in line.char_indices().skip(1) {
            if char.is_alphabetic() {
                // int division is enough to find the right stack here (see above)
                stacks[idx / 4].push_front(char);
            }
        }
    }

    let mut actions = vec![];
    lines.next(); // empty line

    for line in lines {
        let mut nums = line
            .split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_numeric())) // skip "move", "from", "to"
            .map(str::parse::<usize>)
            .map(Result::unwrap);

        let action = (
            nums.next()?,     // amount
            nums.next()? - 1, // from
            nums.next()? - 1, // to
        );
        actions.push(action);
    }

    Some((stacks, actions))
}

fn solution(input: &str, part: i32) -> Option<String> {
    let (mut stacks, actions) = parse(input)?;

    for (amount, from, to) in actions {
        let mut middleman = VecDeque::new();
        for _ in 0..amount {
            let item = stacks[from].pop_back()?;
            if part == 1 {
                middleman.push_back(item);
            } else {
                middleman.push_front(item);
            }
        }
        stacks[to].extend(middleman.into_iter());
    }
    let mut res = String::new();

    for stack in stacks.iter() {
        if let Some(item) = stack.back() {
            res.push(*item);
        }
    }

    Some(res)
}

fn part1(input: &str) -> Option<String> {
    solution(input, 1)
}

fn part2(input: &str) -> Option<String> {
    solution(input, 2)
}

const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!("CMZ".to_string(), part1(INPUT).unwrap());
        assert_eq!("MCD".to_string(), part2(INPUT).unwrap());
    }
}
