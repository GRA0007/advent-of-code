use std::fs;

struct Action {
    amount: usize,
    from: usize,
    to: usize,
}

enum CrateMoverModel {
    CrateMover9000,
    CrateMover9001,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Parse input
    let stacks = parse_stacks(input.lines().take_while(|line| !line.is_empty()).collect());
    let actions = parse_actions(
        input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .collect(),
    );

    let top_of_stacks = run_actions(&stacks, &actions, CrateMoverModel::CrateMover9000);
    println!("The top of all stacks is: {top_of_stacks}");

    let top_of_stacks_9001 = run_actions(&stacks, &actions, CrateMoverModel::CrateMover9001);
    println!("The top of all stacks using model 9001 is: {top_of_stacks_9001}");
}

fn run_actions(stacks: &[Vec<char>], actions: &Vec<Action>, model: CrateMoverModel) -> String {
    // Clone stacks
    let mut stacks = stacks.to_owned();

    // Run actions
    for action in actions {
        // Grab the crates to move
        let mut gripper: Vec<char> = (0..action.amount)
            .map(|_| stacks[action.from - 1].pop().unwrap())
            .collect();

        if matches!(model, CrateMoverModel::CrateMover9001) {
            gripper.reverse();
        }

        // Deposit in their new home
        gripper
            .into_iter()
            .for_each(|item| stacks[action.to - 1].push(item));
    }

    // Find out what's on top of each stack
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn parse_stacks(stacks: Vec<&str>) -> Vec<Vec<char>> {
    let number_of_stacks = stacks.last().unwrap().split_whitespace().count();
    let chars: Vec<Vec<char>> = stacks
        .iter()
        .take(stacks.len() - 1)
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line_chars| {
            line_chars
                .chunks(4)
                .map(|col| col.get(1).unwrap().to_owned())
                .collect::<Vec<char>>()
        })
        .collect();

    (0..number_of_stacks)
        .map(|i| {
            chars
                .iter()
                .map(|row| row[i])
                .filter(|x| !x.is_whitespace())
                .rev()
                .collect()
        })
        .collect()
}

fn parse_actions(actions: Vec<&str>) -> Vec<Action> {
    actions
        .iter()
        .map(|action| {
            let values: Vec<usize> = action
                .split_whitespace()
                .filter(|x| x != &"move" && x != &"from" && x != &"to")
                .map(|x| x.parse().unwrap())
                .collect();
            Action {
                amount: values[0],
                from: values[1],
                to: values[2],
            }
        })
        .collect()
}
