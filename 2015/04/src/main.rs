fn main() {
    let answer = calculate_answer("ckczppom", 5);
    println!("The answer is {answer}.");
}

fn calculate_answer(secret_key: &str, number_of_zeroes: usize) -> usize {
    println!("Calculating for {number_of_zeroes} zeroes...");

    let mut answer: usize = 1;

    let zeroes = "0".repeat(number_of_zeroes);
    loop {
        let hash = format!("{:x}", md5::compute(format!("{secret_key}{answer}")));
        if hash[0..number_of_zeroes] == zeroes {
            println!("Found hash that matches requirement: {hash}");
            return answer;
        }
        answer += 1;
    }
}
