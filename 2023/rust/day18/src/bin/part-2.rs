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

pub fn process(input: &str) -> i64 {
    let instructions: Vec<(char, i64)> = parse_input(input.to_string());

    let mut vertices = vec![(0, 0)];
    let mut current_position = (0, 0);

    for (direction, distance) in instructions {
        match direction {
            'R' => current_position.0 += distance,
            'L' => current_position.0 -= distance,
            'U' => current_position.1 += distance,
            'D' => current_position.1 -= distance,
            _ => panic!("Invalid direction"),
        }
        vertices.push(current_position);
    }

    let (area, boundary_points) = calculate_area_and_boundary(&vertices);
    let interior_points = area - boundary_points / 2 + 1;
    let a = interior_points + boundary_points;
    println!("{a}");
    a
}

fn calculate_area_and_boundary(vertices: &[(i64, i64)]) -> (i64, i64) {
    let mut area = 0;
    let mut boundary_points = 0;
    let n = vertices.len();

    for i in 0..n - 1 {
        area += vertices[i].0 * vertices[i + 1].1;
        area -= vertices[i + 1].0 * vertices[i].1;
        boundary_points += gcd(
            (vertices[i].0 - vertices[i + 1].0).abs(),
            (vertices[i].1 - vertices[i + 1].1).abs(),
        );
    }

    area += vertices[n - 1].0 * vertices[0].1;
    area -= vertices[0].0 * vertices[n - 1].1;
    boundary_points += gcd(
        (vertices[n - 1].0 - vertices[0].0).abs(),
        (vertices[n - 1].1 - vertices[0].1).abs(),
    );
    ((area.abs() / 2) as i64, boundary_points)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn parse_input(input: String) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            let hex = parts
                .last()
                .unwrap()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_start_matches('#');

            let distance_hex = &hex[..5];
            let direction_hex = hex.chars().last().unwrap();

            let distance = i64::from_str_radix(distance_hex, 16).unwrap();
            let direction = match direction_hex {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!(),
            };

            (direction, distance)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "R 6 (#70c710)
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
        U 2 (#7a21e3)";
        assert_eq!(952408144115, process(input))
    }
}
