use advent::days23::*;
use advent::Result;

fn main() -> Result<()> {
    dotenv::dotenv()?;
    advent::trace::init();

    let day = std::env::args()
        .nth(1)
        .expect("please specify a day!")
        .parse()?;

    let solve = match day {
        1 => d01::solve,
        2 => d02::solve,
        3 => d03::solve,
        4 => d04::solve,
        5 => d05::solve,
        6 => d06::solve,
        7 => d07::solve,
        8 => d08::solve,
        9 => d09::solve,
        10 => d10::solve,
        11 => d11::solve,
        12 => d12::solve,
        13 => d13::solve,
        14 => d14::solve,
        15 => d15::solve,
        16 => d16::solve,
        17 => d17::solve,
        18 => d18::solve,
        19 => d19::solve,
        20 => d20::solve,
        21 => d21::solve,
        22 => d22::solve,
        23 => d23::solve,
        24 => d24::solve,
        25 => d25::solve,
        26 => d26::solve,
        27 => d27::solve,
        28 => d28::solve,
        29 => d29::solve,
        30 => d30::solve,
        31 => d31::solve,
        _ => panic!("choose a day 1-31"),
    };
    let (p1, p2) = solve()?;

    println!("Answer to part 1 is {}", p1);
    println!("Answer to part 2 is {}", p2);

    Ok(())
}
