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

pub fn process(input: &str) -> u32 {
    let lines = input.lines();

    let mut sum_valid: u32 = 0;
    for line in lines {
        let splitted_line = line.split(" ").collect::<Vec<&str>>();
        let springs: Vec<char> = splitted_line[0].chars().collect::<Vec<char>>();

        let ss = splitted_line[1].split(",").collect::<Vec<&str>>();
        let mut spring_sizes = Vec::new();
        for s in ss {
            spring_sizes.push(s.parse::<u32>().unwrap());
        }

        // let spring_sizes: Vec<u32> = splitted_line[1]
        //     .split(",")
        //     .map(|s| s.parse::<u32>().unwrap());

        // Find all valid arrangements
        let valid_arrangements_springs: u32 = valid_arrangements(springs, spring_sizes);
        sum_valid += valid_arrangements_springs;
    }

    sum_valid
}

fn is_valid(springs: Vec<char>, spring_sizes: Vec<u32>) -> bool {
    let mut curr_broken = 0;
    let mut curr_size_i = 0;

    for c in springs.clone() {
        // Count consecutive broken springs
        if c == '#' {
            curr_broken += 1;
        }
        // Check if found broken springs match the next spring size
        else if curr_broken > 0 {
            if curr_size_i >= spring_sizes.len() {
                return false;
            }
            if curr_broken != spring_sizes[curr_size_i] {
                return false;
            }
            curr_size_i += 1;
            curr_broken = 0;
        }
    }

    // Check last sequence of broken springs
    if curr_broken > 0 {
        if curr_size_i >= spring_sizes.len() {
            return false;
        }
        if curr_broken != spring_sizes[curr_size_i] {
            return false;
        }
        curr_size_i += 1;
    }

    // Check if all spring sizes are used
    if spring_sizes.len() != curr_size_i {
        return false;
    }
    return true;
}

fn valid_arrangements(springs: Vec<char>, spring_sizes: Vec<u32>) -> u32 {
    // Check if all unknowns are filled, check validity
    if springs.iter().all(|&c| c != '?') {
        if is_valid(springs, spring_sizes) {
            return 1;
        } else {
            return 0;
        }
    }

    // Find first unknown, start recursion
    let mut valids = 0;
    for i in 0..springs.len() {
        if springs[i] == '?' {
            let mut springs_copy = springs.clone();
            springs_copy[i] = '#';
            valids += valid_arrangements(springs_copy, spring_sizes.clone());
            let mut springs_copy_2 = springs.clone();
            springs_copy_2[i] = '.';
            valids += valid_arrangements(springs_copy_2, spring_sizes.clone());
            break;
        }
    }

    valids
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
        assert_eq!(21, process(input))
    }
}
