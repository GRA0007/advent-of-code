use std::fs;

struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn new(range_str: &str) -> Self {
        let mut numbers = range_str.split('-').map(|x| x.parse().unwrap());
        Range {
            from: numbers.next().unwrap(),
            to: numbers.next().unwrap(),
        }
    }

    fn contains(&self, range: &Range) -> bool {
        self.from <= range.from && self.to >= range.to
    }

    fn overlaps(&self, range: &Range) -> bool {
        (self.from <= range.to && self.to >= range.to)
            || (self.to >= range.from && self.to <= range.to)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let input = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut ranges = line.split(',');
            (
                Range::new(ranges.next().unwrap()),
                Range::new(ranges.next().unwrap()),
            )
        });

    let fully_contained = input
        .clone()
        .map(|(range_1, range_2)| range_1.contains(&range_2) || range_2.contains(&range_1))
        .filter(|x| *x)
        .count();

    println!(
        "There are {} assignment pairs where one range fully includes the other",
        fully_contained
    );

    let overlapping = input
        .map(|(range_1, range_2)| range_1.overlaps(&range_2))
        .filter(|x| *x)
        .count();

    println!("There are {} assignment pairs that overlap", overlapping);
}
