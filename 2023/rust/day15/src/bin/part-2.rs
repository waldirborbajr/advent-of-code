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

#[derive(Debug, Clone)]
struct LensHashMap<'a> {
    lens_boxes: [Vec<(&'a str, i32)>; 256],
}

impl<'a> LensHashMap<'a> {
    fn new() -> Self {
        Self {
            lens_boxes: [(); 256].map(|_| Vec::new()),
        }
    }

    fn get_box(&mut self, lens_name: &str) -> &mut Vec<(&'a str, i32)> {
        let hash = hash_algorithm(lens_name.as_bytes());
        &mut self.lens_boxes[hash as usize]
    }

    fn remove(&mut self, lens_name: &str) {
        let lens_box = self.get_box(lens_name);
        if let Some(pos) = lens_box.iter().position(|(name, _)| *name == lens_name) {
            lens_box.remove(pos);
        }
    }

    fn insert(&mut self, lens_name: &'a str, focal_length: i32) {
        let lens_box = self.get_box(lens_name);
        if let Some(slot) = lens_box.iter_mut().find(|(name, _)| *name == lens_name) {
            slot.1 = focal_length;
        } else {
            lens_box.push((lens_name, focal_length));
        }
    }

    fn focusing_power(&self) -> i32 {
        let mut sum = 0;
        for (i, lens_box) in self.lens_boxes.iter().enumerate() {
            for (j, (_, focal_length)) in lens_box.iter().enumerate() {
                sum += (i + 1) as i32 * (j + 1) as i32 * *focal_length;
            }
        }

        sum
    }
}

fn hash_algorithm(chars: &[u8]) -> u8 {
    chars
        .iter()
        .fold(0, |acc, &c| acc.overflowing_add(c).0.overflowing_mul(17).0)
}

pub fn process(input: &str) -> i32 {
    let steps = parse_input(input);
    let mut hashmap = LensHashMap::new();

    for step in steps {
        match step {
            InitStep::Remove { lens_name } => hashmap.remove(lens_name),
            InitStep::Insert {
                lens_name,
                focal_length,
            } => hashmap.insert(lens_name, focal_length),
        }
    }

    hashmap.focusing_power()
}

#[derive(Debug, Clone, Copy)]
enum InitStep<'a> {
    Remove {
        lens_name: &'a str,
    },
    Insert {
        lens_name: &'a str,
        focal_length: i32,
    },
}

fn parse_input(input: &str) -> Vec<InitStep<'_>> {
    input
        .trim()
        .split(',')
        .map(|step| {
            if let Some(lens_name) = step.strip_suffix('-') {
                InitStep::Remove { lens_name }
            } else if let Some((lens_name, focal_length)) = step.split_once('=') {
                InitStep::Insert {
                    lens_name,
                    focal_length: focal_length.parse().unwrap(),
                }
            } else {
                panic!("Invalid step `{step}`");
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "";
        assert_eq!(400, process(input))
    }
}
