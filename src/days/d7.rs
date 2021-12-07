use crate::Result;
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

fn solve(input: &str, pt1: bool) -> Result<i32> {
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

pub fn part1(input: &str) -> Result<i32> {
    solve(input, true)
}

fn get_fuel(i: i32, pt1: bool) -> i32 {
    if pt1 {
        return i;
    }
    i * (i + 1) / 2
}

pub fn part2(input: &str) -> Result<i32> {
    solve(input, false)
}

const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        let output = part1(INPUT).unwrap();

        assert_eq!(37, output);
    }

    #[test]
    fn test2() {
        let output = part2(INPUT).unwrap();

        assert_eq!(168, output);
    }

    #[bench]
    fn bench1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    static I: &str = include_str!("../../input/7.txt");
    #[bench]
    fn bench2(b: &mut Bencher) {
        b.iter(|| part2(I.trim()));
    }
}
