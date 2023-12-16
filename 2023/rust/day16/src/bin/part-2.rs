use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 2 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

pub fn process(input: &str) -> usize {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    let max_x = grid[0].len();
    let max_y = grid.len();

    let part2 =
        (0..max_y)
            .map(|y| get_energize_count(&grid, Direction::E, (0, y)))
            .chain(
                (0..max_y)
                    .map(|y| get_energize_count(&grid, Direction::W, (0, max_y - 1 - y)))
                    .chain(
                        (0..max_x)
                            .map(|x| get_energize_count(&grid, Direction::S, (x, 0)))
                            .chain((0..max_x).map(|x| {
                                get_energize_count(&grid, Direction::N, (max_x - x - 1, 0))
                            })),
                    ),
            )
            .max()
            .unwrap();
    part2
}

type Grid = Vec<Vec<char>>;

// Direction of beam travel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn get_next_cells(
    grid: &Grid,
    entry_dir: Direction,
    cell: (usize, usize),
) -> Vec<(Direction, (usize, usize))> {
    let (x, y) = cell;
    let this = grid[y][x];
    let max_x = grid[0].len();
    let max_y = grid.len();
    match this {
        // Keep going if empty or a parallel splitter
        '.' => get_neighbour(x, y, entry_dir, max_x, max_y)
            .map(|pos| (entry_dir, pos))
            .into_iter()
            .collect(),
        '|' if entry_dir == Direction::N || entry_dir == Direction::S => {
            get_neighbour(x, y, entry_dir, max_x, max_y)
                .map(|pos| (entry_dir, pos))
                .into_iter()
                .collect()
        }
        '-' if entry_dir == Direction::E || entry_dir == Direction::W => {
            get_neighbour(x, y, entry_dir, max_x, max_y)
                .map(|pos| (entry_dir, pos))
                .into_iter()
                .collect()
        }
        // Bend
        '/' => {
            let new_dir = match entry_dir {
                Direction::N => Direction::E,
                Direction::E => Direction::N,
                Direction::S => Direction::W,
                Direction::W => Direction::S,
            };
            get_neighbour(x, y, new_dir, max_x, max_y)
                .map(|pos| (new_dir, pos))
                .into_iter()
                .collect()
        }
        '\\' => {
            let new_dir = match entry_dir {
                Direction::N => Direction::W,
                Direction::E => Direction::S,
                Direction::S => Direction::E,
                Direction::W => Direction::N,
            };
            get_neighbour(x, y, new_dir, max_x, max_y)
                .map(|pos| (new_dir, pos))
                .into_iter()
                .collect()
        }
        // Splits.
        '-' => get_neighbour(x, y, Direction::E, max_x, max_y)
            .map(|pos| (Direction::E, pos))
            .into_iter()
            .chain(get_neighbour(x, y, Direction::W, max_x, max_y).map(|pos| (Direction::W, pos)))
            .collect(),
        '|' => get_neighbour(x, y, Direction::N, max_x, max_y)
            .map(|pos| (Direction::N, pos))
            .into_iter()
            .chain(get_neighbour(x, y, Direction::S, max_x, max_y).map(|pos| (Direction::S, pos)))
            .collect(),
        _ => panic!("Invalid char"),
    }
}

fn get_neighbour(
    x: usize,
    y: usize,
    dir: Direction,
    max_x: usize,
    max_y: usize,
) -> Option<(usize, usize)> {
    match dir {
        Direction::N => {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        }
        Direction::W => {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        }
        Direction::S => {
            if y < max_y - 1 {
                Some((x, y + 1))
            } else {
                None
            }
        }
        Direction::E => {
            if x < max_x - 1 {
                Some((x + 1, y))
            } else {
                None
            }
        }
    }
}

fn get_energize_count(grid: &Grid, start_dir: Direction, start_pos: (usize, usize)) -> usize {
    let mut beams = vec![(start_dir, start_pos)];
    let mut visited = HashSet::new();
    while !beams.is_empty() {
        beams = beams
            .iter()
            .flat_map(|(dir, pos)| {
                // Move on if we haven't arrived at this cell from this direction
                if visited.insert((*dir, *pos)) {
                    get_next_cells(grid, *dir, *pos)
                } else {
                    vec![]
                }
            })
            .collect();
    }
    // We want to de-deuplciate on direction, so re-combine into a HashSet that drops the direction,
    // and then take its size to get the energized cell count.
    // (We could separately be building up the two hashSets, but perf is good enough).
    visited
        .iter()
        .map(|(_, pos)| pos)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#".|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v.."#;
        assert_eq!(51, process(input))
    }
}
