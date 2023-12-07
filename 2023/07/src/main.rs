use std::{cmp::Ordering, fs};

use itertools::Itertools;

const CARD_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Ord, PartialEq, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut cards: Vec<char> = self.cards.clone();
        cards.sort();
        let mut groups = Vec::new();
        for (_, group) in &cards.into_iter().group_by(|card| *card) {
            groups.push(group.count())
        }
        groups.sort_by(|a, b| b.cmp(a));

        if groups.len() == 1 {
            HandType::FiveOfAKind
        } else if groups[0] == 4 {
            HandType::FourOfAKind
        } else if groups[0] == 3 && groups[1] == 2 {
            HandType::FullHouse
        } else if groups[0] == 3 && groups.len() == 3 {
            HandType::ThreeOfAKind
        } else if groups[0] == 2 && groups[1] == 2 {
            HandType::TwoPair
        } else if groups[0] == 2 {
            HandType::OnePair
        } else if groups.len() == 5 {
            HandType::HighCard
        } else {
            unreachable!()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| {
                    match CARD_VALUES
                        .iter()
                        .position(|v| v == a)
                        .cmp(&CARD_VALUES.iter().position(|v| v == b))
                    {
                        Ordering::Equal => None,
                        ord => Some(ord),
                    }
                })
                .unwrap_or(Ordering::Equal), // Hands are identical
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
    }
}

impl Eq for Hand {}

#[derive(Debug)]
struct Hands(Vec<Hand>);

impl From<String> for Hands {
    fn from(value: String) -> Self {
        Self(
            value
                .lines()
                .map(|line| {
                    let (cards, bid) = line.trim().split_once(' ').unwrap();
                    let cards = cards.chars().collect();
                    let bid = bid.parse().unwrap();
                    Hand { cards, bid }
                })
                .collect(),
        )
    }
}

impl Hands {
    fn total_winnings(&self) -> usize {
        let mut sorted_hands = self.0.clone();
        sorted_hands.sort_by(|a, b| b.cmp(a));
        sorted_hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid)
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let hands = Hands::from(input);

    println!("Total winnings are {}", hands.total_winnings());
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("test.txt").unwrap();
        let hands = Hands::from(input);
        assert_eq!(hands.total_winnings(), 6440);
    }
}
