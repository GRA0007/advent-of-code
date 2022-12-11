use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

#[derive(Debug)]
enum Operation {
    Add(OpNumber),
    Multiply(OpNumber),
}

#[derive(Debug)]
enum OpNumber {
    Current,
    Number(usize),
}

impl TryFrom<(&str, &str)> for Operation {
    type Error = &'static str;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let number = match value.1 {
            "old" => OpNumber::Current,
            x => OpNumber::Number(x.parse().unwrap()),
        };
        match value.0 {
            "+" => Ok(Self::Add(number)),
            "*" => Ok(Self::Multiply(number)),
            _ => Err("Unknown operation"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Rc<RefCell<Vec<usize>>>,
    op: Operation,
    divisible_test: usize,
    if_true: usize,
    if_false: usize,
}

/// Yes this is the word for a group of monkeys
struct Troop {
    monkeys: Vec<Monkey>,
    lcm: usize,
}

impl From<Vec<Monkey>> for Troop {
    fn from(monkeys: Vec<Monkey>) -> Self {
        Troop {
            lcm: monkeys.iter().map(|monkey| monkey.divisible_test).product(),
            monkeys,
        }
    }
}

impl Troop {
    /// Runs an amount of rounds and returns the number of times each
    /// monkey inspected an item
    fn run_rounds(&mut self, count: usize, worried: bool) -> HashMap<usize, usize> {
        let mut inspections: HashMap<usize, usize> = HashMap::new();

        for _ in 0..count {
            for (i, monkey) in self.monkeys.iter().enumerate() {
                for item in &*monkey.items.borrow() {
                    // Monkey inspects item and increases worry level
                    let mut item = match &monkey.op {
                        Operation::Add(num) => match num {
                            OpNumber::Current => item + item,
                            OpNumber::Number(x) => item + x,
                        },
                        Operation::Multiply(num) => match num {
                            OpNumber::Current => item * item,
                            OpNumber::Number(x) => item * x,
                        },
                    };

                    // Record the inspection
                    inspections.entry(i).and_modify(|v| *v += 1).or_insert(1);

                    // Monkey gets bored with item
                    if worried {
                        // Take the remainder of the worry level divided by the
                        // lowest common multiple of all the divisibility checks
                        item %= self.lcm;
                    } else {
                        item = (item as f64 / 3_f64).floor() as usize;
                    }

                    // Check where to pass the item
                    let target = if item % monkey.divisible_test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };
                    self.monkeys[target].items.borrow_mut().push(item);
                }
                monkey.items.borrow_mut().clear();
            }
        }

        inspections
    }

    fn calculate_monkey_business(&mut self, rounds: usize, worried: bool) -> usize {
        let inspections = self.run_rounds(rounds, worried);
        let mut sorted: Vec<usize> = inspections.values().cloned().collect();
        sorted.sort_unstable();
        sorted.into_iter().rev().take(2).product()
    }
}

fn main() {
    let mut monkeys: Troop = parse_input("input.txt").into();
    println!(
        "The amount of monkey business is {}",
        monkeys.calculate_monkey_business(20, false)
    );

    let mut monkeys: Troop = parse_input("input.txt").into();
    println!(
        "The amount of monkey business after 10000 rounds (you're very worried!) is {}",
        monkeys.calculate_monkey_business(10000, true)
    );
}

#[cfg(test)]
#[test]
fn test_part_1() {
    let mut monkeys: Troop = parse_input("test.txt").into();
    assert_eq!(monkeys.calculate_monkey_business(20, false), 10605);
}

#[cfg(test)]
#[test]
fn test_part_2() {
    let mut monkeys: Troop = parse_input("test.txt").into();
    assert_eq!(monkeys.calculate_monkey_business(10000, true), 2713310158);
}

fn parse_input(file_name: &str) -> Vec<Monkey> {
    let input = fs::read_to_string(file_name).unwrap();

    input
        .trim()
        .split("\n\n")
        .map(|monkey| {
            let mut monkey = monkey.lines().skip(1);

            Monkey {
                items: Rc::new(RefCell::new(
                    monkey
                        .next()
                        .unwrap()
                        .replace("  Starting items: ", "")
                        .split(", ")
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>(),
                )),

                op: monkey
                    .next()
                    .unwrap()
                    .replace("  Operation: new = old ", "")
                    .split_once(' ')
                    .unwrap()
                    .try_into()
                    .unwrap(),

                divisible_test: monkey
                    .next()
                    .unwrap()
                    .replace("  Test: divisible by ", "")
                    .parse()
                    .unwrap(),

                if_true: monkey
                    .next()
                    .unwrap()
                    .replace("    If true: throw to monkey ", "")
                    .parse()
                    .unwrap(),

                if_false: monkey
                    .next()
                    .unwrap()
                    .replace("    If false: throw to monkey ", "")
                    .parse()
                    .unwrap(),
            }
        })
        .collect()
}
