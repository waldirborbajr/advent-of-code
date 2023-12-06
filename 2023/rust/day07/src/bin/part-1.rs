fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!(
        "took {}ms ({}us)  ",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
}

fn process(input: &str) -> u64 {
    let input: Vec<String> = input
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect(); //

    let times: Vec<u32> = input[0].split_whitespace().collect::<Vec<&str>>()[1..]
        .to_vec()
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let records: Vec<u32> = input[1].split_whitespace().collect::<Vec<&str>>()[1..]
        .to_vec()
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut total = 1;
    for (record, time) in times.iter().enumerate() {
        let mut win = 0;
        for i in 0..*time {
            if (time - i) * i > records[record] {
                win += 1;
            }
        }
        total *= win;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200
       "#;
        assert_eq!(288, process(input));
    }
}
