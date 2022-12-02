use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut bags: Vec<usize> = Vec::new();
    let mut subtotal: usize = 0;
    for line in input.lines() {
        if let Ok(value) = line.parse::<usize>() {
            subtotal += value;
        } else {
            bags.push(subtotal);
            subtotal = 0;
        }
    }

    bags.sort_unstable_by(|a, b| b.cmp(a));

    println!("The largest bag contains: {} calories", bags[0]);

    let largest3: usize = bags.iter().take(3).sum();
    println!("The largest 3 bags have: {} calories", largest3);
}
