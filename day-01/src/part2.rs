const NUMS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(input: &str) -> anyhow::Result<String> {
    let lines = parse(input);
    let mut sum = 0;
    for mut line in lines {
        while !NUMS.iter().any(|s| line.starts_with(s))
            && !line.chars().next().map(|c| c.is_numeric()).unwrap_or(true)
        {
            line = &line[1..]
        }
        while !NUMS.iter().any(|s| line.ends_with(s))
            && !line.chars().last().map(|c| c.is_numeric()).unwrap_or(true)
        {
            line = &line[..line.len() - 1]
        }

        let mut a = 0;
        let mut b = 0;

        for (x, s) in NUMS.iter().enumerate() {
            if line.starts_with(s) {
                a = x + 1
            }
            if line.ends_with(s) {
                b = x + 1
            }
        }
        let mut chars = line.chars().peekable();
        if chars.peek().unwrap().is_numeric() {
            a = chars.peek().unwrap().to_digit(10).unwrap() as usize;
        }
        let last = chars.last().unwrap();
        if last.is_numeric() {
            b = last.to_digit(10).unwrap() as usize;
        }
        sum += a * 10 + b
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", run(input)?);

        Ok(())
    }
}
