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

struct Grid {
    grid: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

impl Grid {
    fn new() -> Self {
        Grid { grid: Vec::new() }
    }

    fn new_from_size(x: usize, y: usize) -> Self {
        Grid {
            grid: vec![vec!['.'; x]; y],
        }
    }

    fn get_value(&self, point: Point) -> Option<char> {
        self.grid
            .get(point.y as usize)?
            .get(point.x as usize)
            .copied()
    }

    fn set_value(&mut self, point: Point, val: char) {
        self.grid[point.y as usize][point.x as usize] = val;
    }

    fn insert(&mut self, new: Vec<char>) {
        self.grid.insert(0, new);
    }

    fn print(&self) {
        for row in self.grid.iter().rev() {
            println!("{}", row.iter().collect::<String>())
        }
    }

    fn expand(&self) -> Grid {
        let mut new_grid = Grid::new();
        for line in self.grid.iter() {
            let mut line0 = String::new();
            let mut line1 = String::new();
            let mut line2 = String::new();
            for ch in line.iter() {
                match ch {
                    'S' => {
                        line0.push_str(".|.");
                        line1.push_str("-S-");
                        line2.push_str(".|.");
                    }
                    'F' => {
                        line0.push_str("...");
                        line1.push_str(".F-");
                        line2.push_str(".|.");
                    }
                    'L' => {
                        line0.push_str(".|.");
                        line1.push_str(".L-");
                        line2.push_str("...");
                    }
                    'J' => {
                        line0.push_str(".|.");
                        line1.push_str("-J.");
                        line2.push_str("...");
                    }
                    '7' => {
                        line0.push_str("...");
                        line1.push_str("-7.");
                        line2.push_str(".|.");
                    }
                    '-' => {
                        line0.push_str("...");
                        line1.push_str("---");
                        line2.push_str("...");
                    }
                    '|' => {
                        line0.push_str(".|.");
                        line1.push_str(".|.");
                        line2.push_str(".|.");
                    }
                    _ => {
                        line0.push_str("...");
                        line1.push_str("...");
                        line2.push_str("...");
                    }
                }
            }
            new_grid.grid.push(line2.chars().collect());
            new_grid.grid.push(line1.chars().collect());
            new_grid.grid.push(line0.chars().collect());
        }
        return new_grid;
    }

    fn reduce(&self) -> Grid {
        Grid {
            grid: self
                .grid
                .as_slice()
                .windows(3)
                .step_by(3)
                .map(|line| {
                    line[1]
                        .as_slice()
                        .windows(3)
                        .step_by(3)
                        .map(|x| x[1])
                        .collect()
                })
                .collect(),
        }
    }
}

impl Point {
    fn step(&self, heading: Heading) -> Point {
        match heading {
            Heading::North => Point {
                x: self.x,
                y: self.y + 1,
            },
            Heading::South => Point {
                x: self.x,
                y: self.y - 1,
            },
            Heading::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Heading::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

fn make_grid(input: &str) -> Grid {
    let lines = input.lines();

    let mut grid: Grid = Grid::new();

    for line in lines {
        grid.insert(line.chars().collect());
    }
    return grid;
}

fn find_start(grid: &Grid) -> Option<Point> {
    for (y, line) in grid.grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 'S' {
                return Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    return None;
}

fn find_first_heading(grid: &Grid, start: Point) -> Option<Heading> {
    if grid.get_value(start.step(Heading::North)).is_some()
        && "|7F".contains(grid.get_value(start.step(Heading::North)).unwrap())
    {
        return Some(Heading::North);
    } else if grid.get_value(start.step(Heading::South)).is_some()
        && "|LJ".contains(grid.get_value(start.step(Heading::South)).unwrap())
    {
        return Some(Heading::South);
    } else if grid.get_value(start.step(Heading::East)).is_some()
        && "-J7".contains(grid.get_value(start.step(Heading::East)).unwrap())
    {
        return Some(Heading::East);
    } else if grid.get_value(start.step(Heading::West)).is_some()
        && "-LF".contains(grid.get_value(start.step(Heading::West)).unwrap())
    {
        return Some(Heading::West);
    }
    return None;
}

fn calculate_loop_length(grid: &Grid, start: Point, mut new_grid: Option<&mut Grid>) -> usize {
    let mut length = 0;
    let mut cur_pos = start;
    let mut heading = find_first_heading(&grid, start).expect("Should find a heading");

    loop {
        if let Some(ref mut new_grid) = new_grid {
            new_grid.set_value(
                cur_pos,
                grid.get_value(cur_pos).expect("Should be valid pos"),
            );
        }
        cur_pos = cur_pos.step(heading);
        let next_val = grid.get_value(cur_pos).expect("Should be valid pos");
        heading = match heading {
            Heading::North => match next_val {
                '|' => Heading::North,
                '7' => Heading::West,
                'F' => Heading::East,
                _ => Heading::North,
            },
            Heading::South => match next_val {
                '|' => Heading::South,
                'J' => Heading::West,
                'L' => Heading::East,
                _ => Heading::South,
            },
            Heading::East => match next_val {
                '-' => Heading::East,
                'J' => Heading::North,
                '7' => Heading::South,
                _ => Heading::East,
            },
            Heading::West => match next_val {
                '-' => Heading::West,
                'L' => Heading::North,
                'F' => Heading::South,
                _ => Heading::West,
            },
        };
        length += 1;

        if cur_pos == start {
            break;
        }
    }

    return length;
}

fn is_outside_heading(grid: &Grid, point: Point, heading: Heading) -> bool {
    if let Some(p2) = grid.get_value(point.step(heading)) {
        if p2 == 'O' {
            return true;
        }
    } else {
        return true;
    }
    return false;
}

fn is_outside(grid: &Grid, point: Point) -> bool {
    is_outside_heading(&grid, point, Heading::North)
        || is_outside_heading(&grid, point, Heading::South)
        || is_outside_heading(&grid, point, Heading::East)
        || is_outside_heading(&grid, point, Heading::West)
}

fn process(input: &str) -> u64 {
    let grid = make_grid(input);

    let start = find_start(&grid).expect("There should be one point");

    let mut new_grid = Grid::new_from_size(grid.grid[0].len(), grid.grid.len());

    calculate_loop_length(&grid, start, Some(&mut new_grid));

    // new_grid.print();

    let mut expanded = new_grid.expand();

    // expanded.print();

    let mut found_any = true;

    while found_any {
        found_any = false;
        for x in 0..expanded.grid[0].len() {
            for y in 0..expanded.grid.len() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };

                if let Some(value) = expanded.get_value(point) {
                    if value == '.' {
                        if is_outside(&expanded, point) {
                            expanded.set_value(point, 'O');
                            found_any = true;
                        }
                    }
                }
            }
        }
    }

    // expanded.print();

    let reduced = expanded.reduce();

    // reduced.print();

    return reduced.grid.concat().iter().filter(|x| **x == '.').count() as u64;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_process() {
//         let input = " ";
//         assert_eq!(1, process(input))
//     }
// }
