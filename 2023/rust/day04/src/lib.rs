pub fn process_part1(input: &str) -> u32 {
 r  todo!()
}

pub fn process_part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT01: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT01);
        assert_eq!(result, 4361);
    }

    #[test]
    // #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT01);
        assert_eq!(result, 467835);
    }
}
