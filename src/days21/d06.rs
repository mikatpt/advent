use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<u64> {
    let mut fish: Vec<u8> = input
        .split(',')
        .map(|c| c.parse().expect("Not a u8!"))
        .collect();

    for i in 0..80 {
        let mut newborns: Vec<u8> = vec![];
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 7;
                newborns.push(8);
            }
            *f -= 1;
        }
        fish.extend(newborns);
    }
    Ok(fish.len() as u64)
}

use std::collections::HashMap;

fn part2(input: &str) -> Result<u128> {
    let mut fish: HashMap<u8, u128> = input.split(',').fold(HashMap::new(), |mut map, char| {
        let n = char.parse().expect("Not a u8!");
        *map.entry(n).or_default() += 1;
        map
    });

    for _ in 0..256 {
        let new_fish = *fish.entry(0).or_default();
        for j in 0..6 {
            *fish.entry(j).or_default() = *fish.entry(j + 1).or_default();
        }
        *fish.entry(6).or_default() = *fish.entry(7).or_default() + new_fish;
        *fish.entry(7).or_default() = *fish.entry(8).or_default();
        *fish.entry(8).or_default() = new_fish;
    }

    let res = fish.into_values().sum();

    Ok(res)
}

fn solve1(input: &str) -> Result<u64> {
    let mut fish: [u64; 9] = [0; 9];
    input
        .split(',')
        .for_each(|c| fish[c.parse::<usize>().expect("NaN")] += 1);

    for _ in 0..256 {
        let next_fish = fish[0];
        for j in 0..8 {
            fish[j] = fish[j + 1];
        }
        fish[6] += next_fish;
        fish[8] = next_fish;
    }
    Ok(fish.into_iter().sum())
}

const INPUT: &str = "3,4,3,1,2";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(5934, part1(INPUT).unwrap());
        assert_eq!(26984457539, part2(INPUT).unwrap());
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }
}
