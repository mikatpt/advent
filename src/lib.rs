#![allow(dead_code, unused_variables)]
#![feature(test)]

extern crate test;

pub mod days21;
pub mod days22;
pub mod trace;

pub use color_eyre::eyre::eyre;
pub use color_eyre::Result;

use std::{env, fs, io::Write, path};
use tracing::info;

pub fn get_input(day: i32) -> Result<String> {
    if day == 0 {
        return Err(color_eyre::eyre::eyre!("Please input a day."));
    }
    let file = format!("{}/input/22/{day:02}.txt", env!("CARGO_MANIFEST_DIR"));

    let path = path::Path::new(&file);
    if path.exists() {
        info!("input for day {day:02} already downloaded, reading from disk.");

        return Ok(fs::read_to_string(path)?);
    }

    let input = download_input(path, day)?;

    Ok(input)
}

fn download_input(path: &path::Path, day: i32) -> Result<String> {
    info!("downloading input for day {day:02}");

    let session = env::var("SESSION")?;
    let cookie = format!("session={}", session);
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let client = reqwest::blocking::Client::new();

    let text = client.get(url).header("cookie", cookie).send()?.text()?;

    let mut file = fs::File::create(path)?;
    file.write_all(text.as_bytes())?;
    info!("saved input to disk");

    Ok(text)
}
