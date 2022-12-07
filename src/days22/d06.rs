use crate::{get_input, Result};

pub fn solve() -> Result<(usize, usize)> {
    let input = get_input(6)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

use std::collections::{HashMap, VecDeque};

fn solution(input: &str, limit: usize) -> Option<usize> {
    let mut map = HashMap::new();
    let mut window = VecDeque::new();
    let mut doubles = 0;

    for (i, c) in input.char_indices() {
        if i > limit - 1 {
            let to_remove = window.pop_front()?;
            if let Some(item) = map.get_mut(&to_remove) {
                *item -= 1;
                if *item == 0 {
                    map.remove(&to_remove);
                } else if *item == 1 {
                    doubles -= 1;
                }
            }
        }

        window.push_back(c);

        let entry = map.entry(c).or_insert(0);
        *entry += 1;
        if *entry == 2 {
            doubles += 1;
        }

        if i > 3 && doubles == 0 && map.contains_key(&c) {
            return Some(i + 1);
        }
    }

    None
}

fn part1(input: &str) -> Option<usize> {
    solution(input, 4)
}

fn part2(input: &str) -> Option<usize> {
    solution(input, 14)
}

const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(7, part1(INPUT1).unwrap());
        assert_eq!(5, part1(INPUT2).unwrap());
        assert_eq!(6, part1(INPUT3).unwrap());
        assert_eq!(10, part1(INPUT4).unwrap());
        assert_eq!(11, part1(INPUT5).unwrap());

        assert_eq!(19, part2(INPUT1).unwrap());
        assert_eq!(23, part2(INPUT2).unwrap());
        assert_eq!(23, part2(INPUT3).unwrap());
        assert_eq!(29, part2(INPUT4).unwrap());
        assert_eq!(26, part2(INPUT5).unwrap());
    }
}
