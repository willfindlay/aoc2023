use nom::{
    bytes::complete::tag,
    character::{
        complete,
        complete::{alpha0, line_ending, space1},
    },
    combinator::opt,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

struct BoatRace {
    distance: u32,
    time: u32,
}

impl BoatRace {
    fn ways(&self) -> u32 {
        (1..self.time)
            .map(|button_seconds| self.race(button_seconds))
            .filter(|&distance| distance > self.distance)
            .count() as u32
    }

    fn race(&self, button_seconds: u32) -> u32 {
        let remaining = self.time - button_seconds;
        button_seconds * remaining
    }
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = terminated(alpha0, tag(":"))(input)?;
    let (input, _) = space1(input)?;
    separated_list1(space1, complete::u32)(input)
}

fn parse_boat_races(input: &str) -> IResult<&str, Vec<BoatRace>> {
    let (input, times) = terminated(parse_nums, line_ending)(input)?;
    let (input, distances) = terminated(parse_nums, opt(line_ending))(input)?;
    Ok((
        input,
        times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| BoatRace { distance, time })
            .collect(),
    ))
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let (_, races) = parse_boat_races(input).expect("must parse");
    Ok(races.iter().map(|r| r.ways()).product::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", run(input)?);

        Ok(())
    }

    #[test]
    fn test_parse() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let (_, races) = parse_boat_races(input).expect("must parse");
        assert_eq!(3, races.len(), "race length");
    }
}
