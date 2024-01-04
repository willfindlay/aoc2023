use std::{collections::HashMap, str::FromStr};

pub fn run(input: &str) -> anyhow::Result<String> {
    let mut hand_bids = parse(input);
    hand_bids.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    Ok(hand_bids
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.bid)
        .sum::<u32>()
        .to_string())
}

fn parse(input: &str) -> Vec<HandBids> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            HandBids {
                hand: Hand::from_str(hand).unwrap(),
                bid: bid.parse::<u32>().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct HandBids {
    hand: Hand,
    bid: u32,
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
enum Card {
    N(u32),
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            c if "23456789".contains(c) => Self::N(
                c.to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("invalid digit"))?,
            ),
            _ => anyhow::bail!("invalid card type"),
        })
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut cards = HashMap::new();
        for card in &self.cards {
            cards.entry(card).and_modify(|v| *v += 1).or_insert(1u32);
        }

        let mut pairs = 0;
        let mut triples = 0;
        let mut quads = 0;
        let mut pentas = 0;
        for n in cards.values() {
            match n {
                2 => pairs += 1,
                3 => triples += 1,
                4 => quads += 1,
                5 => pentas += 1,
                _ => {}
            }
        }

        match (pairs, triples, quads, pentas) {
            (0, 0, 0, 0) => HandType::HighCard,
            (1, 0, 0, 0) => HandType::OnePair,
            (2, 0, 0, 0) => HandType::TwoPair,
            (0, 1, 0, 0) => HandType::ThreeOfAKind,
            (1, 1, 0, 0) => HandType::FullHouse,
            (0, 0, 1, 0) => HandType::FourOfAKind,
            (0, 0, 0, 1) => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(std::cmp::Ordering::Equal) => {
                for (ours, theirs) in self.cards.iter().zip(&other.cards) {
                    if ours > theirs {
                        return Some(std::cmp::Ordering::Greater);
                    } else if theirs > ours {
                        return Some(std::cmp::Ordering::Less);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            comp => comp,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, Self::Err>>()?;
        Ok(Hand { cards })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", run(input)?);

        Ok(())
    }

    #[test]
    fn test_card_ordering() {
        assert!(Card::A > Card::T);
        assert!(Card::T > Card::N(9));
        assert!(Card::N(5) > Card::N(2));
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand::from_str("33332").expect("parse");
        let hand2 = Hand::from_str("2AAAA").expect("parse");
        assert!(hand1 > hand2);

        let hand1 = Hand::from_str("77888").expect("parse");
        let hand2 = Hand::from_str("77788").expect("parse");
        assert!(hand1 > hand2);

        let hand1 = Hand::from_str("AAA4A").expect("parse");
        let hand2 = Hand::from_str("AJAAA").expect("parse");
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_hand_type() {
        let hand = Hand::from_str("AAAA4").expect("parse");
        assert_eq!(HandType::FourOfAKind, hand.hand_type());
    }
}
