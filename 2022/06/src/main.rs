use std::{collections::HashSet, fs};

struct Datastream<'a>(&'a str);

impl Datastream<'_> {
    fn start_of_packet(&self, unique_after: usize) -> Option<usize> {
        for i in 0..self.0.len() - unique_after {
            let buffer: HashSet<char> = HashSet::from_iter(self.0[i..i + unique_after].chars());
            if buffer.len() == unique_after {
                return Some(i + unique_after);
            }
        }
        None
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let ds = Datastream(input.trim());
    let start = ds.start_of_packet(4).unwrap();
    println!("The start of the packet is at {start}");
    let start_of_message = ds.start_of_packet(14).unwrap();
    println!("The start of the message is at {start_of_message}");
}
