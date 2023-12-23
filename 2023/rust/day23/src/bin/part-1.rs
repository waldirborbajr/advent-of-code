fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

type Route = ((usize, usize), Vec<(usize, usize)>);

pub fn process(content: &str) -> usize {
    let input: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let grid = input
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Since the grid is quite constrained, we can just brute force it.
    let start_pos: Route = ((1, 0), Vec::new());
    let mut routes: Vec<Route> = vec![start_pos];
    let mut final_routes: Vec<Route> = Vec::new();

    while let Some(current_route) = routes.pop() {
        let mut current_pos = current_route.0;
        let mut path = current_route.1.clone();
        let mut new_steps = Vec::new();

        if current_pos.0 == grid[0].len() - 2 && current_pos.1 == grid.len() - 1 {
            final_routes.push(current_route);
            continue;
        }

        match grid[current_pos.1][current_pos.0] {
            '>' => {
                current_pos.0 += 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            }
            '<' => {
                current_pos.0 -= 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            }
            '^' => {
                current_pos.1 -= 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            }
            'v' => {
                current_pos.1 += 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            }
            _ => {
                if current_pos.0 > 0
                    && grid[current_pos.1][current_pos.0 - 1] != '#'
                    && !path.contains(&(current_pos.0 - 1, current_pos.1))
                {
                    new_steps.push((current_pos.0 - 1, current_pos.1));
                }
                if current_pos.0 < grid[0].len() - 1
                    && grid[current_pos.1][current_pos.0 + 1] != '#'
                    && !path.contains(&(current_pos.0 + 1, current_pos.1))
                {
                    new_steps.push((current_pos.0 + 1, current_pos.1));
                }
                if current_pos.1 > 0
                    && grid[current_pos.1 - 1][current_pos.0] != '#'
                    && !path.contains(&(current_pos.0, current_pos.1 - 1))
                {
                    new_steps.push((current_pos.0, current_pos.1 - 1));
                }
                if current_pos.1 < grid.len() - 1
                    && grid[current_pos.1 + 1][current_pos.0] != '#'
                    && !path.contains(&(current_pos.0, current_pos.1 + 1))
                {
                    new_steps.push((current_pos.0, current_pos.1 + 1));
                }
            }
        }

        for step in new_steps {
            let mut new_path = path.clone();
            new_path.push(step);
            routes.push((step, new_path));
        }
    }

    final_routes.iter().map(|r| r.1.len()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
        assert_eq!(94, process(input))
    }
}
