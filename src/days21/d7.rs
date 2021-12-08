/*
Day 7: Treachery of Whales

A whale wants to eat you. But some crabs come to rescue you!

Crabs have horizontal positioning as such:
16,1,2,0,4,2,7,1,2,14

Each step change will cost 1 fuel. Align all crabs with the minimal amount of fuel.

Naively, O(n**2):
    res
    for i in vec:
        new_res = 0
        for j in vec:
            new_res += abs(j - i);
        res = min(res, new_res)
*/
use crate::Result;
use tracing::instrument;

#[instrument(skip_all)]
fn solve(input: &str, pt1: bool) -> color_eyre::Result<i32> {
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

#[instrument(skip_all)]
pub fn part1(input: &str) -> Result<i32> {
    solve(input, true)
}

fn get_fuel(i: i32, pt1: bool) -> i32 {
    if pt1 {
        return i;
    }
    i * (i + 1) / 2
}

#[instrument(skip_all)]
pub fn part2(input: &str) -> Result<i32> {
    solve(input, false)
}

const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::init;
    use test::Bencher;

    #[test]
    fn test1() {
        init();

        assert_eq!(37, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        init();

        assert_eq!(168, part2(INPUT).unwrap());
    }

    static FILE: &str = include_str!("../../input/21/7.txt");
    #[bench]
    fn bench1(b: &mut Bencher) {
        init();

        b.iter(|| part1(FILE.trim()));
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        init();

        b.iter(|| part2(FILE.trim()));
    }
}
