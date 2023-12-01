pub fn process_part1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
            if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
            } else {
                0
            }
        })
        .sum();

    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let output = input.lines().map(process_line).sum::<u32>();

    output.to_string()
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT01: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT02: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT01);
        assert_eq!(result, "142");
    }

    #[test]
    // #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT02);
        assert_eq!(result, "281");
    }
}
