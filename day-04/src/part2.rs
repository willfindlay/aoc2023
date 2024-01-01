use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Card {
    number: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

#[derive(Debug)]
struct CardState {
    // card number -> matches
    matches: BTreeMap<u32, u32>,
    // card number -> number of cards
    counts: BTreeMap<u32, u32>,
    // track score
    score: u32,
}

impl CardState {
    fn round(&mut self) -> bool {
        let nums: Vec<u32> = self.counts.keys().cloned().collect();
        for num in nums {
            if *self.counts.get(&num).expect("no count") == 0 {
                continue;
            }

            let matches = self.matches.get(&num).expect("no matches");
            for i in 1..=*matches {
                let to_inc = num + i;
                self.counts
                    .entry(to_inc)
                    .and_modify(|count| *count = (*count).saturating_add(1));
            }

            self.score += 1;
            self.counts
                .entry(num)
                .and_modify(|count| *count = (*count).saturating_sub(1));
        }
        if self.counts.iter().all(|(_, count)| *count == 0) {
            return false;
        }
        return true;
    }
}

impl From<Vec<Card>> for CardState {
    fn from(cards: Vec<Card>) -> Self {
        let mut matches: BTreeMap<u32, u32> = Default::default();
        let mut counts: BTreeMap<u32, u32> = Default::default();
        for card in cards {
            matches.insert(
                card.number,
                card.winning.intersection(&card.have).count() as u32,
            );
            counts.insert(card.number, 1u32);
        }
        CardState {
            matches,
            counts,
            score: 0,
        }
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
    let (input, (winning, have)) =
        separated_pair(parse_set, delimited(space1, tag("|"), space1), parse_set)(input)?;
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
    let (_, cards) =
        parse_cards(input).map_err(|e| anyhow::anyhow!("parsing failed: {}", e.to_string()))?;
    let mut state = CardState::from(cards);
    while state.round() {}
    Ok(state.score.to_string())
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
        assert_eq!("30", run(input)?);

        Ok(())
    }
}
