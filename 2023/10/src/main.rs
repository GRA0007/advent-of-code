use std::{collections::HashMap, fs};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug)]
struct Pipe(Vec<Direction>);

impl Pipe {
    fn has(&self, direction: Direction) -> bool {
        self.0.contains(&direction)
    }
}

impl TryFrom<char> for Pipe {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self(vec![Direction::North, Direction::South])),
            '-' => Ok(Self(vec![Direction::East, Direction::West])),
            'L' => Ok(Self(vec![Direction::North, Direction::East])),
            'J' => Ok(Self(vec![Direction::North, Direction::West])),
            '7' => Ok(Self(vec![Direction::South, Direction::West])),
            'F' => Ok(Self(vec![Direction::South, Direction::East])),
            'S' => Ok(Self(vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ])),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Field {
    tiles: HashMap<Position, Pipe>,
    start_pos: Position,
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut start_pos = None;

        for (y, line) in value.lines().enumerate() {
            for (x, tile) in line.trim().char_indices() {
                if let Ok(pipe) = Pipe::try_from(tile) {
                    tiles.insert(Position { x, y }, pipe);
                }

                if tile == 'S' {
                    start_pos = Some(Position { x, y });
                }
            }
        }

        Field {
            tiles,
            start_pos: start_pos.unwrap(),
        }
    }
}

impl Field {
    fn can_travel(&self, from: Position, direction: Direction) -> bool {
        match self.tiles.get(&from.go(direction)) {
            Some(new_pipe) => new_pipe.has(direction.opposite()),
            _ => false,
        }
    }

    fn farthest_distance(&self) -> usize {
        let mut distance = 0;
        let mut current_pos = self.start_pos;
        let mut backwards_dir: Option<Direction> = None;

        loop {
            let pipe = self.tiles.get(&current_pos).unwrap();
            let direction = pipe
                .0
                .iter()
                .filter(|direction| match backwards_dir {
                    Some(d) => d != **direction,
                    _ => true,
                })
                .find(|direction| self.can_travel(current_pos, **direction))
                .unwrap();

            backwards_dir = Some(direction.opposite());
            current_pos = current_pos.go(*direction);

            distance += 1;

            if current_pos == self.start_pos {
                break;
            }
        }

        distance / 2
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let field = Field::from(input.as_str());

    println!(
        "The distance to the farthest point is {}",
        field.farthest_distance()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_simple() {
        let input = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";
        let field = Field::from(input);
        assert_eq!(field.farthest_distance(), 4)
    }

    #[test]
    fn part_1_complex() {
        let input = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";
        let field = Field::from(input);
        assert_eq!(field.farthest_distance(), 8)
    }
}
