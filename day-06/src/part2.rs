use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, digit1, line_ending, space1},
    combinator::opt,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

struct BoatRace {
    distance: u128,
    time: u128,
}

impl BoatRace {
    fn ways(&self) -> u128 {
        (1..self.time)
            .map(|button_seconds| self.race(button_seconds))
            .filter(|&distance| distance > self.distance)
            .count() as u128
    }

    fn race(&self, button_seconds: u128) -> u128 {
        let remaining = self.time - button_seconds;
        button_seconds * remaining
    }
}

fn parse_num(input: &str) -> IResult<&str, u128> {
    let (input, _) = terminated(alpha0, tag(":"))(input)?;
    let (input, _) = space1(input)?;
    let (input, nums) = separated_list1(space1, digit1)(input)?;
    Ok((input, nums.concat().parse().expect("should be valid int")))
}

fn parse_boat_race(input: &str) -> IResult<&str, BoatRace> {
    let (input, time) = terminated(parse_num, line_ending)(input)?;
    let (input, distance) = terminated(parse_num, opt(line_ending))(input)?;
    Ok((input, BoatRace { distance, time }))
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let (_, race) = parse_boat_race(input).expect("must parse");
    Ok(race.ways().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", run(input)?);

        Ok(())
    }

    #[test]
    fn test_parse() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        parse_boat_race(input).expect("must parse");
    }
}
