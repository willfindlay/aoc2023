pub fn run(input: &str) -> anyhow::Result<String> {
    let lines = parse(input);
    let mut sum = 0;
    for line in lines {
        let mut iter = line
            .chars()
            .filter(|c| c.is_digit(10))
            .into_iter()
            .peekable();
        let first = iter.peek().cloned();
        let last = iter.last();
        let ns: String = [first.unwrap(), last.unwrap()].iter().collect();
        let n = ns.parse::<i32>().unwrap();
        sum += n
    }
    Ok(sum.to_string())
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", run(input)?);

        Ok(())
    }
}
