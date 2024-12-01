use std::fs;

struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn build(dimensions: &str) -> Self {
        let dimensions: Vec<usize> = dimensions.split('x').map(|d| d.parse().unwrap()).collect();
        Self {
            length: dimensions[0],
            width: dimensions[1],
            height: dimensions[2],
        }
    }

    fn calculate_paper_required(&self) -> usize {
        let sides = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let smallest = sides.iter().min().unwrap();
        sides.iter().map(|side| side * 2).sum::<usize>() + smallest
    }

    fn calculate_ribbon_required(&self) -> usize {
        let mut sides = vec![self.length * 2, self.width * 2, self.height * 2];
        sides.sort_unstable();
        let wrap_length: usize = sides.into_iter().take(2).sum();
        let bow_length = self.length * self.width * self.height;
        wrap_length + bow_length
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let presents: Vec<Present> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(Present::build)
        .collect();

    let paper_required: usize = presents
        .iter()
        .map(|present| present.calculate_paper_required())
        .sum();

    let ribbon_required: usize = presents
        .iter()
        .map(|present| present.calculate_ribbon_required())
        .sum();

    println!("The total amount of paper required is: {paper_required}");
    println!("The total amount of ribbon required is: {ribbon_required}");
}
