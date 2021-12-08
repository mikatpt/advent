/*
Day 3: diagnostics report.

Find gamma rate and epsilon rate for each binary number.
Power consumption = gamma * epsilon


Just ran through the below example, seems like this is how you do bit addition.
To add a one, you run  (x << 1) | 1
To add a zero, you run (x << 1)

0     << 1 = 10    | 1 = 11     (3)
11    << 1 = 110                (6)
11    << 1 = 1100  | 1 = 1101   (11)
1101  << 1 = 11010 | 1 = 11011  (16)
11011 << 1 = 110100             (22)

AND:            a * b
    0 & 1 = 0
    0 & 0 = 0
    1 & 1 = 1
OR:
    0 | 1 = 1
    0 | 0 = 0
    1 | 1 = 1
XOR:
    a^b == b^a
    a^a == 0

    0 ^ 1 = 1
    0 ^ 0 = 0
    1 ^ 1 = 0
NOT:
    !0 = 1, !1 = 0
LSHIFT:
    001 << 1 = 010
RSHIFT:
    100 >> 1 = 010
*/

use crate::{eyre, Result};
use tracing::instrument;

#[instrument(skip_all)]
pub fn part1(s: &str) -> Result<u32> {
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

/*
Find oxygen generator rating and C02 scrubber rating.

1. Iterate through list of binary numbers. Consider only the first bit.
    - Keep number if it passes bit criteria. Otherwise, discard.
    - If you have only one number left, this is the value you are searching for.
    - Otherwise, repeat for the next bit.

CRITERIA (oxygen):
    - Find most common value (0 or 1) for current bit. Keep only # with that bit.
        - If count_0 == count_1, keep 0 values.

CRITERIA (c02):
    - Find least common value, if equal keep 0's.

*/

#[instrument(skip_all)]
pub fn part2(s: &str) -> Result<u32> {
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
    use crate::tracing::init;

    #[test]
    fn test() {
        init();

        assert_eq!(198, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        init();

        assert_eq!(230, part2(INPUT).unwrap());
    }
}
