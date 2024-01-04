use anyhow::{Context as _, Result};
use day_05::part1::run;

fn main() -> Result<()> {
    let file = include_str!("../../input1.txt");
    let res = run(file).context("part 1")?;
    println!("{}", res);
    Ok(())
}