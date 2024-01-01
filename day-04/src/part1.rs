use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    number: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn power(&self) -> u32 {
        let count = self.winning.intersection(&self.have).count() as u32;
        if count == 0 {
            return 0;
        }
        2u32.pow(count.saturating_sub(1))
    }
}

fn parse_set(s: &str) -> IResult<&str, HashSet<u32>> {
    let (input, cards) = separated_list1(space1, complete::u32)(s)?;
    let mut set = HashSet::new();
    for &card in cards.iter() {
        set.insert(card);
    }
    Ok((input, set))
}

fn parse_card(line: &str) -> IResult<&str, Card> {
    let (input, number) = delimited(
        terminated(tag("Card"), space1),
        complete::u32,
        terminated(tag(":"), space1),
    )(line)?;
    let (input, (winning, have)) = separated_pair(
        parse_set,
        delimited(space1, tag("|"), space1),
        parse_set,
    )(input)?;
    Ok((
        input,
        Card {
            number,
            winning,
            have,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let (_, cards) = parse_cards(input)
        .map_err(|e| anyhow::anyhow!("parsing failed: {}", e.to_string()))?;
    Ok(cards.iter().map(|c| c.power()).sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", run(input)?);

        Ok(())
    }

    #[test]
    fn test_card() {
        let input: &str = "Card    5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let (_, c) = parse_card(input).expect("parse card");
        assert_eq!(5, c.number);
        assert_eq!(HashSet::from_iter([87, 83, 26, 28, 32]), c.winning);
        assert_eq!(
            HashSet::from_iter([88, 30, 70, 12, 93, 22, 82, 36]),
            c.have
        );

        let input: &str = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (_, _) = parse_card(input).expect("parse card");

        let input: &str = "Card  47: 25 62 88 36 24 10 15 90  8 23 |  4 54  9 80 42 78 32 19 21 65 86 26 28  7 96 71 48 64 36 95 69 44 89 34 57";
        let (_, _) = parse_card(input).expect("parse card");
    }
}
