use crate::Result;

pub fn part1(input: &str) -> Result<i32> {
    Ok(0)
}

pub fn part2(input: &str) -> Result<i32> {
    Ok(0)
}

const INPUT: &str = "";

#[cfg(test)]
mod tests {
    use super::*;
    use eyre_test::test;

    #[test]
    fn test1() {
        assert_eq!(0, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(0, part2(INPUT).unwrap());
    }
}
