use std::{collections::HashMap, fs};

struct Map {
    instructions: String,
    nodes: HashMap<String, (String, String)>,
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let mut lines = value.lines();

        let instructions = lines.next().unwrap().to_owned();
        lines.next().unwrap();

        let nodes = HashMap::from_iter(lines.map(|line| {
            let (from, to) = line.trim().split_once(" = ").unwrap();
            let (left, right) = to
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            (from.to_owned(), (left.to_owned(), right.to_owned()))
        }));

        Self {
            instructions,
            nodes,
        }
    }
}

/// Calculate greatest common divisor
fn gcd(a: usize, b: usize) -> usize {
    if a > 0 {
        gcd(b % a, a)
    } else {
        b
    }
}

/// Calculate least common multiple
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

impl Map {
    /// Steps from a starting location to any location ending with a string
    fn steps(&self, start: &str, end: &str) -> usize {
        let mut steps = 0;
        let mut location = String::from(start);

        for direction in self.instructions.chars().cycle() {
            let node = self.nodes.get(&location).unwrap();
            location = if direction == 'L' {
                node.0.clone()
            } else {
                node.1.clone()
            };
            steps += 1;

            if location.ends_with(end) {
                break;
            }
        }

        steps
    }

    fn ghost_steps(&self) -> usize {
        self.nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|start| self.steps(start, "Z"))
            .reduce(lcm) // Find the least common multiple of all distances
            .unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = Map::from(input);

    println!(
        "It takes you {} steps to reach ZZZ",
        map.steps("AAA", "ZZZ")
    );
    println!("As a ghost, it takes you {} steps", map.ghost_steps());
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1_2_steps() {
        let input = String::from(
            "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)",
        );
        let map = Map::from(input);
        assert_eq!(map.steps("AAA", "ZZZ"), 2);
    }

    #[test]
    fn part_1_6_steps() {
        let input = String::from(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
        );
        let map = Map::from(input);
        assert_eq!(map.steps("AAA", "ZZZ"), 6);
    }

    #[test]
    fn part_2() {
        let input = String::from(
            "LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)",
        );
        let map = Map::from(input);
        assert_eq!(map.ghost_steps(), 6);
    }
}
