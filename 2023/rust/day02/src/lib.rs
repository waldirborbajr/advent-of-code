pub fn process_part1(input: &str) -> u32 {
    let (red, green, blue) = (("red", 12), ("green", 13), ("blue", 14));
    let lines = input
        .lines()
        .map(|x| x.split(':').skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ids = vec![];

    for (index, line) in lines.iter().enumerate() {
        let sep = line.join("");
        let sep = sep.split(';').collect::<Vec<_>>();

        let mut seen = true;

        'sep: for it in sep {
            let ab = it.split(',').map(|x| x.trim()).collect::<Vec<_>>();

            for bc in ab {
                let split = bc.split(' ').collect::<Vec<_>>();
                let num = split
                    .first()
                    .expect("must be num")
                    .parse::<i32>()
                    .expect("is number");
                let color = split.last().expect("must be num");

                if color == &red.0 && num > red.1 {
                    seen = false;
                    break 'sep;
                }
                if color == &green.0 && num > green.1 {
                    seen = false;
                    break 'sep;
                }
                if color == &blue.0 && num > blue.1 {
                    seen = false;
                    break 'sep;
                }
            }
        }

        if seen {
            ids.push((index + 1) as u32);
        };
    }

    ids.iter().sum()
}

pub fn process_part2(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.split(':').skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!(" ");
    println!("input {:?}", lines);

    let mut arry_multiples = vec![];
    for (_index, line) in lines.iter().enumerate() {
        let sep = line.join("");
        let sep = sep.split(';').collect::<Vec<_>>();

        let (mut red, mut green, mut blue) = (("red", 0), ("green", 0), ("blue", 0));

        for (it_index, it) in sep.iter().enumerate() {
            let ab = it.split(',').map(|x| x.trim()).collect::<Vec<_>>();

            for bc in ab {
                let split = bc.split(' ').collect::<Vec<_>>();
                let num = split
                    .first()
                    .expect("must be num")
                    .parse::<u32>()
                    .expect("is number");
                let color = split.last().expect("must be num");

                if color == &red.0 {
                    red.1 = if red.1 > num { red.1 } else { num }
                }
                if color == &green.0 {
                    green.1 = if green.1 > num { green.1 } else { num }
                }
                if color == &blue.0 {
                    blue.1 = if blue.1 > num { blue.1 } else { num }
                }
            }
            let num = red.1 * green.1 * blue.1;
            if it_index == sep.len() - 1 {
                arry_multiples.push(num);
            }
        }
    }
    arry_multiples.iter().sum()
}

// pub fn process_part2(input: &str) -> u32 {
//     input
//         .trim()
//         .split("\n")
//         .map(|n| n.to_string())
//         .collect::<Vec<String>>()
//         .iter()
//         .map(|line| {
//             let game = get_sets_from_line(line);
//
//             let colors = game
//                 .groups
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<ColorCount>>();
//
//             let red = colors
//                 .iter()
//                 .filter(|col| col.color == Colors::Red)
//                 .max_by_key(|col| col.number)
//                 .unwrap()
//                 .number as usize;
//             let green = colors
//                 .iter()
//                 .filter(|col| col.color == Colors::Green)
//                 .max_by_key(|col| col.number)
//                 .unwrap()
//                 .number as usize;
//             let blue = colors
//                 .iter()
//                 .filter(|col| col.color == Colors::Blue)
//                 .max_by_key(|col| col.number)
//                 .unwrap()
//                 .number as usize;
//
//             (red * green * blue) as u32
//         })
//         .sum()
// }
//
// #[derive(Debug, PartialEq)]
// enum Colors {
//     Red,
//     Blue,
//     Green,
// }
//
// #[derive(Debug)]
// struct ColorCount {
//     color: Colors,
//     number: u8,
// }
//
// #[derive(Debug)]
// struct Game {
//     id: u8,
//     groups: Vec<Vec<ColorCount>>,
// }
//
// fn get_sets_from_line(line: &str) -> Game {
//     let split: Vec<&str> = line.split(":").collect();
//     let label_str = split.first().unwrap();
//     let game_id = label_str.replace("Game ", "").parse::<u8>().unwrap();
//
//     let groups_str = split.last().unwrap().trim();
//     let groups_vec = groups_str.split(";").collect::<Vec<&str>>();
//     let groups: Vec<Vec<ColorCount>> = groups_vec
//         .iter()
//         .map(|group| {
//             group
//                 .split(",")
//                 .collect::<Vec<&str>>()
//                 .iter()
//                 .map(|color| {
//                     let split: Vec<&str> = color.trim().split_whitespace().collect();
//                     let number = split.first().unwrap().parse::<u8>().unwrap();
//                     let col = split.last().unwrap().to_owned();
//
//                     ColorCount {
//                         color: match col {
//                             "red" => Colors::Red,
//                             "blue" => Colors::Blue,
//                             "green" => Colors::Green,
//                             _ => panic!(),
//                         },
//                         number,
//                     }
//                 })
//                 .collect()
//         })
//         .collect();
//
//     Game {
//         id: game_id,
//         groups,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT01: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT01);
        assert_eq!(result, 8);
    }

    #[test]
    // #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT01);
        assert_eq!(result, 2286);
    }
}
