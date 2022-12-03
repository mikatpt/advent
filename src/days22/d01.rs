use crate::{get_input, Result};
use std::collections::BinaryHeap;

fn make_heap(input: &str) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::new();
    for elf in input.split("\n\n") {
        let total_calories: i32 = elf
            .lines()
            .map(|s| s.parse::<i32>())
            .map(Result::unwrap)
            .sum();

        heap.push(total_calories);
    }
    heap
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn part1(input: &str) -> Result<i32> {
    let mut heap = make_heap(input);
    let max = heap.pop().unwrap();
    Ok(max)
}

fn part2(input: &str) -> Result<i32> {
    let mut heap = make_heap(input);

    let mut amt = 0;
    for _ in 0..3 {
        amt += heap.pop().unwrap();
    }
    Ok(amt)
}

const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(24000, part1(INPUT).unwrap());
        assert_eq!(45000, part2(INPUT).unwrap());
    }
}
