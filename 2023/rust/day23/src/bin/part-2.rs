fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input.lines().collect());
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn directions() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

use Direction::*;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

use Tile::*;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
struct Node {
    i: usize,
    j: usize,
}

impl From<(usize, usize)> for Node {
    fn from(value: (usize, usize)) -> Self {
        Node {
            i: value.0,
            j: value.1,
        }
    }
}

pub fn process(lines: Vec<&str>) -> String {
    let board: Vec<Vec<Tile>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Path,
                    '#' => Forest,
                    '^' => Slope(Up),
                    'v' => Slope(Down),
                    '<' => Slope(Left),
                    '>' => Slope(Right),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let start = (
        0,
        board
            .first()
            .unwrap()
            .iter()
            .position(|&tile| tile == Path)
            .unwrap(),
    );

    let end = (
        board.len() - 1,
        board
            .last()
            .unwrap()
            .iter()
            .position(|&tile| tile == Path)
            .unwrap(),
    );

    let mut nodes: HashSet<Node> = HashSet::from([start.into(), end.into()]);
    let mut weights: HashMap<(Node, Node), i64> = HashMap::new();

    let mut visited: HashSet<Node> = HashSet::from([start.into()]);
    let mut stack: Vec<(Node, Node, i64)> = vec![(start.into(), (start.0 + 1, start.1).into(), 1)];

    while let Some((from, cur, steps)) = stack.pop() {
        if cur == from {
            continue;
        }

        if cur == end.into() {
            weights.insert((from, cur), steps);
            weights.insert((cur, from), steps);
            continue;
        }

        let next_nodes: Vec<Node> = Direction::directions()
            .into_iter()
            .filter_map(|direction| {
                let (di, dj) = direction.offset();
                let next = (
                    cur.i.checked_add_signed(di as isize).unwrap(),
                    cur.j.checked_add_signed(dj as isize).unwrap(),
                );

                if board[next.0][next.1] != Forest {
                    Some(next.into())
                } else {
                    None
                }
            })
            .collect();

        // cur is not passage
        if next_nodes.len() != 2 {
            weights.insert((from, cur), steps);
            weights.insert((cur, from), steps);
            nodes.insert(cur);
        }

        if visited.contains(&cur.into()) {
            continue;
        }

        visited.insert(cur);

        for &next in &next_nodes {
            match next_nodes.len() {
                2 => stack.push((from, next.into(), steps + 1)),
                _ => stack.push((cur, next.into(), 1)),
            }
        }
    }

    let mut stack: Vec<(Node, HashSet<Node>, i64)> =
        vec![(start.into(), HashSet::from([start.into()]), 0)];

    let mut result = 0;

    while let Some((cur, visited, steps)) = stack.pop() {
        for (next, weight) in weights
            .iter()
            .filter_map(|(&(node_from, node_to), &weight)| match node_from {
                x if x == cur => Some((node_to, weight)),
                _ => None,
            })
        {
            if next == end.into() {
                if steps + weight > result {
                    result = steps + weight;
                    continue;
                }
            }

            if visited.contains(&next) {
                continue;
            }

            let mut next_visited = visited.clone();
            next_visited.insert(next);

            stack.push((next, next_visited, steps + weight));
        }
    }

    result.to_string()
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
        assert_eq!("154", process(input.lines().collect()))
    }
}
