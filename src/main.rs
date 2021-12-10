use std::{env, fs};
use tracing::{info, instrument};

use advent::days21::d10::{part1, part2};
use advent::Result;

fn main() -> Result<()> {
    advent::tracing::init();

    let input = read_file()?;

    println!("Answer to part 1 is {}", part1(&input)?);
    println!("Answer to part 2 is {}", part2(&input)?);

    Ok(())
}

#[instrument]
fn read_file() -> Result<String> {
    info!("Reading file from args");
    let filename = env::args().nth(1).expect("Couldn't read file!");

    let input = fs::read_to_string(filename)?;
    Ok(input.trim().to_string())
}
