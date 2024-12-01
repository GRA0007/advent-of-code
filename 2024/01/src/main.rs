use std::fs;

struct List {
    left: Vec<isize>,
    right: Vec<isize>,
}

impl List {
    fn get_pairs(&mut self) -> Vec<Pair> {
        self.left.sort();
        self.right.sort();

        self.left
            .iter()
            .enumerate()
            .map(|(i, left)| Pair(*left, self.right[i]))
            .collect()
    }
}

impl From<String> for List {
    fn from(value: String) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in value.lines() {
            let (l, r) = line.split_once("   ").unwrap();
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
        Self { left, right }
    }
}

struct Pair(isize, isize);

impl Pair {
    fn distance(&self) -> usize {
        (self.0 - self.1).abs().try_into().unwrap()
    }
}

fn total_distance(pairs: Vec<Pair>) -> usize {
    pairs.iter().map(|pair| pair.distance()).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut list: List = input.into();
    println!("Total distance is: {}", total_distance(list.get_pairs()))
}

#[cfg(test)]
mod test {
    use crate::{total_distance, List};

    #[test]
    fn test() {
        let mut list: List = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_owned()
            .into();
        assert_eq!(total_distance(list.get_pairs()), 11);
    }
}
