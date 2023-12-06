use std::fs;

struct Race {
    duration: usize,
    record: usize,
}

fn distance_travelled(speed: usize, duration: usize) -> usize {
    speed * duration
}

fn parse_races(value: String) -> Vec<Race> {
    let numbers: Vec<Vec<usize>> = value
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    numbers
        .first()
        .unwrap()
        .iter()
        .zip(numbers.last().unwrap())
        .map(|(duration, record)| Race {
            duration: *duration,
            record: *record,
        })
        .collect()
}

fn parse_big_race(value: String) -> Race {
    let numbers: Vec<usize> = value
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .parse()
                .unwrap()
        })
        .collect();

    Race {
        duration: *numbers.first().unwrap(),
        record: *numbers.last().unwrap(),
    }
}

impl Race {
    fn count_win_conditions(&self) -> usize {
        let mut button_duration = 1;
        let mut win_conditions = 0;

        loop {
            if distance_travelled(button_duration, self.duration - button_duration) > self.record {
                win_conditions += 1;
            } else if win_conditions > 0 {
                break;
            }
            button_duration += 1;
        }

        win_conditions
    }
}

fn total_margin(races: Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| race.count_win_conditions())
        .product()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!(
        "The product of the number of ways to win each race is {}",
        total_margin(parse_races(input.clone()))
    );

    println!(
        "The number of ways to win the big race is {}",
        parse_big_race(input).count_win_conditions()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(total_margin(parse_races(input)), 288);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(parse_big_race(input).count_win_conditions(), 71503);
    }
}
