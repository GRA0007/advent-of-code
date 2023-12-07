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
    fn get_type(&self, joker: char) -> HandType {
        let mut cards: Vec<char> = self.cards.clone();

        // Replace jokers
        cards = cards
            .into_iter()
            .map(|card| match card {
                'J' => joker,
                c => c,
            })
            .collect();

        // Find groups
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

    fn highest_joker(&self) -> char {
        CARD_VALUES
            .into_iter()
            .filter(|v| *v != 'J')
            .map(|value| (value, self.get_type(value)))
            .max_by(|a, b| b.1.cmp(&a.1))
            .unwrap()
            .0
    }

    fn cmp(&self, other: &Self, jokers: bool) -> Ordering {
        let mut card_values = CARD_VALUES.to_vec();
        let mut joker_values = ('J', 'J');

        if jokers {
            // Move joker to the end
            card_values.retain(|c| *c != 'J');
            card_values.push('J');

            // Find the highest jokers to use
            joker_values = (self.highest_joker(), other.highest_joker())
        }

        match self
            .get_type(joker_values.0)
            .cmp(&other.get_type(joker_values.1))
        {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| {
                    match card_values
                        .iter()
                        .position(|v| v == a)
                        .cmp(&card_values.iter().position(|v| v == b))
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
    fn total_winnings(&self, jokers: bool) -> usize {
        let mut sorted_hands = self.0.clone();
        sorted_hands.sort_by(|a, b| b.cmp(a, jokers));
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

    println!("Total winnings are {}", hands.total_winnings(false));
    println!(
        "Total winnings with Jokers is {}",
        hands.total_winnings(true)
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("test.txt").unwrap();
        let hands = Hands::from(input);
        assert_eq!(hands.total_winnings(false), 6440);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("test.txt").unwrap();
        let hands = Hands::from(input);
        assert_eq!(hands.total_winnings(true), 5905);
    }
}
