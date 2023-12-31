pub fn run(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut nums: Vec<u32> = Vec::default();
    for (y, line) in grid.iter().enumerate() {
        let mut curr_number: String = String::default();
        let mut attached_to_symbol = false;
        for (x, &c) in line.iter().enumerate() {
            if c.is_numeric() {
                curr_number.push(c);
                for x1 in (x.saturating_sub(1))..=(x + 1) {
                    for y1 in (y.saturating_sub(1))..=(y + 1) {
                        if let Some(line) = grid.get(y1) {
                            if let Some(c) = line.get(x1) {
                                if is_symbol(*c) {
                                    attached_to_symbol = true;
                                }
                            }
                        }
                    }
                }
                if x != line.len() - 1 {
                    continue;
                }
            }
            if !curr_number.is_empty() && attached_to_symbol {
                nums.push(curr_number.parse().unwrap())
            }
            attached_to_symbol = false;
            curr_number.clear();
        }
    }
    Ok(nums.iter().sum::<u32>().to_string())
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
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
        assert_eq!("4361", run(input)?);

        Ok(())
    }
}
