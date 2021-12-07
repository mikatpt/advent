#![allow(dead_code, unused_variables)]
#![feature(test)]

extern crate test;

mod days;
use std::{env, fs};

use days::d6::{part1, part2};

fn main() -> color_eyre::Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let filename = env::args().nth(1).expect("Couldn't read file!");

    let input = fs::read_to_string(filename)?;
    let input = input.trim();

    let output = part1(input)?;
    let output2 = part2(input)?;

    println!("Answer to part 1 is {}", output);
    println!("Answer to part 2 is {}", output2);
    Ok(())
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
