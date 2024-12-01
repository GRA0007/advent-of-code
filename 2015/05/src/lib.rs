use itertools::Itertools;

const NAUGHTY_COMBOS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn nice(word: &str) -> bool {
    // Doesn't contain any naughty combos
    if NAUGHTY_COMBOS.iter().any(|combo| word.contains(combo)) {
        return false;
    }

    // Has at least 3 vowels
    if word
        .chars()
        .into_iter()
        .filter(|letter| VOWELS.contains(letter))
        .count()
        < 3
    {
        return false;
    }

    // Has at least one double letter
    if !word
        .chars()
        .into_iter()
        .enumerate()
        .any(|(i, letter)| letter == word.chars().nth(i + 1).unwrap_or('!'))
    {
        return false;
    }

    true
}

pub fn new_nice(word: &str) -> bool {
    let counts: Vec<(char, usize)> = word
        .chars()
        .map(|letter| (letter, 1))
        .coalesce(|(l1, c1), (l2, c2)| {
            if l1 == l2 {
                Ok((l1, c1 + c2))
            } else {
                Err(((l1, c1), (l2, c2)))
            }
        })
        .collect();
    println!("{:?}", counts);

    true
}

#[cfg(test)]
mod tests {
    use crate::{new_nice, nice};

    #[test]
    fn nice_words() {
        assert!(nice("ugknbfddgicrmopn"));
        assert!(nice("aaa"));
    }

    #[test]
    fn naughty_words() {
        assert!(!nice("jchzalrnumimnmhp"));
        assert!(!nice("haegwjzuvuyypxyu"));
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn new_nice_words() {
        assert!(new_nice("aabbbaabbc"));
    }
}
