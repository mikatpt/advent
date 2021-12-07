use crate::Result;
/*
Day 6: Lanternfish

Each fish makes a new fish every 7 days. This isn't synced, however.

Fish timer = 3:
    - after the 4th iteration (2, 1, 0, HERE), it creates a new fish with timer = 8.
    - It will reset to a timer of 6
Find total fish after 80 days.

0: map{
    0: 1,
    1: 2,
    2: 3,
    3: 4,
    4: 5,
    5: 6,
    6: 7,
    7: 8,
    8: 9,
}

1. let item = map[0] (1)
2.   map[0] = map[1]
3.   map[1] = map[2]
4.   map[2] = map[3]
5.   map[3] = map[4]
6.   map[4] = map[5]
7.   map[5] = map[6]
8.   map[6] = map[7] + item
9.   map[7] = map[8]
10.  map[8] = item

*/

pub fn part1(input: &str) -> Result<u64> {
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
        fish.extend(newborns.into_iter());
    }
    Ok(fish.len() as u64)
}

use std::collections::HashMap;

pub fn part2(input: &str) -> Result<u128> {
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

pub fn solve(input: &str) -> Result<u64> {
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
        let output = part1(INPUT).unwrap();

        assert_eq!(5934, output);
    }

    #[test]
    fn test2() {
        let output = part2(INPUT).unwrap();

        assert_eq!(26984457539, output);
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    #[test]
    fn test3() {
        let output = solve(INPUT).unwrap();

        assert_eq!(26984457539, output);
    }
}
