use std::{cmp::Ordering, fs};

use serde::Deserialize;

// serde op frfr
#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum Value {
    Integer(usize),
    List(Vec<Value>),
}

impl Default for Value {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        serde_json::from_str(value).unwrap()
    }
}

impl Ord for Value {
    /// Less is in order, Greater is not
    /// This is a recursive function that will traverse throught the layers
    fn cmp(&self, other: &Self) -> Ordering {
        // If both integers
        if let (Value::Integer(left), Value::Integer(right)) = (self, other) {
            return left.cmp(right);
        }

        // If left or right is an integer, convert to a list
        let left = match self {
            Value::Integer(i) => Value::List(vec![Value::Integer(*i)]),
            x => x.clone(),
        };
        let right = match other {
            Value::Integer(i) => Value::List(vec![Value::Integer(*i)]),
            x => x.clone(),
        };
        if let (Value::List(left), Value::List(right)) = (left, right) {
            for (i, left_item) in left.iter().enumerate() {
                let right_item = right.get(i);
                if let Some(right_item) = right_item {
                    match left_item.cmp(right_item) {
                        Ordering::Equal => {}
                        x => return x,
                    }
                } else {
                    return Ordering::Greater;
                }
            }
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

/// Count the number of ordered pairs
fn count_ordered_pairs(pairs: Vec<(Value, Value)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| match left.cmp(right) {
            Ordering::Less => Some(i + 1), // In order
            _ => None,
        })
        .sum()
}

/// Insert the divider packets and calculate the decoder key
fn calculate_decoder_key(pairs: Vec<(Value, Value)>) -> usize {
    // Flatten the pairs, we don't care about them
    let mut keys: Vec<Value> = pairs.into_iter().flat_map(|p| vec![p.0, p.1]).collect();

    // Insert the divider packets
    keys.push(Value::List(vec![Value::List(vec![Value::Integer(2)])]));
    keys.push(Value::List(vec![Value::List(vec![Value::Integer(6)])]));

    // Sort the keys (uses the ordering we set up above!)
    keys.sort();

    keys.iter()
        .enumerate()
        .filter_map(|(i, key)| {
            // I dislike all this nesting but it appears necessary to find the appropriate enum types
            if let Value::List(list) = key {
                if list.len() != 1 {
                    return None;
                }
                if let Value::List(list) = &list[0] {
                    if list.len() != 1 {
                        return None;
                    }
                    return match list[0] {
                        Value::Integer(2) => Some(i + 1),
                        Value::Integer(6) => Some(i + 1),
                        _ => None,
                    };
                }
            }
            None
        })
        .product()
}

fn main() {
    let pairs = parse_input("input.txt");

    println!(
        "The number of ordered pairs is {}",
        count_ordered_pairs(pairs.clone())
    );

    println!("The decoder key is {}", calculate_decoder_key(pairs));
}

#[cfg(test)]
#[test]
fn test_part_1() {
    let pairs = parse_input("test.txt");

    assert_eq!(count_ordered_pairs(pairs), 13);
}

#[cfg(test)]
#[test]
fn test_part_2() {
    let pairs = parse_input("test.txt");

    assert_eq!(calculate_decoder_key(pairs), 140);
}

/// Parse the input into a vector of tuples of values
fn parse_input(file_name: &str) -> Vec<(Value, Value)> {
    let input = fs::read_to_string(file_name).unwrap();

    input
        .trim()
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap())
        .map(|(l, r)| (l.into(), r.into()))
        .collect()
}
