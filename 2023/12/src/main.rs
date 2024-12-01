enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Unknown spring"),
        }
    }
}

struct ConditionRecords(Vec<(Vec<Spring>, Vec<usize>)>);

impl From<String> for ConditionRecords {
    fn from(value: String) -> Self {
        Self(
            value
                .lines()
                .map(|line| {
                    let (springs, groups) = line.trim().split_once(' ').unwrap();
                    let springs = springs.chars().map(Spring::from).collect();
                    let groups = groups
                        .split(',')
                        .map(|group| group.parse().unwrap())
                        .collect();
                    (springs, groups)
                })
                .collect(),
        )
    }
}

fn main() {
    println!("Hello, world!");
}
