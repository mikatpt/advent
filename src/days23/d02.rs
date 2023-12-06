use crate::{get_input, Result};
/*
cubes are red,green,blue

pull random cubes, then put back.
which games are possible if bag had only 12 red, 13 green, 14 blue?

in example, 1, 2, 5.
3 is impossible because 20 red shown at once.
4 is impossible because 15 blue shown.

add up id's of possible games.
*/

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(2)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

#[derive(Default, Debug)]
struct Game {
    games: Vec<Draw>,
}

#[derive(Default, Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn is_possible(&self) -> bool {
        for draw in &self.games {
            if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
                return false;
            }
        }
        true
    }

    fn find_min(&self) -> i32 {
        use std::cmp::max;
        let (mut blue, mut red, mut green) = (0, 0, 0);
        for draw in &self.games {
            blue = max(blue, draw.blue);
            green = max(green, draw.green);
            red = max(red, draw.red);
        }
        blue * red * green
    }
}

impl<'a> FromIterator<&'a str> for Game {
    fn from_iter<T: IntoIterator<Item = &'a str>>(draws: T) -> Self {
        let games = draws
            .into_iter()
            .map(|draw| {
                let mut d = Draw::default();
                for set in draw.split(", ") {
                    let (amt, color) = set.split_once(' ').unwrap();
                    let amt = amt.parse().expect("is num");
                    match color {
                        "red" => d.red = amt,
                        "green" => d.green = amt,
                        "blue" => d.blue = amt,
                        _ => {}
                    }
                }
                d
            })
            .collect();
        Self { games }
    }
}

fn get_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (_, game) = line.split_once(": ").unwrap();
            game.split("; ").collect()
        })
        .collect()
}

fn part1(input: &str) -> Result<i32> {
    Ok(get_games(input)
        .iter()
        .enumerate()
        .fold(0, |mut prev, (idx, game)| {
            if game.is_possible() {
                prev += idx as i32 + 1;
            }
            prev
        }))
}

fn part2(input: &str) -> Result<i32> {
    Ok(get_games(input)
        .iter()
        .fold(0, |prev, acc| prev + acc.find_min()))
}

const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(8, part1(INPUT).unwrap());
        assert_eq!(2286, part2(INPUT).unwrap());
    }
}
