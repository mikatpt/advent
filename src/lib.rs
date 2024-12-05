#![allow(dead_code, unused_variables)]
#![feature(test)]
#![feature(let_chains)]

extern crate test;

pub mod days21;
pub mod days22;
pub mod days23;
pub mod days24;
pub mod template;
pub mod trace;

pub use color_eyre::eyre::eyre;
pub use color_eyre::Result;

use std::{env, fs, io::Write, path};
use tracing::info;

pub const YEAR: i32 = 24;

pub fn get_input(day: i32) -> Result<String> {
    if day == 0 {
        return Err(color_eyre::eyre::eyre!("Please input a day."));
    }
    let file = format!("{}/input/{YEAR}/{day:02}.txt", env!("CARGO_MANIFEST_DIR"));

    let path = path::Path::new(&file);
    if path.exists() {
        return Ok(fs::read_to_string(path)?);
    }

    let input = download_input(path, day)?;

    Ok(input)
}

fn download_input(path: &path::Path, day: i32) -> Result<String> {
    info!("downloading input for day {day:02}");

    let session = env::var("SESSION")?;
    let cookie = format!("session={}", session);
    let url = format!("https://adventofcode.com/20{YEAR}/day/{}/input", day);
    let client = reqwest::blocking::Client::new();

    let text = client.get(url).header("cookie", cookie).send()?.text()?;

    fs::create_dir_all(path.parent().expect("is dir"))?;
    let mut file = fs::File::create(path)?;
    file.write_all(text.as_bytes())?;
    info!("saved input to disk");

    Ok(text)
}
