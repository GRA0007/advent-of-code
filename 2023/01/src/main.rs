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
    /// Get the first (or last) digit from a line, and its index
    fn first_digit(&self, backwards: bool) -> Option<(usize, usize)> {
        if backwards {
            self.0.char_indices().rfind(|(_, c)| c.is_ascii_digit())
        } else {
            self.0.char_indices().find(|(_, c)| c.is_ascii_digit())
        }
        .map(|(index, digit)| (index, digit.to_digit(10).unwrap() as usize))
    }

    /// Get the first (or last) number word from a line, and its index
    fn first_word(&self, backwards: bool) -> Option<(usize, usize)> {
        // Calculate word indexes
        let mut word_indexes: Vec<(usize, usize)> = DIGITS
            .iter()
            .enumerate()
            .filter_map(|(i, digit_str)| {
                if backwards {
                    self.0.rmatch_indices(digit_str).next()
                } else {
                    self.0.match_indices(digit_str).next()
                }
                .map(|(index, _)| (index, i + 1))
            })
            .collect();

        word_indexes.sort_by(|a, b| a.0.cmp(&b.0));

        if backwards {
            word_indexes.last()
        } else {
            word_indexes.first()
        }
        .copied()
    }

    fn first(&self, backwards: bool) -> usize {
        let first_word = self.first_word(backwards);
        let first_digit = self.first_digit(backwards);

        if let Some((word_index, word_number)) = first_word {
            if let Some((digit_index, digit_number)) = first_digit {
                return if word_index < digit_index {
                    if backwards {
                        digit_number.to_string().parse().unwrap()
                    } else {
                        word_number
                    }
                } else if backwards {
                    word_number
                } else {
                    digit_number.to_string().parse().unwrap()
                };
            }
            return word_number;
        } else if let Some((_, digit_number)) = first_digit {
            return digit_number.to_string().parse().unwrap();
        };
        unreachable!();
    }

    fn get_calibration_number(&self) -> usize {
        let first_digit = self.first(false);
        let last_digit = self.first(true);

        format!("{first_digit}{last_digit}")
            .parse::<usize>()
            .unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(Line::from);

    let calibration_total: usize = lines
        .clone()
        .map(|line| {
            let first_digit = line.first_digit(false).unwrap().1;
            let last_digit = line.first_digit(true).unwrap().1;

            format!("{first_digit}{last_digit}")
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    println!("Total is {}", calibration_total);

    let calibration_total: usize = lines.map(|line| line.get_calibration_number()).sum();
    println!("Total including words is {}", calibration_total);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        assert_eq!(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
                .lines()
                .map(Line::from)
                .map(|line| {
                    let first_digit = line.first_digit(false).unwrap().1;
                    let last_digit = line.first_digit(true).unwrap().1;

                    format!("{first_digit}{last_digit}")
                        .parse::<usize>()
                        .unwrap()
                })
                .sum::<usize>(),
            142
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
                .lines()
                .map(Line::from)
                .map(|line| line.get_calibration_number())
                .sum::<usize>(),
            281
        )
    }
}
