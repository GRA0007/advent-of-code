use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const TIME_LIMIT: usize = 30;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: usize,
    leads_to: Vec<String>,
}

impl Valve {
    /// Open a valve and return it's total flow rate
    fn open(&self, time_remaining: usize) -> usize {
        self.flow_rate * time_remaining
    }
}

#[derive(Debug)]
struct Volcano {
    start: String,
    valves: HashMap<String, Valve>,
}

impl Volcano {
    /// Find the best path to the destination valve
    fn flow_rate_between(&self) -> Option<usize> {
        // if time_remaining == 0 {
        //     return None;
        // }

        // let dest_valve = self.valves.get(&dest).unwrap();

        // if self.valves.get(&source).unwrap().leads_to.contains(&dest) {
        //     return Some(dest_valve.open(time_remaining));
        // }

        for (label, valve) in self.valves {
            let mut flow_rate
        }

        let mut time_remaining = TIME_LIMIT;
        let mut flow_rate = 0;
        let mut visited: HashSet<String> = vec![self.start.clone()].into_iter().collect();
        let mut queue: VecDeque<Valve> = vec![self.valves.get(&self.start).unwrap().clone()].into();

        while let Some(valve) = queue.pop_front() {
            time_remaining -= 1;
            // If time is up
            if time_remaining == 0 {
                return Some(flow_rate);
            }

            for tunnel in valve.leads_to {
                if !visited.contains(&tunnel) {
                    time_remaining -= 1;
                    flow_rate += self.valves.get(&tunnel).unwrap().open(time_remaining);
                    queue.push_back(self.valves.get(&tunnel).unwrap().clone());
                    visited.insert(tunnel);
                }
            }
        }
        None
    }
}

fn main() {
    println!("Hello, world!");
}

fn parse_input(file_name: &str) -> Volcano {
    let input = fs::read_to_string(file_name).unwrap();

    let valves: HashMap<String, Valve> = input
        .trim()
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let label = words.nth(1).unwrap().to_string();
            let flow_rate: usize = words
                .nth(2)
                .unwrap()
                .replace(';', "")
                .split_once('=')
                .unwrap()
                .1
                .parse()
                .unwrap();
            let leads_to: Vec<String> = words.skip(4).map(|l| l.replace(',', "")).collect();
            (
                label,
                Valve {
                    flow_rate,
                    leads_to,
                },
            )
        })
        .collect();

    let start = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_string();

    Volcano { start, valves }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_1() {
        let volcano = parse_input("test.txt");
        dbg!(volcano);
        assert_eq!(1, 0);
    }
}
