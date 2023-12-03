use std::collections::HashMap;
use std::collections::HashSet;

pub fn process_part1(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let h = map.len();
    let w = map[0].len();

    let mut symbols_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut numbers = Vec::new();

    // well, in rust you cannot iterate over -1..=1 range..
    let offsets: Vec<i32> = vec![-1, 0, 1];

    for y in 0..h {
        let mut x = 0;
        while x < w {
            let v = map[y][x];
            if v != '.' && !v.is_numeric() {
                for dx in &offsets {
                    for dy in &offsets {
                        let px = (x as i32 + dx).clamp(0, w as i32);
                        let py = (y as i32 + dy).clamp(0, h as i32);
                        symbols_pos.insert((px as usize, py as usize));
                    }
                }
                x += 1;
            } else if v.is_numeric() {
                // parse number
                let mut n = v.to_digit(10).unwrap() as u32;
                let mut positions = vec![(x, y)];
                positions.push((x, y));
                x += 1;
                while x < w && map[y][x].is_numeric() {
                    n = n * 10 + map[y][x].to_digit(10).unwrap() as u32;
                    positions.push((x, y));
                    x += 1;
                }
                numbers.push((n, positions));
            } else {
                x += 1;
            }
        }
    }

    let result = numbers
        .into_iter()
        .filter_map(|(value, positions)| {
            for pos in positions {
                if symbols_pos.contains(&pos) {
                    return Some(value);
                }
            }
            None
        })
        .sum();
    result
}

pub fn process_part2(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let h = map.len();
    let w = map[0].len();

    let mut symbols_pos: HashMap<(usize, usize), i32> = HashMap::new();
    let mut numbers = Vec::new();

    // well, in rust you cannot iterate over -1..=1 range..
    let offsets: Vec<i32> = vec![-1, 0, 1];

    let mut gear_number = 0;
    for y in 0..h {
        let mut x = 0;
        while x < w {
            let v = map[y][x];
            if v == '*' {
                for dx in &offsets {
                    for dy in &offsets {
                        let px = (x as i32 + dx).clamp(0, w as i32);
                        let py = (y as i32 + dy).clamp(0, h as i32);
                        symbols_pos.insert((px as usize, py as usize), gear_number);
                    }
                }
                x += 1;
                gear_number += 1;
            } else if v.is_numeric() {
                // parse number
                let mut n = v.to_digit(10).unwrap() as u32;
                let mut positions = vec![(x, y)];
                positions.push((x, y));
                x += 1;
                while x < w && map[y][x].is_numeric() {
                    n = n * 10 + map[y][x].to_digit(10).unwrap() as u32;
                    positions.push((x, y));
                    x += 1;
                }
                numbers.push((n, positions));
            } else {
                x += 1;
            }
        }
    }

    let mut gears_num: HashMap<i32, Vec<u32>> = HashMap::new();

    numbers.into_iter().for_each(|(value, positions)| {
        for pos in positions {
            let gear = symbols_pos.get(&pos);
            match gear {
                Some(id) => {
                    gears_num
                        .entry(*id)
                        .and_modify(|v| v.push(value))
                        .or_insert_with(|| vec![value]);
                    break;
                }
                None => {}
            }
        }
    });

    let result = gears_num
        .iter()
        .filter_map(|(key, values)| {
            if values.len() != 2 {
                return None;
            }
            Some(values.iter().product::<u32>())
        })
        .sum();

    result
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
