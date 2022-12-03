use crate::{
    eyre, {get_input, Result},
};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(s: &str) -> Result<u32> {
    // length of binary number
    let cols = s.lines().next().ok_or_else(|| eyre!("Empty input"))?.len();

    // # of inputs
    let rows = s.lines().count();

    // Iterate through columns. Index to find correct digit, check if it == 1, then count 1's.
    let ones = (0..cols).map(|i| s.lines().filter(|line| line.as_bytes()[i] == b'1').count());

    // Reduce ones into gamma and epsilon using bit addition.
    // if count_1 > count_0, g + 1, e + 0
    // else g + 0, e + 1
    let (gamma, epsilon) = ones.fold((0, 0), |(g, e), val| {
        if val * 2 > rows {
            ((g << 1) | 1, e << 1)
        } else {
            (g << 1, (e << 1) | 1)
        }
    });

    Ok(gamma * epsilon)
}

fn part2(s: &str) -> Result<u32> {
    let rating = |common: bool| -> Result<u32> {
        let mut lines: Vec<_> = s.lines().collect();

        let mut i = 0;
        while lines.len() > 1 {
            let ones = lines
                .iter()
                .filter(|line| line.as_bytes()[i] == b'1')
                .count();
            let bit = match (common, ones * 2 >= lines.len()) {
                (true, true) | (false, false) => b'1',
                _ => b'0',
            };

            lines = lines
                .into_iter()
                .filter(|line| line.as_bytes()[i] == bit)
                .collect();
            i += 1;
        }

        let res = u32::from_str_radix(lines.first().ok_or_else(|| eyre!("empty"))?, 2)?;

        Ok(res)
    };

    let (o2_rating, co2_rating) = (rating(true)?, rating(false)?);
    Ok(o2_rating * co2_rating)
}

const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(198, part1(INPUT).unwrap());
        assert_eq!(230, part2(INPUT).unwrap());
    }
}
