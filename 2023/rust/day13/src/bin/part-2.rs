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

fn get_horizontale_reflection_value(mirror: &Vec<String>) -> usize {
    for i in 0..mirror.len() - 1 {
        let mut ni = i + 1;
        let mut ri: isize = i as isize;
        let mut reflecting = true;
        let mut smudge_error = 1;

        while ri >= 0 && ni < mirror.len() {
            for j in 0..mirror[0].len() {
                if mirror[ri as usize].chars().nth(j).unwrap() != mirror[ni].chars().nth(j).unwrap()
                {
                    if smudge_error == 0 {
                        reflecting = false;
                        break;
                    } else {
                        smudge_error -= 1;
                    }
                }
            }
            if reflecting == true {
                ri -= 1;
                ni += 1;
            } else {
                break;
            }
        }

        if smudge_error == 0 && (ri < 0 || ni >= mirror.len()) {
            return (1 + i) * 100;
        }
    }
    0
}

fn get_verticale_reflection_value(mirror: &Vec<String>) -> usize {
    for j in 0..(mirror[0].len() - 1) {
        let mut reflecting = true;
        let mut smudge_error = 1;
        let mut nj = j + 1;
        let mut rj: isize = j as isize;

        while rj >= 0 && nj < mirror[0].len() {
            for i in 0..mirror.len() {
                if mirror[i].chars().nth(rj as usize).unwrap() != mirror[i].chars().nth(nj).unwrap()
                {
                    if smudge_error == 0 {
                        reflecting = false;
                        break;
                    } else {
                        smudge_error -= 1;
                    }
                }
            }
            if reflecting == true {
                nj += 1;
                rj -= 1;
            } else {
                break;
            }
        }
        if smudge_error == 0 && (rj < 0 || nj >= mirror[0].len()) {
            return 1 + j;
        }
    }

    0
}

fn get_mirror_value(mirror: &Vec<String>) -> usize {
    let mut res = 0;
    res += get_verticale_reflection_value(mirror);
    res += get_horizontale_reflection_value(mirror);

    res
}

pub fn process(input: &str) -> usize {
    let mut mirrors: Vec<Vec<String>> = vec![];

    let mut current_mirror: Vec<String> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            mirrors.push(current_mirror);
            current_mirror = vec![];
        } else {
            current_mirror.push(line.to_string());
        }
    }
    mirrors.push(current_mirror);

    let mut acc = 0;
    for mirror in mirrors {
        acc += get_mirror_value(&mirror);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(400, process(input))
    }
}
