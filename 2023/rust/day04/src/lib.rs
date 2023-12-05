use std::collections::{HashMap, HashSet};

pub fn process_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.split(':').collect::<Vec<_>>())
        .map(|x| *x.last().expect(""))
        .map(|x| x.split('|').collect::<Vec<_>>())
        .map(|x| {
            let first = x
                .first()
                .expect("some")
                .split(' ')
                .filter(|x| x != &"")
                .collect::<Vec<_>>();
            // println!("first {first:?}");
            let last = x
                .last()
                .expect("some")
                .split(' ')
                .filter(|x| x != &"")
                .collect::<Vec<_>>();
            let ve = first
                .iter()
                .filter(|x| last.contains(x))
                .cloned()
                .collect::<Vec<_>>();
            ve
        })
        .filter(|x| !x.is_empty())
        .map(|x| {
            let cal = |n: i32| 2i32.pow((n - 1) as u32);
            cal(x.len() as i32)
        })
        .sum::<i32>()
    // input
    //     .lines()
    //     .map(|card| {
    //         match card
    //             .split(": ")
    //             .last()
    //             .expect("the card contains a ': '")
    //             .split("| ")
    //             .map(|group| {
    //                 group
    //                     .split_whitespace()
    //                     .map(|number| number.parse::<u32>().expect("each element is a u32"))
    //                     .collect::<HashSet<u32>>()
    //             })
    //             .reduce(|acc, group| {
    //                 acc.intersection(&group)
    //                     .map(|number| *number)
    //                     .collect::<HashSet<u32>>()
    //             })
    //             .expect("there are two groups")
    //             .len()
    //         {
    //             0 => 0,
    //             wins => 2_u32.pow((wins as u32) - 1),
    //         }
    //     })
    //     .sum::<u32>()
}

pub fn process_part2(input: &str) -> u32 {
    let mut ticket_count: HashMap<usize, usize> = HashMap::new();

    for (card, l) in input.lines().enumerate() {
        *ticket_count.entry(card + 1).or_insert(0) += 1;

        let ticket = l
            .split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .map(|s| s.split_whitespace().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let cnt = ticket
            .get(0)
            .unwrap()
            .intersection(ticket.get(1).unwrap())
            .count();

        for n in 0..cnt {
            let num = *ticket_count.get(&(card + 1)).unwrap();
            *ticket_count.entry(card + 2 + n).or_insert(0) += num;
        }
    }
    ticket_count.values().sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT01: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT01);
        assert_eq!(result, 13);
    }

    #[test]
    // #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT01);
        assert_eq!(result, 30);
    }
}
