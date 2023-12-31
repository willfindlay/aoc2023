use std::collections::{BTreeMap, HashSet};

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut maybe_gears: BTreeMap<(usize, usize), Vec<u32>> = Default::default();
    for (y, line) in grid.iter().enumerate() {
        let mut curr_number: String = String::default();
        let mut curr_gears: HashSet<(usize, usize)> = Default::default();

        for (x, &c) in line.iter().enumerate() {
            if c.is_numeric() {
                curr_number.push(c);
                for x1 in (x.saturating_sub(1))..=(x + 1) {
                    for y1 in (y.saturating_sub(1))..=(y + 1) {
                        if let Some(line) = grid.get(y1) {
                            if let Some(c) = line.get(x1) {
                                if *c == '*' {
                                    curr_gears.insert((x1, y1));
                                }
                            }
                        }
                    }
                }
                if x != line.len() - 1 {
                    continue;
                }
            }

            if !curr_number.is_empty() {
                let num: u32 = curr_number.parse().unwrap();
                for (x, y) in &curr_gears {
                    maybe_gears
                        .entry((*x, *y))
                        .and_modify(|v| v.push(num))
                        .or_insert(vec![num]);
                }
            }

            curr_gears.clear();
            curr_number.clear();
        }
    }
    Ok(maybe_gears
        .iter()
        .filter(|(_, ns)| ns.len() == 2)
        .map(|(_, ns)| ns.iter().product::<u32>())
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", run(input)?);

        Ok(())
    }
}
