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

#[derive(Debug, Clone)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn go(&self, direction: Direction) -> Result<Self, ()> {
        match direction {
            Direction::North => {
                if self.y == 0 {
                    Err(())
                } else {
                    Ok(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Direction::South => Ok(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::East => Ok(Self {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::West => {
                if self.x == 0 {
                    Err(())
                } else {
                    Ok(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
        }
    }
}

struct Field {
    tiles: HashMap<Position, Pipe>,
    height: usize,
    width: usize,
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
            width: value.lines().next().unwrap().len(),
            height: value.lines().count(),
        }
    }
}

impl Field {
    fn can_travel(&self, from: Position, direction: Direction) -> bool {
        match from.go(direction) {
            Ok(new_pos) => match self.tiles.get(&new_pos) {
                Some(new_pipe) => new_pipe.has(direction.opposite()),
                _ => false,
            },
            Err(_) => false,
        }
    }

    /// Replace start pipe with pipe that fits
    fn infer_start(&mut self) {
        let directions = self
            .tiles
            .get(&self.start_pos)
            .unwrap()
            .0
            .clone()
            .into_iter()
            .filter(|direction| self.can_travel(self.start_pos, *direction))
            .collect();

        self.tiles.insert(self.start_pos, Pipe(directions));
    }

    /// Return a list of all pipes in the loop
    fn calculate_loop(&self) -> Vec<Position> {
        let mut pipes = Vec::new();
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

            pipes.push(current_pos);

            backwards_dir = Some(direction.opposite());
            current_pos = current_pos.go(*direction).unwrap();

            if current_pos == self.start_pos {
                break;
            }
        }

        pipes
    }

    /// Count number of tiles enclosed by the loop
    fn count_enclosed(&self, pipes: Vec<Position>) -> usize {
        let mut enclosed = 0;

        for y in 0..self.height {
            let mut in_loop = false;
            for x in 0..self.width {
                let pos = Position { x, y };
                if let Some(pipe) = self.tiles.get(&pos) {
                    if pipes.contains(&pos) && pipe.0.contains(&Direction::North) {
                        in_loop = !in_loop;
                    }
                } else if in_loop {
                    enclosed += 1;
                }
            }
        }

        enclosed
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut field = Field::from(input.as_str());

    field.infer_start();
    let pipes = field.calculate_loop();

    println!("The distance to the farthest point is {}", pipes.len() / 2);
    println!(
        "The number of tiles enclosed within the loop is {}",
        field.count_enclosed(pipes)
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
        assert_eq!(field.calculate_loop().len() / 2, 4)
    }

    #[test]
    fn part_1_complex() {
        let input = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";
        let field = Field::from(input);
        assert_eq!(field.calculate_loop().len() / 2, 8)
    }

    #[test]
    fn part_2_simple() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        let mut field = Field::from(input);
        field.infer_start();
        assert_eq!(field.count_enclosed(field.calculate_loop()), 4)
    }

    #[test]
    fn part_2_medium() {
        let input = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";
        let mut field = Field::from(input);
        field.infer_start();
        assert_eq!(field.count_enclosed(field.calculate_loop()), 8)
    }

    #[test]
    fn part_2_complex() {
        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        let mut field = Field::from(input);
        field.infer_start();
        assert_eq!(field.count_enclosed(field.calculate_loop()), 10)
    }
}
