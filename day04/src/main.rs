use std::str::FromStr;

#[derive(Debug)]
struct Card {
    nums: Vec<u32>,
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
        let winners = winner.nums.iter().filter(|w| card.nums.contains(w)).count();
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

    let mut done = vec![];
    let mut cards = vec![];
    for idx in 1..=pairs.len() {
        cards.push(idx);
    }

    while let Some(c) = cards.pop() {
        done.push(c);
        let (winner, card) = &pairs[c - 1];
        let winners = winner.nums.iter().filter(|w| card.nums.contains(w)).count();
        for n in (c + 1)..=(c + winners) {
            cards.push(n);
        }
    }
    done.len() as u32
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
