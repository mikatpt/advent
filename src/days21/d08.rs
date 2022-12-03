use crate::{get_input, Result};

fn count_unique(count: i32, digit: &str) -> i32 {
    count
        + match digit.chars().count() {
            2..=4 | 7 => 1,
            _ => 0,
        }
}

fn combine_counts(count: i32, (_, digits): (&str, &str)) -> i32 {
    count + digits.split_whitespace().fold(0, count_unique)
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn part1(input: &str) -> Result<i32> {
    let count = input
        .lines()
        .map(|line| line.split_once(" | ").expect("malformed input"))
        .fold(0, combine_counts);

    Ok(count)
}

/// Find signal of len l and count how many characters match in encrypted str.
fn find_matches(encrypted: &str, signals: &str, l: usize) -> i32 {
    let signal = signals.split_whitespace().find(|s| s.len() == l).unwrap();
    let mut res = 0;

    for char in signal.chars() {
        if encrypted.contains(char) {
            res += 1;
        }
    }
    res
}

fn decrypt_digit(digit: &str, signals: &str) -> char {
    let matches_one = find_matches(digit, signals, 2);
    let matches_four = find_matches(digit, signals, 4);

    match (digit.len(), matches_one, matches_four) {
        (6, 2, 3) => '0',
        (2, _, _) => '1',
        (5, 1, 2) => '2',
        (5, 2, 3) => '3',
        (4, _, _) => '4',
        (5, 1, 3) => '5',
        (6, 1, 3) => '6',
        (3, _, _) => '7',
        (7, _, _) => '8',
        (6, 2, 4) => '9',
        _ => unreachable!(),
    }
}

fn decode_line(signals: &str, digits: &str) -> i32 {
    let mut num = String::new();

    for digit in digits.split_whitespace() {
        let d = decrypt_digit(digit, signals);
        num.push(d)
    }

    num.parse().unwrap()
}

fn part2(input: &str) -> Result<i32> {
    let mut res = 0;
    for line in input.lines() {
        let (signals, digits) = line.split_once(" | ").expect("malformed input");
        res += decode_line(signals, digits);
    }
    Ok(res)
}

const INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(26, part1(INPUT).unwrap());
        assert_eq!(61229, part2(INPUT).unwrap());
    }
}
