use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_enemy_action(action: char) -> Self {
        match action {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            x => panic!("Unknown enemy action {}", x),
        }
    }

    fn from_response(action: char) -> Self {
        match action {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            x => panic!("Unknown response {}", x),
        }
    }

    fn score(&self) -> usize {
        match &self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn match_score_against(&self, shape: &Self) -> usize {
        if self == shape {
            return 3;
        }
        if match &self {
            Self::Rock => shape == &Self::Scissors,
            Self::Paper => shape == &Self::Rock,
            Self::Scissors => shape == &Self::Paper,
        } {
            6
        } else {
            0
        }
    }

    fn score_against(&self, shape: &Self) -> usize {
        self.match_score_against(shape) + self.score()
    }

    fn score_using_action(&self, action: char) -> usize {
        let chosen_shape = match action {
            'X' => match &self {
                // lose
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            'Y' => *self, // draw
            'Z' => match &self {
                // win
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            x => panic!("Unknown action {}", x),
        };
        chosen_shape.score_against(self)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let input = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().filter(|char| !char.is_whitespace()))
        .map(|mut chars| (chars.next().unwrap(), chars.next().unwrap()));

    let total_score: usize = input
        .clone()
        .map(|actions| {
            (
                Shape::from_enemy_action(actions.0),
                Shape::from_response(actions.1),
            )
        })
        .map(|shapes| shapes.1.score_against(&shapes.0))
        .sum();

    println!("The total score is: {}", total_score);

    let total_score_part_2: usize = input
        .map(|actions| Shape::from_enemy_action(actions.0).score_using_action(actions.1))
        .sum();

    println!(
        "The total score with corrected instructions is: {}",
        total_score_part_2
    );
}
