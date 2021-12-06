#![allow(dead_code, unused_variables)]
mod template;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

use std::{env, fs};

use d5::{part1, part2};

fn main() -> color_eyre::Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let filename = env::args().nth(1).ok_or("Couldn't read file!")?;

    let input = fs::read_to_string(filename)?;
    let input = input.trim();

    let output = part1(input)?;
    let output2 = part2(input)?;

    println!("Answer to part 1 is {}", output);
    println!("Answer to part 2 is {}", output2);
    Ok(())
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
