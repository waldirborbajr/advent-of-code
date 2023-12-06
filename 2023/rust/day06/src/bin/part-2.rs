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

fn process(input: &str) -> usize {
    let input: Vec<String> = input
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect();

    let time: u64 = input[0].split_whitespace().collect::<Vec<&str>>()[1..]
        .to_vec()
        .join("")
        .parse()
        .unwrap();

    let record: u64 = input[1].split_whitespace().collect::<Vec<&str>>()[1..]
        .to_vec()
        .join("")
        .parse()
        .unwrap();

    let mut total = 0;
    for i in 0..time {
        if (time - i) * i > record {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
        let input = r#"Time:      71530
Distance:  940200 "#;
        assert_eq!(71503, process(input));
    }
}
