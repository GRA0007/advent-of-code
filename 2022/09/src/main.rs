use std::{collections::HashSet, fs};

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        match letter {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Unknown direction"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    /// Check if two positions are not touching
    fn not_touching(&self, pos: Position) -> bool {
        (self.x - pos.x).abs() > 1 || (self.y - pos.y).abs() > 1
    }
}

struct Rope(Vec<Position>);

impl Rope {
    /// Initialise a new rope with n amount of knots
    fn new(knots: usize) -> Self {
        if knots < 2 {
            panic!("Number of knots must be at least 2");
        }
        Self((0..knots).map(|_| Position::default()).collect())
    }

    /// Move the head of the rope in a certain direction, and calculate where the tail will move
    fn go(&mut self, direction: Direction) -> Position {
        // Move the head
        self.0[0].go(&direction);

        // Move each knot down the line
        let mut prev_knot = self.0[0];
        for knot in self.0.iter_mut().skip(1) {
            if knot.not_touching(prev_knot) {
                // Calculate which direction to first go in
                let dir = if prev_knot.x > knot.x {
                    Direction::Right
                } else if prev_knot.x < knot.x {
                    Direction::Left
                } else if prev_knot.y > knot.y {
                    Direction::Down
                } else {
                    Direction::Up
                };

                // If on the same row or column, follow the head directly
                if knot.x == prev_knot.x || knot.y == prev_knot.y {
                    knot.go(&dir);
                } else {
                    // If not on the same row or column, move one step diagonally
                    knot.go(&dir);
                    let other_dir = if dir == Direction::Up || dir == Direction::Down {
                        if prev_knot.x > knot.x {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    } else if prev_knot.y > knot.y {
                        Direction::Down
                    } else {
                        Direction::Up
                    };
                    knot.go(&other_dir);
                }
                prev_knot = *knot;
            } else {
                break;
            }
        }
        *self.0.last().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let actions = parse_input(input);

    // Initialise the rope
    let mut rope = Rope::new(2);

    // Peform actions
    let tail_positions: HashSet<Position> = actions
        .clone()
        .into_iter()
        .map(|dir| rope.go(dir))
        .collect();

    println!("The tail touches {} unique positions", tail_positions.len());

    // Initialise the rope
    let mut longer_rope = Rope::new(10);

    // Peform actions
    let longer_tail_positions: HashSet<Position> =
        actions.into_iter().map(|dir| longer_rope.go(dir)).collect();

    println!(
        "The longer rope tail touches {} unique positions",
        longer_tail_positions.len()
    );
}

/// Take the input and expand each step into the directions to move in
fn parse_input(input: String) -> Vec<Direction> {
    input
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .flat_map(|(dir, count)| {
            (0..count.parse().unwrap()).map(|_| dir.chars().next().unwrap().try_into().unwrap())
        })
        .collect()
}

#[cfg(test)]
#[test]
fn test_small() {
    let mut rope = Rope::new(2);
    let tail_positions: HashSet<Position> = parse_input(String::from(
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    ))
    .into_iter()
    .map(|dir| rope.go(dir))
    .collect();

    assert_eq!(tail_positions.len(), 13);
}

#[cfg(test)]
#[test]
fn test_large() {
    let mut rope = Rope::new(10);
    let tail_positions: HashSet<Position> = parse_input(String::from(
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    ))
    .into_iter()
    .map(|dir| rope.go(dir))
    .collect();

    assert_eq!(tail_positions.len(), 36);
}
