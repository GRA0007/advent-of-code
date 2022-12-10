use std::{fs, str};

enum Command {
    Addx(isize),
    Noop,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            v if v.starts_with("addx") => {
                Ok(Self::Addx(v.split_once(' ').unwrap().1.parse().unwrap()))
            }
            "noop" => Ok(Self::Noop),
            _ => Err("Couldn't parse command"),
        }
    }
}

struct Cpu {
    commands: Vec<Command>,
}

impl From<Vec<Command>> for Cpu {
    fn from(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}

impl Cpu {
    /// Runs an amount of cycles and returns the crt characters and the X value
    fn run_cycles(&self, count: isize) -> (Vec<char>, isize) {
        let mut commands = self.commands.iter();
        let mut x = 1;
        let mut crt: Vec<char> = Vec::new();
        let mut addx: Option<isize> = None;

        for i in 0..count - 1 {
            crt.push(if x - 1 <= i % 40 && x + 1 >= i % 40 {
                '#'
            } else {
                '.'
            });

            if let Some(v) = addx {
                x += v;
                addx = None;
            } else {
                match commands.next().unwrap() {
                    Command::Addx(v) => addx = Some(*v),
                    Command::Noop => {}
                }
            }
        }

        (crt, x)
    }

    fn sum_strength(&self) -> isize {
        const CYCLES: [isize; 6] = [20, 60, 100, 140, 180, 220];

        CYCLES
            .into_iter()
            .map(|count| self.run_cycles(count).1 * count)
            .sum()
    }

    fn draw_crt(&self) -> String {
        let pixels = self.run_cycles(241).0;

        pixels
            .chunks(40)
            .flat_map(|section| [section, &['\n']].concat())
            .collect()
    }
}

fn main() {
    let cpu: Cpu = parse_input("input.txt").into();
    println!("The sum of signal strengths is {}", cpu.sum_strength());
    println!("{}", cpu.draw_crt());
}

fn parse_input(filename: &str) -> Vec<Command> {
    let input = fs::read_to_string(filename).unwrap();

    input
        .trim()
        .lines()
        .map(|line| line.try_into().unwrap())
        .collect()
}

#[cfg(test)]
#[test]
fn test() {
    let cpu: Cpu = parse_input("test.txt").into();
    assert_eq!(cpu.sum_strength(), 13140);
    assert_eq!(
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n",
        cpu.draw_crt()
    );
}

#[cfg(test)]
#[test]
fn test_small() {
    let cpu: Cpu = parse_input("test_small.txt").into();
    let values = (1..=5)
        .map(|cycle| cpu.run_cycles(cycle).1)
        .collect::<Vec<_>>();
    assert_eq!(values.get(1), Some(&1));
    assert_eq!(values.get(4), Some(&4));
}
