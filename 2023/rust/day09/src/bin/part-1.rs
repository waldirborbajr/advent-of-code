use itertools::Itertools;

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

pub fn process(input: &str) -> i32 {
    let sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect();
    let res = sequences
        .iter()
        .map(|orig_seq| {
            let mut seq = orig_seq.clone();
            let mut intermediate_seqs = vec![];
            loop {
                let diffs = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
                if diffs.iter().all(|&x| x == 0) {
                    break;
                }

                intermediate_seqs.push(diffs.clone());
                seq = diffs;
            }
            let final_diff = intermediate_seqs
                .iter()
                .map(|v| v.last().unwrap())
                .sum::<i32>();
            (orig_seq, final_diff)
        })
        .map(|(seq, diff)| seq.last().unwrap() + diff)
        .sum::<i32>();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input))
    }
}
