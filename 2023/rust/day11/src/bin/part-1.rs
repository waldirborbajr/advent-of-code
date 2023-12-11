fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!(
        "took {}ms ({}us)  ",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
}

pub fn process(input: &str) -> u64 {
    let empty_rows: Vec<u64> = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().filter(|c| c != &'.').count() == 0)
        .map(|(i, _)| i as u64)
        .collect();
    let empty_cols: Vec<u64> = {
        let mut iter = input.lines();
        let mut cols: Vec<char> = iter.next().unwrap().chars().collect();
        while let Some(current) = iter.next() {
            for (x, c) in current.chars().enumerate() {
                if c != '.' {
                    cols[x] = '#';
                }
            }
        }

        cols.iter()
            .enumerate()
            .filter(|(_, c)| **c == '.')
            .map(|(i, _)| i as u64)
            .collect()
    };

    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let row_len = map.len();

    for (off, i) in empty_rows.iter().enumerate() {
        map.insert(*i as usize + off, vec!['.'; row_len])
    }

    for (off, i) in empty_cols.iter().enumerate() {
        for x in 0..row_len + empty_rows.len() {
            map[x].insert(*i as usize + off, '.');
        }
    }

    let points: Vec<(u64, u64)> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(|(i, _)| (i as u64, y as u64))
                .collect::<Vec<(u64, u64)>>()
        })
        .filter(|line| !line.is_empty())
        .fold(Vec::new(), |mut v, mut l| {
            v.append(&mut l);
            v
        });

    let len = points.len();
    let mut sum = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            sum += (points[i].0 as i64 - points[j].0 as i64).abs()
                + (points[i].1 as i64 - points[j].1 as i64).abs();
        }
    }
    //for row in map {
    //    for c in row {
    //        print!("{}", c);
    //    }
    //    print!("\n");
    //}
    //println!("{:?}", points);

    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, process(input))
    }
}
