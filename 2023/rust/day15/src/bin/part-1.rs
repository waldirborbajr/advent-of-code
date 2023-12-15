fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

pub fn process(input: &str) -> i32 {
    let input = input.trim();
    input
        .split(',')
        .map(|step| hash_algorithm(step.as_bytes()) as i32)
        .sum()
}

fn hash_algorithm(chars: &[u8]) -> u8 {
    chars
        .iter()
        .fold(0, |acc, &c| acc.overflowing_add(c).0.overflowing_mul(17).0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, process(input))
    }
}
