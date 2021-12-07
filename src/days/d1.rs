/*
sonar sweep
read line by line

How quickly does depth increase?
    - Count # of times depth measurement increases from prev measurement

199 (N/A - no previous measurement)
200 (increased)
208 (increased)
210 (increased)
200 (decreased)
207 (increased)
240 (increased)
269 (increased)
260 (decreased)
263 (increased)

output = 7

follow-up sliding window.

199  A
200  A B
208  A B C          607
210    B C D        618
200  E   C D        618
207  E F   D
240  E F G
269    F G H
260      G H
263        H


first  = 199 + 200 + 208 (0, 1, 2)
second = 200 + 208 + 210 (1, 2, 3) = first - d[0] + d[3]
third  = second - d[1] + d[4];

*/
use crate::Result;

pub fn part1(input: &str) -> Result<i32> {
    let mut res = 0;

    let mut lines = input.split_whitespace();
    let mut first: i32 = lines.next().unwrap().parse::<i32>()?;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let second: i32 = line.parse::<i32>()?;

        if second > first {
            res += 1;
        }

        first = second;
    }

    Ok(res)
}

pub fn part2(input: &str) -> Result<i32> {
    let mut res = 0;

    let lines: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut first = 0;

    for line in lines.iter().take(3) {
        first += line;
    }

    for (i, line) in lines.iter().enumerate().skip(3) {
        println!("{:?}", first);
        let second = first - lines[i - 3] + line;

        if second > first {
            res += 1;
        }
        first = second;
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = " 199 200 208 210 200 207 240 269 260 263 ";
        let output = part1(input).unwrap();

        assert_eq!(7, output);
    }

    #[test]
    fn example2() {
        let input = " 199 200 208 210 200 207 240 269 260 263 ";
        let output = part2(input).unwrap();

        assert_eq!(5, output);
    }
}
