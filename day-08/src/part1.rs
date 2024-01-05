use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

pub fn run(input: &str) -> anyhow::Result<String> {
    let (_, state) = parse(input).expect("parse");
    Ok(state.steps("AAA", "ZZZ").to_string())
}

struct State<'a> {
    directions: Vec<Direction>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> State<'a> {
    fn steps(&self, start: &str, end: &str) -> usize {
        let mut direction = self.directions.iter().cycle();
        let mut curr = start;
        let mut steps = 0;
        loop {
            let (left, right) = self.map.get(curr).unwrap();
            match direction.next().unwrap() {
                Direction::R => curr = right,
                Direction::L => curr = left,
            }
            steps += 1;
            if curr == end {
                break;
            }
        }
        steps
    }
}

enum Direction {
    R,
    L,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'R' => Self::R,
            'L' => Self::L,
            c => anyhow::bail!("bad character {}", c),
        })
    }
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    terminated(many1(one_of("RL")), line_ending)(input).map(|(input, chars)| {
        (
            input,
            chars
                .iter()
                .map(|&c| Direction::try_from(c).expect("valid direction"))
                .collect(),
        )
    })
}

fn parse_mapping(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, key) = terminated(alpha1, tag(" = "))(input)?;
    let (input, (left, right)) = delimited(
        complete::char('('),
        separated_pair(alpha1, tag(", "), alpha1),
        complete::char(')'),
    )(input)?;
    Ok((input, (key, (left, right))))
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (input, mappings) = separated_list1(line_ending, parse_mapping)(input)?;
    let mut m = HashMap::new();
    for (key, pair) in mappings {
        m.insert(key, pair);
    }
    Ok((input, m))
}

fn parse(input: &str) -> IResult<&str, State> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = line_ending(input)?;
    let (input, map) = parse_map(input)?;
    Ok((input, State { directions, map }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", run(input)?);

        Ok(())
    }
}
