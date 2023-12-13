fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 2 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!(
        "took {}ms ({}us)  ",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
}

pub fn process(input: &str) -> usize {
    todo!("comming soon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "";
        assert_eq!(400, process(input))
    }
}
