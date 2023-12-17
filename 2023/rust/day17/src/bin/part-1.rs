use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::Neg,
};

fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input).unwrap();
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

use Direction::*;

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            UP => DOWN,
            RIGHT => LEFT,
            DOWN => UP,
            LEFT => RIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
    moves_vertically: bool,
}

impl Node {
    fn new(x: usize, y: usize, moves_vertically: bool) -> Self {
        Node {
            x,
            y,
            moves_vertically,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| (c as u8 - '0' as u8) as usize).collect())
        .collect()
}

pub fn process(input: &str) -> Option<usize> {
    let map = parse_input(input);

    let w = map[0].len();
    let h = map.len();
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::<Node>::new();
    let starth = Node::new(0, 0, false);
    let startv = Node::new(0, 0, true);
    pq.push(Reverse((0, starth)));
    pq.push(Reverse((0, startv)));

    while !pq.is_empty() {
        let Reverse((distance, node)) = pq.pop().unwrap();

        if node.x == map[0].len() - 1 && node.y == map.len() - 1 {
            return Some(distance);
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for dir in [LEFT, RIGHT, UP, DOWN] {
            if node.moves_vertically {
                if dir == UP || dir == DOWN {
                    continue;
                }
            } else if dir == LEFT || dir == RIGHT {
                continue;
            }

            let mut delta = 0;
            for i in 1..4 {
                let x = match dir {
                    LEFT => node.x.checked_sub(i),
                    RIGHT => {
                        if node.x + i >= w {
                            None
                        } else {
                            Some(node.x + i)
                        }
                    }
                    _ => Some(node.x),
                };

                let y = match dir {
                    UP => node.y.checked_sub(i),
                    DOWN => {
                        if node.y + i >= h {
                            None
                        } else {
                            Some(node.y + i)
                        }
                    }
                    _ => Some(node.y),
                };

                if x.is_none() || y.is_none() {
                    break;
                }

                let x = x.unwrap();
                let y = y.unwrap();

                delta += map[y][x];

                pq.push(Reverse((
                    distance + delta,
                    Node::new(x, y, !node.moves_vertically),
                )));
            }
        }
    }

    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_process() {
//         let input = "2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533";
//         assert_eq!(102, process(input))
//     }
// }
