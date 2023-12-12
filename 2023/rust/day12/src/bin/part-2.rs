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
    input
        .lines()
        .filter_map(|s| {
            let (s, rest) = s.split_once(' ')?;

            let b = s.as_bytes();
            let mut s = Vec::with_capacity((s.len() + 1) * 5);
            for r in 0..5 {
                s.extend_from_slice(b);
                s.push(if r != 4 { b'?' } else { b'.' });
            }

            let mut xs: Vec<i32> = rest.split(',').filter_map(|x| x.parse().ok()).collect();

            let n = xs.len();
            for _ in 0..4 {
                xs.extend_from_within(..n);
            }

            Some(solve(&s, &xs))
        })
        .sum()
}

fn solve(s: &[u8], xs: &[i32]) -> i64 {
    let n = s.len();
    let m = xs.len();

    // (parsed position, number of groups of #, last group len)
    let x = n + 1;
    let y = m + 2;
    let z = n + 2;
    let mut dp = vec![0i64; x * y * z]; // [i,j,k] = (i * y * z) + (j * z) + k;
    dp[0] = 1;

    for i in 0..n {
        for j in 0..=m {
            for k in 0..=n {
                let curr = dp[i * y * z + j * z + k]; // [i, j, k]
                if curr == 0 {
                    continue;
                }
                // current group is zero or of required length -> continue without a group
                if (s[i] == b'.' || s[i] == b'?') && (k == 0 || k == (xs[j - 1] as usize)) {
                    dp[(i + 1) * y * z + j * z] += curr; // [i + 1, j, 0]
                }
                // continue the previous group or start a new one
                if s[i] == b'#' || s[i] == b'?' {
                    let nz = if k == 0 { 1 } else { 0 };
                    dp[(i + 1) * y * z + (j + nz) * z + (k + 1)] += curr; // [i + 1, j + nz, k + 1]
                }
            }
        }
    }

    // [n, m, 0] when we parsed all n inputs and matched all m group-lengths
    dp[n * y * z + m * z]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "?###???????? 3,2,1
.###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#";
        assert_eq!(525152, process(input))
    }
}
