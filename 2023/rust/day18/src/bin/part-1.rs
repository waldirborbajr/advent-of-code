fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

pub fn process(input: &str) -> usize {
    let mut sum = 0;
    let instructions: Vec<(char, i32, String)> = parse_input(input.to_string());
    let (grid, start_x, start_y) = make_empty_grid(&instructions);

    let mut grid = grid;
    let mut current_position = (start_x, start_y);

    for instruction in instructions {
        dig(&mut current_position, &mut grid, instruction);
    }

    flood_fill_outside(&mut grid);

    for line in grid.iter() {
        for c in line {
            if c == &'#' || c == &'.' {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
    sum
}

fn flood_fill_outside(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];

    for i in 0..rows {
        dfs(i, 0, grid, &mut visited);
        dfs(i, cols - 1, grid, &mut visited);
    }
    for j in 0..cols {
        dfs(0, j, grid, &mut visited);
        dfs(rows - 1, j, grid, &mut visited);
    }
}

fn dfs(row: usize, col: usize, grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
    if row >= grid.len() || col >= grid[0].len() || visited[row][col] || grid[row][col] == '#' {
        return;
    }
    visited[row][col] = true;
    grid[row][col] = 'o';

    dfs(row.wrapping_sub(1), col, grid, visited);
    dfs(row + 1, col, grid, visited);
    dfs(row, col.wrapping_sub(1), grid, visited);
    dfs(row, col + 1, grid, visited);
}

fn make_empty_grid(instructions: &Vec<(char, i32, String)>) -> (Vec<Vec<char>>, i32, i32) {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, 0, 0, 0);
    let (mut x, mut y) = (0, 0);

    for (direction, distance, _) in instructions {
        match direction {
            'U' => y -= distance,
            'D' => y += distance,
            'L' => x -= distance,
            'R' => x += distance,
            _ => panic!("Invalid direction"),
        }
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    let width = (x_max - x_min + 1) as usize;
    let height = (y_max - y_min + 1) as usize;

    let grid = vec![vec!['.'; width]; height];

    let start_x = -x_min;
    let start_y = -y_min;

    (grid, start_x, start_y)
}

fn dig(
    current_position: &mut (i32, i32),
    grid: &mut Vec<Vec<char>>,
    instruction: (char, i32, String),
) {
    let (direction, length, _) = instruction;

    for _ in 0..length {
        match direction {
            'R' => current_position.0 += 1,
            'L' => current_position.0 -= 1,
            'U' => current_position.1 -= 1,
            'D' => current_position.1 += 1,
            _ => panic!("Invalid direction"),
        }

        if current_position.0 >= 0
            && current_position.1 >= 0
            && (current_position.1 as usize) < grid.len()
            && (current_position.0 as usize) < grid[0].len()
        {
            grid[current_position.1 as usize][current_position.0 as usize] = '#';
        } else {
            panic!("Position out of bounds: {:?}", current_position);
        }
    }
}

fn parse_input(input: String) -> Vec<(char, i32, String)> {
    let mut out: Vec<(char, i32, String)> = Vec::new();

    for line in input.lines() {
        let split: Vec<_> = line.trim().split(" ").collect();
        let char = split[0].parse::<char>().unwrap();
        let numb = split[1].parse::<i32>().unwrap();
        let hex = split[2].to_string();
        out.push((char, numb, hex))
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        assert_eq!(62, process(input))
    }
}
