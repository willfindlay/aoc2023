use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

pub fn run(input: &str) -> anyhow::Result<String> {
    let (_, histories) = parse(input).expect("parse");
    Ok(histories
        .iter()
        .map(|h| h.find_next())
        .sum::<i64>()
        .to_string())
}

#[derive(Debug)]
struct History(Vec<i64>);

impl History {
    fn find_next(&self) -> i64 {
        let mut ends = vec![];
        let mut nums = self.0.clone();
        loop {
            if nums.iter().all(|&x| x == 0) {
                break;
            }
            nums = nums
                .iter()
                .tuple_windows()
                .with_position()
                .map(|(position, (a, b))| {
                    match position {
                        itertools::Position::Last | itertools::Position::Only => ends.push(*b),
                        _ => {}
                    };
                    b - a
                })
                .collect();
        }
        ends.iter().sum()
    }
}

fn parse_history(input: &str) -> IResult<&str, History> {
    let (input, list) = separated_list1(space1, complete::i64)(input)?;
    Ok((input, History(list)))
}

fn parse(input: &str) -> IResult<&str, Vec<History>> {
    let (input, histories) = separated_list1(line_ending, parse_history)(input)?;
    Ok((input, histories))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", run(input)?);

        Ok(())
    }
}
