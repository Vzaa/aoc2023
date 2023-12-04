use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Card {
    nums: HashSet<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

        Ok(Card { nums })
    }
}

fn p1(instr: &str) -> u32 {
    let pairs: Vec<_> = instr
        .lines()
        .map(|l| {
            let cards_str = l.split_once(':').unwrap().1;
            let (winning_str, card_str) = cards_str.split_once('|').unwrap();
            let winning: Card = winning_str.trim().parse().unwrap();
            let card: Card = card_str.trim().parse().unwrap();
            (winning, card)
        })
        .collect();

    let mut sum = 0;
    for (winner, card) in &pairs {
        let winners = winner.nums.intersection(&card.nums).count();
        if winners > 0 {
            sum += 2_u32.pow(winners as u32 - 1);
        }
    }
    sum
}

fn p2(instr: &str) -> u32 {
    let pairs: Vec<_> = instr
        .lines()
        .map(|l| {
            let cards_str = l.split_once(':').unwrap().1;
            let (winning_str, card_str) = cards_str.split_once('|').unwrap();
            let winning: Card = winning_str.trim().parse().unwrap();
            let card: Card = card_str.trim().parse().unwrap();
            (winning, card)
        })
        .collect();

    let mut done = 0;
    let mut cards = vec![1; pairs.len()];

    for i in 0..cards.len() {
        done += cards[i];
        let (winner, card) = &pairs[i];
        let winners = winner.nums.intersection(&card.nums).count();
        for n in (i + 1)..=(i + winners) {
            cards[n] += cards[i];
        }
    }

    done
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
