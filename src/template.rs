use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
const TEMPLATE_STR: &str = r#"use crate::{get_input, Result};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input({{day}})?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

fn part1(input: &str) -> Result<i32> {
    Ok(0)
}

fn part2(input: &str) -> Result<i32> {
    Ok(0)
}

const INPUT: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(0, part1(INPUT).unwrap());
        assert_eq!(0, part2(INPUT).unwrap());
    }
}
"#;

pub fn gen_year(year_suffix: i32) -> crate::Result<()> {
    fs::create_dir_all(format!("src/days{year_suffix}"))?;

    let mut result = String::new();

    (1..32).for_each(|i| {
        result += &format!("pub mod d{i:02};\n");
    });

    File::create(Path::new(&format!("src/days{year_suffix}/mod.rs")))?
        .write_all(result.as_bytes())?;

    for i in 1..32 {
        gen_day(year_suffix, i)?;
    }

    Ok(())
}

fn gen_day(year_suffix: i32, day: i32) -> crate::Result<()> {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()?
        .parse(TEMPLATE_STR)?;
    let globals = liquid::object!({
        "day": day,
    });

    let output = template.render(&globals)?;

    let f = format!("src/days{year_suffix}/d{day:02}.rs");
    let file_path = Path::new(&f);
    if file_path.exists() {
        return Ok(());
    }
    File::create(file_path)?.write_all(output.as_bytes())?;
    Ok(())
}
