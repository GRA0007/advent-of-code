use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::{from_fn, once},
    rc::Rc,
};

const HEIGHTS: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    height: usize,
}

impl Position {
    fn do_move(&self, action: Action) -> Self {
        match action {
            Action::Up => Self {
                y: self.y - 1,
                ..*self
            },
            Action::Down => Self {
                y: self.y + 1,
                ..*self
            },
            Action::Left => Self {
                x: self.x - 1,
                ..*self
            },
            Action::Right => Self {
                x: self.x + 1,
                ..*self
            },
        }
    }
}

enum Action {
    Up,
    Down,
    Left,
    Right,
}

struct Terrain {
    heightmap: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start_pos: Position,
    end_pos: Position,
}

impl From<String> for Terrain {
    fn from(input: String) -> Self {
        let heightmap: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let start_pos = Terrain::find_pos(&heightmap, 'S')[0];
        Self {
            width: heightmap[0].len(),
            height: heightmap.len(),
            start_pos,
            end_pos: Terrain::find_pos(&heightmap, 'E')[0],
            heightmap,
        }
    }
}

impl Terrain {
    /// Search for a char and return it's position in the terrain
    fn find_pos(heightmap: &Vec<Vec<char>>, target: char) -> Vec<Position> {
        let mut all = Vec::new();
        for y in 0..heightmap.len() {
            for x in 0..heightmap[0].len() {
                if heightmap[y][x] == target {
                    all.push(Position {
                        x,
                        y,
                        height: match heightmap[y][x] {
                            'S' => 0,
                            'E' => HEIGHTS.len() - 1,
                            h => height_of(h),
                        },
                    });
                }
            }
        }
        all
    }

    fn height(&self, pos: Position) -> usize {
        height_of(self.heightmap[pos.y][pos.x])
    }

    /// Look at all tiles adjacent to the current position and return
    /// moves that are legal
    fn available_moves(&self, current_pos: Position) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();

        // Up
        if current_pos.y > 0
            && height_of(self.heightmap[current_pos.y - 1][current_pos.x]) <= current_pos.height + 1
        {
            positions.push(Position {
                x: current_pos.x,
                y: current_pos.y - 1,
                height: height_of(self.heightmap[current_pos.y - 1][current_pos.x]),
            });
        }

        // Down
        if current_pos.y < self.height - 1
            && height_of(self.heightmap[current_pos.y + 1][current_pos.x]) <= current_pos.height + 1
        {
            positions.push(Position {
                x: current_pos.x,
                y: current_pos.y + 1,
                height: height_of(self.heightmap[current_pos.y + 1][current_pos.x]),
            });
        }

        // Left
        if current_pos.x > 0
            && height_of(self.heightmap[current_pos.y][current_pos.x - 1]) <= current_pos.height + 1
        {
            positions.push(Position {
                x: current_pos.x - 1,
                y: current_pos.y,
                height: height_of(self.heightmap[current_pos.y][current_pos.x - 1]),
            });
        }

        // Right
        if current_pos.x < self.width - 1
            && height_of(self.heightmap[current_pos.y][current_pos.x + 1]) <= current_pos.height + 1
        {
            positions.push(Position {
                x: current_pos.x + 1,
                y: current_pos.y,
                height: height_of(self.heightmap[current_pos.y][current_pos.x + 1]),
            });
        }

        positions
    }

    fn search(&self, start_pos: Position) -> Option<usize> {
        let mut visited: HashSet<Position> = vec![start_pos].into_iter().collect();
        let mut queue: VecDeque<Node> = vec![Node::new(start_pos, None)].into();

        while let Some(node) = queue.pop_front() {
            // If goal
            if node.pos == self.end_pos {
                return Some(node.backtrace().len() - 1);
            }

            for pos in self.available_moves(node.pos).into_iter() {
                if !visited.contains(&pos) {
                    queue.push_back(Node::new(pos, Some(node.clone())));
                    visited.insert(pos);
                }
            }
        }
        None
    }
}

/// Find the height of a char as a number from 0 - 25
fn height_of(letter: char) -> usize {
    match letter {
        'S' => 0,
        'E' => HEIGHTS.len() - 1,
        l => HEIGHTS.chars().position(|c| c == l).unwrap(),
    }
}

#[derive(Clone)]
struct Node {
    pos: Position,
    parent: Option<Rc<Node>>,
}

impl Node {
    fn new(pos: Position, parent: Option<Node>) -> Self {
        Self {
            pos,
            parent: parent.map(Rc::new),
        }
    }

    /// Trace through the parent positions until reaching the start state
    fn backtrace(&self) -> Vec<Position> {
        let mut current = Rc::new(self.clone());
        once(self.pos)
            .chain(from_fn(move || {
                let parent = current.parent.clone();
                parent.map(|parent| {
                    current = parent;
                    current.pos
                })
            }))
            .collect()
    }
}

fn main() {
    let terrain: Terrain = fs::read_to_string("input.txt").unwrap().into();

    println!(
        "The shortest path is {}",
        terrain.search(terrain.start_pos).unwrap()
    );

    let shortest = Terrain::find_pos(&terrain.heightmap, 'a')
        .iter()
        .flat_map(|pos| terrain.search(*pos))
        .min()
        .unwrap();

    println!("The shortest path is {}", shortest);
}

#[cfg(test)]
#[test]
fn test() {
    let terrain: Terrain = String::from(
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    )
    .into();

    assert_eq!(terrain.search(terrain.start_pos).unwrap(), 31);
}
