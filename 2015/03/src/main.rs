use std::{collections::HashMap, fs};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location(isize, isize);

impl Location {
    fn up(&self) -> Self {
        Location(self.0, self.1 + 1)
    }
    fn down(&self) -> Self {
        Location(self.0, self.1 - 1)
    }
    fn left(&self) -> Self {
        Location(self.0 - 1, self.1)
    }
    fn right(&self) -> Self {
        Location(self.0 + 1, self.1)
    }
    fn travel(&self, direction: char) -> Self {
        match direction {
            '^' => self.up(),
            'v' => self.down(),
            '<' => self.left(),
            '>' => self.right(),
            _ => panic!("Invalid direction provided"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (robot, santa): (Vec<_>, Vec<_>) = input
        .chars()
        .take_while(|dir| !dir.is_whitespace())
        .enumerate()
        .partition(|&(i, _)| i % 2 == 0);
    let mut map: HashMap<Location, usize> = HashMap::new();

    let mut current_location = Location(0, 0);
    map.insert(current_location, 1);
    for dir in robot.iter().map(|(_, dir)| dir) {
        current_location = current_location.travel(*dir);
        let count = map.entry(current_location).or_insert(0);
        *count += 1;
    }
    current_location = Location(0, 0);
    for dir in santa.iter().map(|(_, dir)| dir) {
        current_location = current_location.travel(*dir);
        let count = map.entry(current_location).or_insert(0);
        *count += 1;
    }
    println!("Houses: {:#?}", map.values().len());
}
