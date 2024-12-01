use std::fs;

use aoc_2015_05::*;

fn main() {
    let strings = fs::read_to_string("input.txt").expect("Failed to read input");
    let nice_count: usize = strings
        .lines()
        .take_while(|line| !line.is_empty())
        .map(nice)
        .filter(|nice| *nice)
        .count();
    println!("There are {} nice words in Santa's list", nice_count);
}
