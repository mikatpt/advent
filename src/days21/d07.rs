use crate::{get_input, Result};

fn solve1(input: &str, pt1: bool) -> color_eyre::Result<i32> {
    let mut res = i32::MAX;
    let nums: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect();
    let max = *nums.iter().max().unwrap();

    for i in 0..=max {
        let mut new_res = 0;
        for j in nums.iter() {
            new_res += get_fuel(i32::abs(i - j), pt1);
        }
        res = std::cmp::min(res, new_res);
    }

    Ok(res)
}

fn get_fuel(i: i32, pt1: bool) -> i32 {
    if pt1 {
        return i;
    }
    i * (i + 1) / 2
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn part1(input: &str) -> Result<i32> {
    solve1(input, true)
}

fn part2(input: &str) -> Result<i32> {
    solve1(input, false)
}

const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(37, part1(INPUT).unwrap());
        assert_eq!(168, part2(INPUT).unwrap());
    }

    static FILE: &str = include_str!("../../input/21/07.txt");
    #[bench]
    fn bench1(b: &mut Bencher) {
        b.iter(|| part1(FILE.trim()));
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        b.iter(|| part2(FILE.trim()));
    }
}
