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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

use std::cmp::Ordering;

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards.len() != other.cards.len() {
            return self.cards.len().cmp(&other.cards.len());
        }

        let mut counter1 = self.cards.iter().fold([0; 13], |mut counter, card| {
            counter[*card as usize] += 1;
            counter
        });
        counter1.sort();
        counter1.reverse();
        let mut counter2 = other.cards.iter().fold([0; 13], |mut counter, card| {
            counter[*card as usize] += 1;
            counter
        });
        counter2.sort();
        counter2.reverse();
        for (a, b) in counter1.iter().zip(counter2.iter()) {
            if a != b {
                return a.cmp(b);
            }
        }

        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return a.cmp(b);
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn from_str(hand_str: &str) -> Result<Hand, &'static str> {
        let mut cards = Vec::new();
        for ch in hand_str.chars() {
            let card = match ch {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => return Err("Invalid character in hand string"),
            };
            cards.push(card);
        }
        Ok(Hand { cards })
    }
}

pub fn process(input: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut hands: Vec<(Hand, i64)> = Vec::new();

    for line in input.lines() {
        let (hand, bet) = line.split_once(" ").unwrap();
        let hand = Hand::from_str(hand).unwrap();
        let bet = bet.parse::<i64>().unwrap();
        hands.push((hand, bet));
    }

    hands.sort();

    for i in 0..hands.len() {
        let (_, bet) = &hands[i];
        ans += ((i + 1) as i64) * bet;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input))
    }
}
