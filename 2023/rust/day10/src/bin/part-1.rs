fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    process(input);
    // let answer = process(input);
    // println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!(
        "took {}ms ({}us)  ",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
}
fn get_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}

fn find_loop(grid: &Vec<Vec<char>>, start: &(usize, usize), dir: (i8, i8)) -> Option<usize> {
    let mut steps = 0;

    let mut x = start.0;
    let mut y = start.1;
    let mut dir = dir;

    loop {
        steps += 1;
        match dir {
            (0, 1) => y += 1,
            (0, -1) => y -= 1,
            (1, 0) => x += 1,
            (-1, 0) => x -= 1,
            _ => panic!("Unknown direction: {:?}", dir),
        }

        match grid[y][x] {
            '|' => {
                if dir.1 == 0 {
                    return None;
                }
            }
            '-' => {
                if dir.0 == 0 {
                    return None;
                }
            }
            'L' => match dir {
                (0, 1) => dir = (1, 0),
                (-1, 0) => dir = (0, -1),
                _ => return None,
            },
            'J' => match dir {
                (0, 1) => dir = (-1, 0),
                (1, 0) => dir = (0, -1),
                _ => return None,
            },
            '7' => match dir {
                (0, -1) => dir = (-1, 0),
                (1, 0) => dir = (0, 1),
                _ => return None,
            },
            'F' => match dir {
                (0, -1) => dir = (1, 0),
                (-1, 0) => dir = (0, 1),
                _ => return None,
            },
            '.' => return None,
            'S' => return Some(steps),
            _ => panic!("Unknown tile: {}", grid[y][x]),
        }
    }
}
pub fn process(input: &str) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = get_start(&grid);

    for dir in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
        if let Some(steps) = find_loop(&grid, &start, dir) {
            println!("{}", steps / 2);
            return;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[ignore]
//     fn part1_process() {
//         let input = " ";
//         assert_eq!(2, process(input))
//     }
// }
