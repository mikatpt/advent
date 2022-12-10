use crate::{get_input, Result};
use std::collections::HashSet;

pub fn solve() -> Result<(usize, usize)> {
    let input = get_input(9)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Option<usize> {
    solution(input, 2)
}

fn part2(input: &str) -> Option<usize> {
    solution(input, 10)
}

fn solution(input: &str, size: usize) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); size];
    visited.insert((0, 0));

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ')?;
        let steps = steps.parse::<isize>().expect("should be num");

        for j in 0..steps {
            knots[0] = next_head_pos(knots[0].0, knots[0].1, dir);

            let mut i = 1;
            while i < size {
                let (x, y) = (knots[i - 1].0, knots[i - 1].1);

                // if a intermediary knot is adjacent, the rest won't move.
                if is_adjacent(x, y, knots[i].0, knots[i].1) {
                    break;
                }
                knots[i] = next_tail_pos(x, y, knots[i].0, knots[i].1);
                i += 1;
            }

            // the tail
            if i == size {
                visited.insert(knots[size - 1]);
            }
        }
    }

    Some(visited.len())
}

/// head (x, y), tail (x2,y2)
fn is_adjacent(x: isize, y: isize, x2: isize, y2: isize) -> bool {
    let y_match = y == y2 - 1 || y == y2 || y == y2 + 1;
    let x_match = x == x2 - 1 || x == x2 || x == x2 + 1;
    x_match && y_match
}

fn next_head_pos(x: isize, y: isize, dir: &str) -> (isize, isize) {
    match dir {
        "U" => (x, y + 1),
        "D" => (x, y - 1),
        "L" => (x - 1, y),
        "R" => (x + 1, y),
        _ => unreachable!("only four dirs"),
    }
}

/// head (x, y), tail (x2,y2)
fn next_tail_pos(x: isize, y: isize, x2: isize, y2: isize) -> (isize, isize) {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let (mut xres, mut yres) = (x2, y2);
    xres += match x2.cmp(&x) {
        Less => 1,
        Equal => 0,
        Greater => -1,
    };
    yres += match y2.cmp(&y) {
        Less => 1,
        Equal => 0,
        Greater => -1,
    };

    (xres, yres)
}

const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        crate::trace::init();
        assert_eq!(13, part1(INPUT).unwrap());
        assert_eq!(1, part2(INPUT).unwrap());
        assert_eq!(36, part2(INPUT2).unwrap());
    }
}
