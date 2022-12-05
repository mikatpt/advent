use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(4)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

/*
assignments overlap.

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
2-6,4-8
4-6,6-6

if a <= c and b <= d, overlapping.

elf 1 is in 2-4, elf 2 is in 6-8

how many assignments has one range fully containing the other?

2-4,6-8

1-5,2-6
5-10,4-7

a <= c && b >= c

a <= c && b <= d

*/

type Elves = ((i32, i32), (i32, i32));

fn get_ranges(input: &str) -> Option<Vec<Elves>> {
    let mut ranges = vec![];

    for line in input.lines() {
        let (elf1, elf2) = line.split_once(',')?;
        let mut iter = elf1.split('-').map(str::parse::<i32>).map(Result::unwrap);
        let (elf1_min, elf1_max) = (iter.next()?, iter.next()?);

        let mut iter = elf2.split('-').map(str::parse::<i32>).map(Result::unwrap);
        let (elf2_min, elf2_max) = (iter.next()?, iter.next()?);

        ranges.push(((elf1_min, elf1_max), (elf2_min, elf2_max)));
    }

    Some(ranges)
}

fn part1(input: &str) -> Option<i32> {
    let mut fully_contained = 0;
    let ranges = get_ranges(input)?;
    for ((elf1_min, elf1_max), (elf2_min, elf2_max)) in ranges {
        if (elf1_min >= elf2_min && elf1_max <= elf2_max)
            || (elf2_min >= elf1_min && elf2_max <= elf1_max)
        {
            fully_contained += 1;
        }
    }

    Some(fully_contained)
}

fn part2(input: &str) -> Option<i32> {
    let mut overlapping = 0;
    let ranges = get_ranges(input)?;

    for ((elf1_min, elf1_max), (elf2_min, elf2_max)) in ranges {
        if (elf1_min <= elf2_min && elf1_max >= elf2_min)
            || (elf2_min <= elf1_min && elf2_max >= elf1_min)
        {
            overlapping += 1
        }
    }

    Some(overlapping)
}

const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(2, part1(INPUT).unwrap());
        assert_eq!(4, part2(INPUT).unwrap());
    }
}
