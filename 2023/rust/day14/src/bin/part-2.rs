use std::collections::HashMap;
use std::{fmt, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let starttime = std::time::Instant::now();

    let answer = process(input.clone(), 1_000_000_000);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rock {
    Cube,
    Rounded,
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    rocks: Vec<Vec<Option<Rock>>>,
    cache: HashMap<Vec<Vec<Option<Rock>>>, u64>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rocks
                .iter()
                .map(|r| {
                    format!(
                        "{}\n",
                        r.iter()
                            .map(|c| {
                                match c {
                                    Some(Rock::Cube) => '#',
                                    Some(Rock::Rounded) => 'O',
                                    None => '.',
                                }
                            })
                            .collect::<String>()
                    )
                })
                .collect::<String>()
        )
    }
}

impl Grid {
    fn new(input: String) -> Grid {
        let rocks: Vec<Vec<Option<Rock>>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Some(Rock::Cube),
                        'O' => Some(Rock::Rounded),
                        '.' => None,
                        _ => panic!("Unexpected character: {}", c),
                    })
                    .collect()
            })
            .collect();
        Grid {
            width: rocks[0].len(),
            height: rocks.len(),
            rocks,
            cache: HashMap::new(),
        }
    }

    fn roll_north(&mut self) {
        for x in 0..self.width {
            let mut current_empty_spot = 0;
            for y in 0..self.height {
                match self.rocks[y][x] {
                    Some(Rock::Cube) if y < self.height - 1 => {
                        current_empty_spot = y + 1;
                    }
                    Some(Rock::Rounded) => {
                        assert!(current_empty_spot < self.height);
                        self.rocks[y][x] = None;
                        self.rocks[current_empty_spot][x] = Some(Rock::Rounded);
                        if current_empty_spot < self.height - 1 {
                            current_empty_spot += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn roll_south(&mut self) {
        for x in 0..self.width {
            let mut current_empty_spot = self.height - 1;
            for y in (0..self.height).rev() {
                match self.rocks[y][x] {
                    Some(Rock::Cube) if y > 0 => {
                        current_empty_spot = y - 1;
                    }
                    Some(Rock::Rounded) => {
                        self.rocks[y][x] = None;
                        self.rocks[current_empty_spot][x] = Some(Rock::Rounded);
                        current_empty_spot = current_empty_spot.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for y in 0..self.height {
            let mut current_empty_spot = 0;
            for x in 0..self.width {
                match self.rocks[y][x] {
                    Some(Rock::Cube) if x < self.width - 1 => {
                        current_empty_spot = x + 1;
                    }
                    Some(Rock::Rounded) => {
                        assert!(current_empty_spot < self.width);
                        self.rocks[y][x] = None;
                        self.rocks[y][current_empty_spot] = Some(Rock::Rounded);
                        if current_empty_spot < self.width - 1 {
                            current_empty_spot += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn roll_east(&mut self) {
        for y in 0..self.height {
            let mut current_empty_spot = self.width - 1;
            for x in (0..self.width).rev() {
                match self.rocks[y][x] {
                    Some(Rock::Cube) if x > 0 => {
                        current_empty_spot = x - 1;
                    }
                    Some(Rock::Rounded) => {
                        self.rocks[y][x] = None;
                        self.rocks[y][current_empty_spot] = Some(Rock::Rounded);
                        current_empty_spot = current_empty_spot.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }

    fn cycle(&mut self, iterations: u64) {
        for cycles in 1..=iterations {
            self.roll_north();
            self.roll_west();
            self.roll_south();
            self.roll_east();

            let seen_rocks = self.cache.get(&self.rocks);
            if let Some(previous_cycles) = seen_rocks {
                // Found loop
                let loop_length = cycles - previous_cycles;
                let iterations_left = iterations - cycles;
                if iterations_left % loop_length == 0 {
                    break;
                }
            }

            self.cache.insert(self.rocks.clone(), cycles);
        }
    }

    fn load_north(&self) -> u64 {
        self.rocks
            .iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.iter().map(move |rock| match rock {
                    Some(Rock::Rounded) => (self.height - y) as u64,
                    _ => 0,
                })
            })
            .sum()
    }
}

pub fn process(input: String, iterations: u64) -> u64 {
    let mut grid = Grid::new(input);
    grid.cycle(iterations);
    grid.load_north()
}

// #[cfg(test)]
// #[ignore]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_process() {
//         let input = "O....#....
// O.OO#....#
// .....##...
// OO.#O....O
// .O.....O#.
// O.#..O.#.#
// ..O..#O..O
// .......O..
// #....###..
// #OO..#....";
//         assert_eq!(136, process(input.clone()))
//     }
// }
