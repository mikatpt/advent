use crate::Result;
use std::collections::HashMap;

fn get_info() -> (HashMap<char, char>, HashMap<char, u32>, HashMap<char, u32>) {
    (
        HashMap::from([('{', '}'), ('[', ']'), ('<', '>'), ('(', ')')]),
        HashMap::from([('}', 1197), (']', 57), ('>', 25137), (')', 3)]),
        HashMap::from([('}', 3), (']', 2), ('>', 4), (')', 1)]),
    )
}

pub fn part1(input: &str) -> Result<u32> {
    let mut score = 0;
    let mut stack = Vec::<char>::new();
    let (closers, illegal, _) = get_info();

    for line in input.lines() {
        let mut s = 0;
        let mut corrupted = false;

        for char in line.chars() {
            if let Some(closer) = closers.get(&char) {
                stack.push(*closer);
            } else if let Some(opener) = stack.pop() {
                if opener != char {
                    corrupted = true;
                    s += illegal.get(&char).unwrap();
                }
            }
        }

        let last = line.chars().last().unwrap();
        if !corrupted && closers.contains_key(&last) {
            continue;
        }
        score += s;
    }

    Ok(score)
}

pub fn part2(input: &str) -> Result<u64> {
    let mut res = Vec::<u64>::new();
    let mut stack = Vec::<char>::new();
    let (closers, _, incomplete) = get_info();

    'outer: for line in input.lines() {
        for char in line.chars() {
            if let Some(closer) = closers.get(&char) {
                stack.push(*closer);
            } else if let Some(opener) = stack.pop() {
                if opener != char {
                    stack.drain(0..);
                    continue 'outer;
                }
            }
        }

        let mut score: u64 = 0;
        while let Some(item) = stack.pop() {
            score = score * 5 + *incomplete.get(&item).unwrap() as u64;
        }
        res.push(score);
    }
    res.sort_unstable();
    Ok(*res.get(res.len() / 2).unwrap())
}

const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(26397, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(288957, part2(INPUT).unwrap());
    }
}
