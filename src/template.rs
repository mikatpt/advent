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
    use crate::tracing::init;
    use test::Bencher;

    #[test]
    fn test1() {
        init();

        assert_eq!(0, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        init();

        assert_eq!(0, part2(INPUT).unwrap());
    }

    #[bench]
    fn bench1(b: &mut Bencher) {
        init();

        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        init();

        b.iter(|| part2(INPUT));
    }
}
