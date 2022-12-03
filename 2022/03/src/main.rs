use std::{collections::HashSet, fs};

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(item: char) -> usize {
    PRIORITIES
        .chars()
        .position(|char| char == item)
        .expect("Item to be in priorities")
        + 1
}

struct Rucksack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}

impl Rucksack {
    fn fill(items: &str) -> Self {
        if items.len() % 2 != 0 {
            panic!("Tried to fill a bag with an uneven number of items");
        }
        let count = items.len() / 2;
        let items = items.chars();
        Rucksack {
            compartment1: items.clone().take(count).collect(),
            compartment2: items.skip(count).take(count).collect(),
        }
    }

    fn common_item(&self) -> char {
        *self
            .compartment1
            .iter()
            .find(|item| self.compartment2.iter().any(|i| i == *item))
            .expect("No common item found")
    }

    fn contents(&self) -> Vec<char> {
        [self.compartment1.clone(), self.compartment2.clone()].concat()
    }
}

struct ElfGroup {
    badge: Option<char>,
}

impl ElfGroup {
    fn new(sacks: &[Rucksack]) -> Self {
        // Find common item in all three sacks
        let set: HashSet<_> = sacks
            .iter()
            .map(|sack| HashSet::from_iter(sack.contents().into_iter()))
            .reduce(|intersection, set| {
                intersection
                    .into_iter()
                    .filter(|char| set.contains(char))
                    .collect::<HashSet<_>>()
            })
            .unwrap();
        ElfGroup {
            badge: set.into_iter().next(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let priority_sum: usize = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(Rucksack::fill)
        .map(|sack| priority(sack.common_item()))
        .sum();

    println!("The total priority is {}", priority_sum);

    let group_sum: usize = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(Rucksack::fill)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(ElfGroup::new)
        .map(|group| priority(group.badge.unwrap()))
        .sum();

    println!("The total group priority is {}", group_sum);
}
