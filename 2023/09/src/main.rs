use std::fs;

#[derive(Clone, Debug)]
struct Sequence(Vec<isize>);

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        Self(
            value
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect(),
        )
    }
}

struct Report(Vec<Sequence>);

impl From<String> for Report {
    fn from(value: String) -> Self {
        Self(value.lines().map(Sequence::from).collect())
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Future,
    Past,
}

impl Sequence {
    fn calculate_differences(&self) -> Self {
        Self(
            self.0
                .windows(2)
                .map(|window| window.last().unwrap() - window.first().unwrap())
                .collect(),
        )
    }

    fn predict(&self, direction: Direction) -> isize {
        let sequence = match direction {
            Direction::Future => self.clone(),
            Direction::Past => Self(self.0.clone().into_iter().rev().collect()),
        };
        let mut diffs = vec![sequence.clone()];

        // Generate diffs
        while !diffs.last().unwrap().0.iter().all(|v| v == &0) {
            diffs.push(diffs.last().unwrap().calculate_differences());
        }

        let mut delta: isize = *diffs.pop().unwrap().0.last().unwrap();
        while let Some(sequence) = diffs.pop() {
            delta += sequence.0.last().unwrap();
        }

        delta
    }
}

impl Report {
    fn sum_predictions(&self, direction: Direction) -> isize {
        self.0
            .iter()
            .map(|sequence| sequence.predict(direction))
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let report = Report::from(input);

    println!(
        "Sum of extrapolated future values is {}",
        report.sum_predictions(Direction::Future)
    );
    println!(
        "Sum of extrapolated past values is {}",
        report.sum_predictions(Direction::Past)
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("test.txt").unwrap();
        let report = Report::from(input);
        assert_eq!(report.sum_predictions(Direction::Future), 114);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("test.txt").unwrap();
        let report = Report::from(input);
        assert_eq!(report.sum_predictions(Direction::Past), 2);
    }
}
