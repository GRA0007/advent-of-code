use std::{collections::HashMap, fmt, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl From<&str> for Position {
    fn from(pos: &str) -> Self {
        let (x, y) = pos.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Position {
    /// Take a position and return a new position moved 1 tile towards another position
    /// Note: must be in the same row or column
    fn move_towards(&self, pos: Position) -> Position {
        Position {
            x: (self.x as isize + (pos.x as isize - self.x as isize).signum()) as usize,
            y: (self.y as isize + (pos.y as isize - self.y as isize).signum()) as usize,
        }
    }
}

enum Tile {
    Rock,
    Sand,
}

struct Scan {
    map: HashMap<Position, Tile>,
    max_y: usize,
    floor: Option<usize>,
}

impl fmt::Debug for Scan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x_values = self.map.keys().map(|key| key.x);
        let (min_x, max_x) = (x_values.clone().min().unwrap(), x_values.max().unwrap());
        for y in 0..=self.floor.unwrap_or(self.max_y) {
            for x in min_x..=max_x {
                write!(
                    f,
                    "{}",
                    if y == self.floor.unwrap_or(self.max_y + 1) {
                        '#'
                    } else {
                        match self.map.get(&Position { x, y }) {
                            Some(Tile::Rock) => '#',
                            Some(Tile::Sand) => 'o',
                            _ => '.',
                        }
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Scan {
    /// Drop a grain of sand and wait for it to settle, returns
    /// Some if it settled, or None if it fell off the map (or can't move from the source)
    fn drop_sand(&self, from: Position) -> Option<Position> {
        // If the source tile is blocked
        if self.map.get(&from).is_some() {
            return None;
        }

        let mut previous = from;
        while let Some(next) = self.next_tile(previous) {
            // Grain out of bounds, failed to settle!
            if next.y > self.max_y && self.floor.is_none() {
                return None;
            }
            previous = next;
        }
        Some(previous)
    }

    /// Try and find the next empty tile from a sand position, or return None
    fn next_tile(&self, from: Position) -> Option<Position> {
        // If there is a floor
        if let Some(floor) = self.floor {
            if floor == from.y + 1 {
                return None;
            }
        }

        let check = vec![
            Position {
                x: from.x,
                y: from.y + 1,
            },
            Position {
                x: from.x - 1,
                y: from.y + 1,
            },
            Position {
                x: from.x + 1,
                y: from.y + 1,
            },
        ];
        check
            .into_iter()
            .find(|&next| !self.map.contains_key(&next))
    }

    /// Simulate the sand falling from a position until it spills over,
    /// returns the amount of sand that came to rest
    fn simulate_sand(&mut self, from: Position) -> usize {
        let mut count: usize = 0;
        while let Some(pos) = self.drop_sand(from) {
            self.map.insert(pos, Tile::Sand);
            count += 1;
        }
        count
    }
}

fn main() {
    let mut scan = parse_input("input.txt", false);
    let resting = scan.simulate_sand(Position { x: 500, y: 0 });
    println!(
        "{} grains came to rest before sand spilled into the abyss",
        resting
    );

    let mut scan = parse_input("input.txt", true);
    let resting = scan.simulate_sand(Position { x: 500, y: 0 });
    println!(
        "{} grains came to rest before the source was blocked",
        resting
    );
}

fn parse_input(file_name: &str, has_floor: bool) -> Scan {
    let input = fs::read_to_string(file_name).unwrap();

    let walls: Vec<Vec<Position>> = input
        .trim()
        .lines()
        .into_iter()
        .map(|line| line.split(" -> ").map(|p| p.into()).collect())
        .collect();

    // Loop through all the walls and fill with rock
    let mut map = HashMap::new();
    for wall in walls {
        for window in wall.windows(2) {
            let (mut pos, target) = (window[0], window[1]);
            map.insert(pos, Tile::Rock);
            while pos != target {
                pos = pos.move_towards(target);
                map.insert(pos, Tile::Rock);
            }
        }
    }

    // Calculate y pos of the lowest wall
    let max_y = map.keys().map(|key| key.y).max().unwrap();

    Scan {
        map,
        max_y,
        floor: if has_floor { Some(max_y + 2) } else { None },
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let mut scan = parse_input("test.txt", false);
        let resting = scan.simulate_sand(Position { x: 500, y: 0 });
        assert_eq!(resting, 24);
    }

    #[test]
    fn part_2() {
        let mut scan = parse_input("test.txt", true);
        let resting = scan.simulate_sand(Position { x: 500, y: 0 });
        assert_eq!(resting, 93);
    }
}
