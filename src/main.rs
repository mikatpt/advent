use std::{env, fs};
use tracing::{info, instrument};

use advent::days21::d7::{part1, part2};
use advent::Result;

fn main() -> Result<()> {
    advent::tracing::init();

    println!("Answer to part 1 is {}", part1(&read_file()?)?);
    println!("Answer to part 2 is {}", part2(&read_file()?)?);

    Ok(())
}

#[instrument]
fn read_file() -> Result<String> {
    info!("Reading file from args");
    let filename = env::args().nth(1).expect("Couldn't read file!");

    let input = fs::read_to_string(filename)?;
    Ok(input.trim().to_string())
}
