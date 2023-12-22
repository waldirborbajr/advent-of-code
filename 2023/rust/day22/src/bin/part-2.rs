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

use std::collections::{HashMap, HashSet};

type Pos = (i16, i16, i16);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

#[derive(Debug, Clone)]
struct Brick {
    cubes: Vec<Pos>,
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let (a_str, b_str) = s.split_once('~').unwrap();

        let mut iter = a_str.split(',').map(|n| n.parse::<i16>().unwrap());
        let a = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        let mut iter = b_str.split(',').map(|n| n.parse::<i16>().unwrap());
        let b = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let cubes = if a.0 != b.0 {
            let min = a.0.min(b.0);
            let max = a.0.max(b.0);
            (min..=max).map(|v| (v, a.1, a.2)).collect()
        } else if a.1 != b.1 {
            let min = a.1.min(b.1);
            let max = a.1.max(b.1);
            (min..=max).map(|v| (a.0, v, a.2)).collect()
        } else if a.2 != b.2 {
            let min = a.2.min(b.2);
            let max = a.2.max(b.2);
            (min..=max).map(|v| (a.0, a.1, v)).collect()
        } else {
            vec![a]
        };
        Brick { cubes }
    }
}

fn fall(bricks: &mut [Brick]) -> Vec<usize> {
    let mut moved = vec![];
    let down = (0, 0, -1);
    let map: HashMap<Pos, usize> = bricks
        .iter()
        .enumerate()
        .flat_map(|(idx, b)| b.cubes.iter().map(move |c| (*c, idx)))
        .collect();

    for (idx, b) in bricks.iter_mut().enumerate() {
        if b.cubes
            .iter()
            .all(|&c| c.2 != 1 && map.get(&add(down, c)).copied().unwrap_or(idx) == idx)
        {
            moved.push(idx);
            b.cubes.iter_mut().for_each(|c| *c = add(down, *c));
        }
    }

    moved
}

pub fn process(input: &str) -> usize {
    let mut bricks: Vec<_> = input.lines().map(Brick::from_str).collect();

    while !fall(&mut bricks).is_empty() {}

    let mut cnt = 0;

    for i in 0..bricks.len() {
        let mut test = bricks.clone();
        test.swap_remove(i);

        let mut list = HashSet::new();

        loop {
            let fell = fall(&mut test);
            if fell.is_empty() {
                break;
            }
            list.extend(fell);
        }

        cnt += list.len();
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#""#;
        assert_eq!(400, process(input))
    }
}
