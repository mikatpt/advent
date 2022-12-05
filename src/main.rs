use advent::days22::d05::solve;
use advent::Result;

fn main() -> Result<()> {
    dotenv::dotenv()?;
    advent::trace::init();

    let (p1, p2) = solve()?;

    println!("Answer to part 1 is {}", p1);
    println!("Answer to part 2 is {}", p2);

    Ok(())
}
