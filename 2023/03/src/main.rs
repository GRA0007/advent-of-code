use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn is_adjacent_to(&self, pos: Position) -> bool {
        let distance = ((self.x as f64 - pos.x as f64).powf(2.0)
            + (self.y as f64 - pos.y as f64).powf(2.0))
        .sqrt();
        distance < 2.0
    }
}

enum Character {
    Number(char),
    Symbol(char),
}

impl TryFrom<char> for Character {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Err(()),
            number if number.is_ascii_digit() => Ok(Self::Number(number)),
            symbol => Ok(Self::Symbol(symbol)),
        }
    }
}

struct Schematic(HashMap<Position, Character>);

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut schematic = HashMap::new();
        value.lines().enumerate().for_each(|(y, line)| {
            line.trim().char_indices().for_each(|(x, char)| {
                if let Ok(character) = Character::try_from(char) {
                    schematic.insert(Position { x, y }, character);
                }
            })
        });

        Self(schematic)
    }
}

impl Schematic {
    fn get_symbol_positions(&self) -> Vec<&Position> {
        self.0
            .iter()
            .filter_map(|(position, character)| match character {
                Character::Symbol(_) => Some(position),
                _ => None,
            })
            .collect()
    }

    /// Number as (width, number)
    fn get_number_starting_with(&self, starting_position: &Position) -> (usize, Vec<Position>) {
        let mut current_pos: Position = *starting_position;
        let mut number = String::new();
        let mut positions = Vec::new();

        while let Some(Character::Number(digit)) = self.0.get(&current_pos) {
            number.push(*digit);
            positions.push(current_pos);
            current_pos = Position {
                x: current_pos.x + 1,
                y: current_pos.y,
            }
        }

        (number.parse().expect("Failed to parse number"), positions)
    }

    fn get_numbers(&self) -> Vec<(usize, Vec<Position>)> {
        self.0
            .iter()
            .filter_map(|(position, character)| {
                // Has to be a number if it starts on x: 0
                if position.x == 0 {
                    return Some(self.get_number_starting_with(position));
                }

                // Ignore number if not the first digit
                if self
                    .0
                    .get(&Position {
                        x: position.x - 1,
                        y: position.y,
                    })
                    .is_some_and(|c| matches!(c, Character::Number(_)))
                {
                    return None;
                }

                match character {
                    Character::Number(_) => Some(self.get_number_starting_with(position)),
                    _ => None,
                }
            })
            .collect()
    }

    fn is_number_adjacent_to_symbol(&self, number: (usize, Vec<Position>)) -> bool {
        let symbol_positions = self.get_symbol_positions();

        number.1.iter().any(|position| {
            symbol_positions
                .iter()
                .any(|s_pos| s_pos.is_adjacent_to(*position))
        })
    }

    /// Find the total of all numbers that are adjacent to symbols
    fn sum_part_numbers(&self) -> usize {
        self.get_numbers()
            .into_iter()
            .filter_map(|number| {
                let number_value = number.0;
                self.is_number_adjacent_to_symbol(number)
                    .then_some(number_value)
            })
            .sum()
    }

    fn get_gear_positions(&self) -> Vec<&Position> {
        self.0
            .iter()
            .filter_map(|(position, character)| match character {
                Character::Symbol(x) if x == &'*' => Some(position),
                _ => None,
            })
            .collect()
    }

    fn sum_gear_ratios(&self) -> usize {
        let gears = self.get_gear_positions();
        let numbers = self.get_numbers();

        gears
            .iter()
            .filter_map(|gear_pos| {
                let adjacent_numbers: Vec<usize> = numbers
                    .iter()
                    .filter_map(|number| {
                        number
                            .1
                            .iter()
                            .any(|number_pos| number_pos.is_adjacent_to(**gear_pos))
                            .then_some(number.0)
                    })
                    .collect();

                match adjacent_numbers.len() {
                    2 => Some(adjacent_numbers.iter().product::<usize>()),
                    _ => None,
                }
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let schematic = Schematic::from(input.as_str());

    println!(
        "Sum of all part numbers is {}",
        schematic.sum_part_numbers()
    );

    println!("Sum of all gear ratios is {}", schematic.sum_gear_ratios());
}

#[cfg(test)]
mod test {
    use crate::*;

    const TEST_INPUT: &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn part_1() {
        let schematic = Schematic::from(TEST_INPUT);

        assert_eq!(schematic.sum_part_numbers(), 4361);
    }

    #[test]
    fn part_2() {
        let schematic = Schematic::from(TEST_INPUT);

        assert_eq!(schematic.sum_gear_ratios(), 467835);
    }
}
