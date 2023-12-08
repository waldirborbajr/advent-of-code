use std::collections::HashMap;

use regex::Regex;

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

pub fn process(input: &str) -> i64 {
    let first_line = input.lines().next().unwrap().to_string();

    let instructions = Regex::new(r"(?m)^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();

    let instructions: HashMap<String, (String, String)> = instructions
        .captures_iter(&input)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().to_string(),
                (
                    c.get(2).unwrap().as_str().to_string(),
                    c.get(3).unwrap().as_str().to_string(),
                ),
            )
        })
        .collect();

    let mut i = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        for char in first_line.chars() {
            i += 1;
            // println!("{cur}");
            if cur == "ZZZ" {
                break;
            }
            if char == 'L' {
                cur = &instructions.get(cur).unwrap().0
            } else if char == 'R' {
                cur = &instructions.get(cur).unwrap().1
            } else {
                panic!("Unknown symbol")
            }
        }
    }

    // dbg!(i);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input))
    }
}
