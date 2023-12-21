use polyfit_rs::polyfit_rs::polyfit;
use std::collections::{BTreeMap, BTreeSet};

fn wrap(x: isize, y: isize, width: isize, height: isize) -> (isize, isize) {
    let wrap_x = if x < 0 {
        width - ((x + 1).abs() % width) - 1
    } else {
        x % width
    };

    let wrap_y = if y < 0 {
        height - ((y + 1).abs() % height) - 1
    } else {
        y % height
    };

    (wrap_x, wrap_y)
}

fn process(input: &str, steps: isize) -> isize {
    let mut grid: BTreeMap<(isize, isize), char> = BTreeMap::new();
    let mut start: (isize, isize) = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert((x as isize, y as isize), c);
            if c == 'S' {
                start = (x as isize, y as isize);
                grid.insert((x as isize, y as isize), '.');
            }
        });
    });

    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    assert_eq!(width, height);

    let mut values: Vec<usize> = vec![];
    let mut plots: BTreeSet<(isize, isize)> = BTreeSet::new();
    plots.insert(start);
    for step in 0..steps as isize {
        let mut next_plots: BTreeSet<(isize, isize)> = BTreeSet::new();
        for (x, y) in plots.iter() {
            let neighbors = vec![(*x, *y - 1), (*x, *y + 1), (*x - 1, *y), (*x + 1, *y)];
            for (nx, ny) in neighbors {
                let (wrap_x, wrap_y) = wrap(nx, ny, width, height);
                let neighbor = (wrap_x, wrap_y);
                if grid.contains_key(&neighbor) {
                    if grid.get(&neighbor).unwrap() == &'.' {
                        next_plots.insert((nx, ny));
                    }
                }
            }
        }
        plots = next_plots;
        let offset = start.0 - 1;
        if step == offset {
            values.push(plots.len());
        }
        if step == offset + width {
            values.push(plots.len());
        }
        if step == offset + 2 * width {
            values.push(plots.len());
            break;
        }
    }

    let x_values = vec![0.0, 1.0, 2.0];
    let y_values = values.iter().map(|x| *x as f64).collect::<Vec<_>>();
    let poly = polyfit(&x_values, &y_values, 2).unwrap();
    let poly = poly
        .iter()
        .map(|x| (x + 0.5).floor() as isize)
        .collect::<Vec<_>>();

    let n = (steps - start.0) / width;

    poly[2] * n * n + poly[1] * n + poly[0]
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input, 26501365));
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_process() {
//         let input = "";
//         assert_eq!(400, process(input))
//     }
// }
