use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)?, part2(&input)?))
}

fn part1(input: &str) -> Result<i32> {
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

fn part2(input: &str) -> Result<i32> {
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

    #[test]
    fn test1() {
        assert_eq!(150, part1(INPUT).unwrap());
        assert_eq!(900, part2(INPUT).unwrap());
    }
}
