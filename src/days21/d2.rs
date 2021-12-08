/*
Day 2: Diving.

Sub can take commands forward x, down x, up x
    - x can be any positive integer

Sub starts at (0, 0)

return x * y
*/

use crate::Result;
use tracing::instrument;

#[instrument(skip_all)]
pub fn part1(input: &str) -> Result<i32> {
    let (mut x, mut y) = (0, 0);

    for line in input.split_terminator('\n') {
        let (dir, count) = line.split_once(' ').unwrap();

        let count = count.parse::<i32>()?;

        match dir {
            "forward" => x += count,
            "down" => y += count,
            "up" => y -= count,
            _ => unreachable!(),
        }
    }

    Ok(x * y)
}

/*
also track 3rd value, 'aim' - also starts at 0.

- down x increases aim by x
- up x decreases aim by x
- forward x increases horizontal pos by x, and depth by aim * x
*/
#[instrument(skip_all)]
pub fn part2(input: &str) -> Result<i32> {
    let (mut x, mut y, mut aim) = (0, 0, 0);

    for line in input.split_terminator('\n') {
        let (dir, count) = line.split_once(' ').unwrap();

        let count = count.parse::<i32>()?;

        match dir {
            "forward" => {
                x += count;
                y += aim * count;
            }
            "down" => aim += count,
            "up" => aim -= count,
            _ => unreachable!(),
        }
    }

    Ok(x * y)
}

const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::init;

    #[test]
    fn test() {
        init();

        assert_eq!(150, part1(INPUT).unwrap());
    }

    #[test]
    fn test2() {
        init();

        assert_eq!(900, part2(INPUT).unwrap());
    }
}
