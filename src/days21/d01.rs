use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn part1(input: &str) -> Result<i32> {
    let mut res = 0;

    let mut lines = input.split_whitespace();
    let mut first: i32 = lines.next().unwrap().parse::<i32>()?;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let second: i32 = line.parse::<i32>()?;

        if second > first {
            res += 1;
        }

        first = second;
    }

    Ok(res)
}

fn part2(input: &str) -> Result<i32> {
    let mut res = 0;

    let lines: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut first = 0;

    for line in lines.iter().take(3) {
        first += line;
    }

    for (i, line) in lines.iter().enumerate().skip(3) {
        let second = first - lines[i - 3] + line;

        if second > first {
            res += 1;
        }
        first = second;
    }

    Ok(res)
}

const INPUT: &str = " 199 200 208 210 200 207 240 269 260 263 ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(7, part1(INPUT).unwrap());
        assert_eq!(5, part2(INPUT).unwrap());
    }
}
