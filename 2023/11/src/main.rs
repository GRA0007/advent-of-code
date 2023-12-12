use std::fs;

use tqdm::Iter;

fn manhatten_distance(coords: ((usize, usize), (usize, usize))) -> usize {
    (((coords.0 .0 as isize) - (coords.1 .0 as isize)).abs()
        + ((coords.0 .1 as isize) - (coords.1 .1 as isize)).abs()) as usize
}

struct Image {
    galaxies: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl From<String> for Image {
    fn from(value: String) -> Self {
        let mut galaxies = Vec::new();

        for (y, line) in value.lines().enumerate() {
            for (x, char) in line.trim().char_indices() {
                if char == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        Self {
            galaxies,
            height: value.lines().count(),
            width: value.lines().next().unwrap().trim().len(),
        }
    }
}

impl Image {
    fn expand(&mut self, age: usize) {
        let empty_cols: Vec<_> = (0..self.width)
            .filter(|x| self.galaxies.iter().all(|galaxy| galaxy.0 != *x))
            .collect();

        let empty_rows: Vec<_> = (0..self.height)
            .filter(|y| self.galaxies.iter().all(|galaxy| galaxy.1 != *y))
            .collect();

        self.galaxies = self
            .galaxies
            .clone()
            .into_iter()
            .map(|galaxy| {
                (
                    galaxy.0 + empty_cols.iter().filter(|x| x < &&galaxy.0).count() * (age - 1),
                    galaxy.1 + empty_rows.iter().filter(|y| y < &&galaxy.1).count() * (age - 1),
                )
            })
            .collect();
    }

    fn find_galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        self.galaxies.iter().tqdm().fold(Vec::new(), |pairs, from| {
            vec![
                pairs.clone(),
                self.galaxies
                    .iter()
                    .filter_map(|to| {
                        if from == to
                            || pairs.contains(&(*from, *to))
                            || pairs.contains(&(*to, *from))
                        {
                            None
                        } else {
                            Some((*from, *to))
                        }
                    })
                    .collect::<Vec<_>>(),
            ]
            .into_iter()
            .flatten()
            .collect()
        })
    }

    fn sum_shortest_paths(&self) -> usize {
        self.find_galaxy_pairs()
            .into_iter()
            .map(manhatten_distance)
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut image = Image::from(input.clone());
    image.expand(2);

    println!(
        "The sum of all the shortest paths is {}",
        image.sum_shortest_paths()
    );

    let mut large_image = Image::from(input);
    large_image.expand(1_000_000);

    println!(
        "The sum of all the shortest paths in a larger image is {}",
        large_image.sum_shortest_paths()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut image = Image::from(input);
        image.expand(2);
        assert_eq!(image.sum_shortest_paths(), 374);
    }

    #[test]
    fn part_2_10() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut image = Image::from(input);
        image.expand(10);
        assert_eq!(image.sum_shortest_paths(), 1030);
    }

    #[test]
    fn part_2_100() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut image = Image::from(input);
        image.expand(100);
        assert_eq!(image.sum_shortest_paths(), 8410);
    }
}
