use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(19)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Result<i32> {
    Ok(0)
}

fn part2(input: &str) -> Result<i32> {
    Ok(0)
}

const INPUT: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(0, part1(INPUT).unwrap());
        assert_eq!(0, part2(INPUT).unwrap());
    }
}
