use std::fs;

struct Stack(Vec<Card>);

struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl From<&str> for Stack {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|line| Card::from(line.trim())).collect())
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut groups = value.split_once(':').unwrap().1.split('|').map(|numbers| {
            numbers
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse().expect("valid number"))
                .collect()
        });

        Self {
            winning_numbers: groups.next().unwrap(),
            numbers: groups.next().unwrap(),
        }
    }
}

impl Card {
    fn calculate_points(&self) -> usize {
        self.numbers.iter().fold(0, |points, number| {
            if self.winning_numbers.contains(number) {
                match points {
                    0 => 1,
                    x => x * 2,
                }
            } else {
                points
            }
        })
    }

    fn count_wins(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

impl Stack {
    fn total_points(&self) -> usize {
        self.0.iter().map(|card| card.calculate_points()).sum()
    }

    fn total_cards(&self) -> usize {
        let mut counts: Vec<usize> = vec![1; self.0.len()];

        self.0.iter().enumerate().for_each(|(id, card)| {
            for _ in 0..counts[id] {
                for i in 0..card.count_wins() {
                    counts[id + i + 1] += 1;
                }
            }
        });

        counts.iter().sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let stack = Stack::from(input.as_str());

    println!("The card pile is worth {} points", stack.total_points());

    println!(
        "After reading the instructions, we have a total of {} cards",
        stack.total_cards()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1() {
        let stack = Stack::from(TEST_INPUT);
        assert_eq!(stack.total_points(), 13);
    }

    #[test]
    fn part_2() {
        let stack = Stack::from(TEST_INPUT);
        assert_eq!(stack.total_cards(), 30);
    }
}
