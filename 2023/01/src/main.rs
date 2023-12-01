use std::fs;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct Line(String);

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Line(value.to_owned())
    }
}

impl Line {
    fn get_digit(&self, backwards: bool) -> usize {
        // Calculate word indexes
        let mut word_indexes: Vec<(usize, usize)> = DIGITS
            .iter()
            .enumerate()
            .filter_map(|(i, digit_str)| {
                if backwards {
                    self.0
                        .find(digit_str.chars().as_str())
                        .map(|index| (index, i + 1))
                } else {
                    self.0
                        .rfind(digit_str.chars().as_str())
                        .map(|index| (index, i + 1))
                }
            })
            .collect();
        word_indexes.sort_by(|a, b| a.0.cmp(&b.0));
        let first_word = if backwards {
            word_indexes.last()
        } else {
            word_indexes.first()
        };

        // Calculate number indexes
        let first_digit = if backwards {
            self.0.char_indices().rfind(|(_, c)| c.is_ascii_digit())
        } else {
            self.0.char_indices().find(|(_, c)| c.is_ascii_digit())
        };

        if let Some((word_index, word_number)) = first_word {
            if let Some((digit_index, digit_number)) = first_digit {
                return if word_index < &digit_index {
                    if backwards {
                        digit_number.to_string().parse().unwrap()
                    } else {
                        *word_number
                    }
                } else if backwards {
                    *word_number
                } else {
                    digit_number.to_string().parse().unwrap()
                };
            }
            return *word_number;
        } else if let Some((_, digit_number)) = first_digit {
            return digit_number.to_string().parse().unwrap();
        };
        panic!("Uh oh");
    }

    fn get_calibration_number(&self) -> usize {
        let first_digit = self.get_digit(false);
        let last_digit = self.get_digit(true);
        format!("{first_digit}{last_digit}")
            .parse::<usize>()
            .unwrap()
    }
}

fn main() {
    let calibration_total: usize = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Line::from)
        .map(|line| line.get_calibration_number())
        .sum();

    println!("Total is {}", calibration_total);
}
