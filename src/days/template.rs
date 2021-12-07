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
    use test::Bencher;

    #[test]
    fn test1() {
        let output = part1(INPUT).unwrap();

        assert_eq!(0, output);
    }

    #[test]
    fn test2() {
        let output = part2(INPUT).unwrap();

        assert_eq!(0, output);
    }

    #[bench]
    fn bench1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }
}
