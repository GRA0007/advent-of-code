use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::RangeInclusive,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl From<&str> for Position {
    fn from(pos: &str) -> Self {
        let (x, y) = pos.split_once(", ").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Position {
    /// Calculate manhatten distance between two positions
    fn distance_to(&self, pos: &Position) -> usize {
        ((self.x - pos.x).abs() + (self.y - pos.y).abs()) as usize
    }

    /// Calculate the tuning frequency of a position
    fn tuning_frequency(&self) -> isize {
        (self.x * 4_000_000) + self.y
    }
}

struct Sensor {
    closest_beacon: Position,
    distance_to_beacon: usize,
}

impl Sensor {
    /// Calculate the range of a sensor at a position on a particular row
    fn range_on_row(&self, pos: &Position, y: isize) -> RangeInclusive<isize> {
        pos.x - self.distance_to_beacon as isize + (y - pos.y).abs()
            ..=pos.x + self.distance_to_beacon as isize - (y - pos.y).abs()
    }
}

struct Map(HashMap<Position, Sensor>);

impl Map {
    /// Ranges covered by all beacons on this row
    fn sensor_ranges(&self, y: isize) -> Vec<RangeInclusive<isize>> {
        self.0
            .iter()
            .map(|(pos, sensor)| sensor.range_on_row(pos, y))
            .filter(|range| !range.is_empty())
            .collect()
    }

    /// Search along a row, return the number of positions that can't contain a beacon
    fn no_beacons(&self, y: isize) -> usize {
        self.sensor_ranges(y)
            .into_iter()
            .flat_map(|range| range.into_iter().collect::<Vec<isize>>())
            .collect::<HashSet<isize>>()
            .into_iter()
            .filter(|x| {
                self.0
                    .values()
                    .all(|sensor| sensor.closest_beacon != Position { x: *x, y })
            })
            .count()
    }

    /// Find the position of a beacon within the coords 0 and max_coords
    fn find_beacon(&self, max_coords: usize) -> Option<Position> {
        for y in 0..=max_coords as isize {
            let ranges = self.sensor_ranges(y);

            let mut iter = 0..=max_coords as isize;
            while let Some(x) = iter.next() {
                let in_range = ranges.iter().find(|range| range.contains(&x));
                if let Some(range) = in_range {
                    // Skip until the end of this range
                    iter.nth((range.end() - x - 1).try_into().unwrap_or(0));
                } else {
                    return Some(Position { x, y });
                }
            }
        }
        None
    }
}

fn main() {
    let map = parse_input("input.txt");

    println!(
        "There are {} positions that cannot contain a beacon on row 2,000,000",
        map.no_beacons(2_000_000)
    );

    println!(
        "The tuning frequency of the beacon is {}",
        map.find_beacon(4_000_000).unwrap().tuning_frequency()
    );
}

fn parse_input(file_name: &str) -> Map {
    let input = fs::read_to_string(file_name).unwrap();

    Map(input
        .trim()
        .lines()
        .map(|line| {
            let line = line
                .replace("x=", "")
                .replace("y=", "")
                .replace("Sensor at ", "");
            let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();
            let (sensor, beacon): (Position, Position) = (sensor.into(), beacon.into());
            (
                sensor,
                Sensor {
                    closest_beacon: beacon,
                    distance_to_beacon: beacon.distance_to(&sensor),
                },
            )
        })
        .collect())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let map = parse_input("test.txt");
        assert_eq!(map.no_beacons(10), 26);
    }

    #[test]
    fn part_2() {
        let map = parse_input("test.txt");
        assert_eq!(map.find_beacon(20).unwrap().tuning_frequency(), 56000011);
    }

    #[test]
    fn sensor_range() {
        let position = Position { x: 8, y: 7 };
        let sensor = Sensor {
            closest_beacon: Position { x: 2, y: 10 },
            distance_to_beacon: 9,
        };

        assert_eq!(sensor.range_on_row(&position, 14), 6..=10);
        assert_eq!(sensor.range_on_row(&position, -2), 8..=8);
        assert_eq!(sensor.range_on_row(&position, 7), -1..=17);
        assert!(sensor.range_on_row(&position, 17).is_empty());
    }
}
