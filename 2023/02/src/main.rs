use std::{collections::HashMap, fs};

/// (colour, amount)
type Set = HashMap<String, usize>;

struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let game = value.trim().replace("Game ", "");
        let (id, sets) = game.split_once(": ").expect("Couldn't split game line");

        let sets: Vec<Set> = sets
            .split("; ")
            .map(|set| {
                set.split(", ")
                    .map(|cube| {
                        cube.split_once(' ')
                            .map(|(count, color)| {
                                (color.to_owned(), count.parse().expect("Should be a number"))
                            })
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        Self {
            id: id.parse().expect("ID is not a number"),
            sets,
        }
    }
}

impl Game {
    /// Check if a game is possible with a known set of cubes
    fn possible_with(&self, contents: Set) -> bool {
        self.sets.iter().all(|set| {
            contents.iter().all(|cube| {
                set.get(cube.0).is_none() || set.get(cube.0).is_some_and(|amount| amount <= cube.1)
            })
        })
    }

    /// Find the minimum possible set of cubes for a game
    fn minimum_set(&self) -> Set {
        let mut set = Set::new();
        self.sets.iter().for_each(|game_set| {
            game_set.iter().for_each(|cube| {
                if !set.get(cube.0).is_some_and(|count| count > cube.1) {
                    set.insert(cube.0.clone(), *cube.1);
                }
            })
        });
        set
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let games = input.lines().map(Game::from);

    let known_set = Set::from([
        ("red".to_owned(), 12),
        ("green".to_owned(), 13),
        ("blue".to_owned(), 14),
    ]);
    let possible_games_total: usize = games
        .clone()
        .filter_map(|game| game.possible_with(known_set.clone()).then_some(game.id))
        .sum();
    println!("Total IDs of possible games is {possible_games_total}");

    let min_set_power_sum: usize = games
        .map(|game| game.minimum_set().values().product::<usize>())
        .sum();
    println!("Total power of game minimum sets is {min_set_power_sum}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let known_set = Set::from([
            ("red".to_owned(), 12),
            ("green".to_owned(), 13),
            ("blue".to_owned(), 14),
        ]);

        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .lines()
                .map(Game::from)
                .filter_map(|game| game.possible_with(known_set.clone()).then_some(game.id))
                .sum::<usize>(),
            8
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .lines()
                .map(Game::from)
                .map(|game| game.minimum_set().values().product::<usize>())
                .sum::<usize>(),
            2286
        );
    }
}
