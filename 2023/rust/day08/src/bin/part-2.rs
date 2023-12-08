use std::collections::HashMap;

use num::Integer;
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

pub fn process(input: &str) -> u128 {
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

    let cur: Vec<&String> = instructions.keys().filter(|p| p.ends_with("A")).collect();

    let val: u128 = cur
        .into_iter()
        .map(|s| {
            let mut i: u128 = 0;
            let mut cur = s;
            'label: loop {
                for char in first_line.chars() {
                    // println!("{cur:?}");
                    if cur.ends_with("Z") {
                        break 'label;
                    }
                    i += 1;
                    if char == 'L' {
                        cur = &instructions.get(cur).unwrap().0
                    } else if char == 'R' {
                        cur = &instructions.get(cur).unwrap().1
                    } else {
                        panic!("Unknown symbol")
                    }
                }
            }
            i
        })
        .fold(1, |l, r| l.lcm(&r));

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(1, process(input))
    }
}
