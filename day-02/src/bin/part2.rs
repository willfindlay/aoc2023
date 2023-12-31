use anyhow::{Context as _, Result};
use day_02::part2::run;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let res = run(file).context("part 2")?;
    println!("{}", res);
    Ok(())
}
