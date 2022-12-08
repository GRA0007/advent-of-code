use std::fs;

use take_until::TakeUntilExt;

struct Forest(Vec<Vec<usize>>);

#[derive(Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Forest {
    fn new(text: &str) -> Self {
        Self(
            text.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>()
                })
                .collect(),
        )
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    /// Check if a tree at a coordinate can be seen from outside the forest
    fn tree_visible(&self, c: Coordinate) -> bool {
        // If on edge
        if c.x == 0 || c.y == 0 || c.x == self.width() - 1 || c.y == self.height() - 1 {
            return true;
        }

        let height = self.0[c.y][c.x];

        // Left
        (0..c.x)
            .map(|i| self.0[c.y][i])
            .all(|tree| tree < height)
            // Right
            || (c.x + 1..self.width())
                .map(|i| self.0[c.y][i])
                .all(|tree| tree < height)
            // Up
            || (0..c.y)
                .map(|i| self.0[i][c.x])
                .all(|tree| tree < height)
            // Down
            || (c.y + 1..self.height())
                .map(|i| self.0[i][c.x])
                .all(|tree| tree < height)
    }

    /// Calculate the scenic score of a tree at a coordinate
    fn scenic_score(&self, c: Coordinate) -> usize {
        let height = self.0[c.y][c.x];

        // Left
        (0..c.x)
            .rev()
            .map(|i| self.0[c.y][i])
            .take_until(|tree| tree >= &height)
            .count()
            // Right
            * (c.x + 1..self.width())
                .map(|i| self.0[c.y][i])
                .take_until(|tree| tree >= &height)
                .count()
            // Up
            * (0..c.y)
                .rev()
                .map(|i| self.0[i][c.x])
                .take_until(|tree| tree >= &height)
                .count()
            // Down
            * (c.y + 1..self.height())
                .map(|i| self.0[i][c.x])
                .take_until(|tree| tree >= &height)
                .count()
    }

    /// Returns a vector of all coordinates in the forest
    fn coords(&self) -> Vec<Coordinate> {
        let (width, height) = (self.width(), self.height());
        (0..height * width)
            .map(|i| Coordinate {
                x: i / height,
                y: i % width,
            })
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let forest = Forest::new(input.trim());

    let visible_count = forest
        .coords()
        .iter()
        .filter(|c| forest.tree_visible(**c))
        .count();

    println!("There are {} trees visible in the forest", visible_count);

    let highest_scenic_score = forest
        .coords()
        .iter()
        .map(|c| forest.scenic_score(*c))
        .max()
        .unwrap_or(0);

    println!("The highest scenic score is {}", highest_scenic_score);
}
