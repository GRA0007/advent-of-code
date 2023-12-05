use std::fs;

use tqdm::Iter;

struct Range {
    destination: usize,
    source: usize,
    length: usize,
}

struct Map(Vec<Range>);

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl FromIterator<Range> for Map {
    fn from_iter<T: IntoIterator<Item = Range>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<String> for Almanac {
    fn from(value: String) -> Self {
        let seeds = value
            .lines()
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_ascii_whitespace()
            .map(|seed| seed.parse().unwrap())
            .collect();

        let maps = value
            .split("\n\n")
            .skip(1)
            .map(|category| {
                category
                    .lines()
                    .skip(1)
                    .map(|map_str| {
                        let mut numbers = map_str
                            .trim()
                            .split_ascii_whitespace()
                            .map(|number| number.parse().unwrap());
                        Range {
                            destination: numbers.next().unwrap(),
                            source: numbers.next().unwrap(),
                            length: numbers.next().unwrap(),
                        }
                    })
                    .collect()
            })
            .collect();

        Self { seeds, maps }
    }
}

impl Map {
    /// Take an input and process it to an output
    fn process(&self, input: usize) -> usize {
        self.0
            .iter()
            .find_map(|range| {
                if input >= range.source && input < (range.source + range.length) {
                    Some(range.destination + (input - range.source))
                } else {
                    None
                }
            })
            .unwrap_or(input)
    }
}

impl Almanac {
    fn lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| {
                let mut value = *seed;
                self.maps.iter().for_each(|map| {
                    value = map.process(value);
                });
                value
            })
            .tqdm()
            .min()
            .unwrap()
    }

    fn expand_seeds(&mut self) {
        let mut seeds = Vec::new();
        self.seeds.chunks_exact(2).for_each(|seed_info| {
            let start: usize = *seed_info.first().unwrap();
            let end: usize = *seed_info.first().unwrap() + *seed_info.last().unwrap();
            seeds.extend(start..end)
        });
        self.seeds = seeds;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut almanac = Almanac::from(input);

    println!("The lowest seed location is {}", almanac.lowest_location());

    almanac.expand_seeds();

    println!(
        "After knowing that the seeds are ranges, the lowest location is {}",
        almanac.lowest_location()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("test.txt").unwrap();
        let almanac = Almanac::from(input);
        assert_eq!(almanac.lowest_location(), 35)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut almanac = Almanac::from(input);
        almanac.expand_seeds();
        assert_eq!(almanac.lowest_location(), 46)
    }
}
