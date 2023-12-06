use crate::{
    eyre, {get_input, Result},
};
use std::collections::HashMap;

struct Board {
    map: HashMap<u16, (usize, usize, bool)>,
    rows: [u16; 5],
    cols: [u16; 5],
    won: bool,
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<u16> {
    let mut lines = input.lines();
    let rand_ints = lines
        .next()
        .ok_or_else(|| eyre!("Empty!"))?
        .split(',')
        .map(|c| c.parse::<u16>().unwrap());
    lines.next();

    let game = populate_boards(&mut lines);

    solve1(game, rand_ints, true)
}

fn part2(input: &str) -> Result<u16> {
    let mut lines = input.lines();
    let rand_ints = lines
        .next()
        .ok_or_else(|| eyre!("Empty!"))?
        .split(',')
        .map(|c| c.parse::<u16>().unwrap());
    lines.next();

    let game = populate_boards(&mut lines);

    solve1(game, rand_ints, false)
}

fn populate_boards(lines: &mut core::str::Lines) -> Vec<Board> {
    let mut game: Vec<Board> = Vec::new();

    while let Some(mut line) = lines.next() {
        let mut board = Board {
            map: HashMap::new(),
            rows: [0; 5],
            cols: [0; 5],
            won: false,
        };

        for i in 0..5 {
            let row = line
                .split_whitespace()
                .enumerate()
                .map(|(j, c)| (c.parse::<u16>().unwrap(), (i, j, false)));

            for (j, (k, v)) in row.enumerate() {
                board.map.insert(k, v);
            }

            line = lines.next().unwrap_or("");
        }
        game.push(board);
    }

    game
}

fn calc_sum(i: u16, map: &HashMap<u16, (usize, usize, bool)>) -> Result<u16> {
    let sum: u16 = map.iter().fold(0, |mut prev, (&key, &(x, y, marked))| {
        if !marked {
            prev += key;
        }
        prev
    });
    Ok(sum * i)
}

fn solve1<I>(mut game: Vec<Board>, rand_ints: I, pt_1: bool) -> Result<u16>
where
    I: Iterator<Item = u16>,
{
    let mut winner: (u16, HashMap<u16, (usize, usize, bool)>) = (0, HashMap::new());

    for i in rand_ints {
        for board in &mut game {
            if !pt_1 && board.won {
                continue;
            }

            if let Some((x, y, marked)) = board.map.get_mut(&i) {
                *marked = true;
                board.rows[*x] += 1;
                board.cols[*y] += 1;
                if board.rows[*x] == 5 || board.cols[*y] == 5 {
                    // winner found.
                    if pt_1 {
                        return calc_sum(i, &board.map);
                    }
                    board.won = true;
                    winner = (i, board.map.clone());
                }
            }
        }
    }

    calc_sum(winner.0, &winner.1)
}

const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4512, part1(INPUT).unwrap());
        assert_eq!(1924, part2(INPUT).unwrap());
    }
}
