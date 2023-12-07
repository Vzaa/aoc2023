use std::cmp::Ordering;
use std::collections::HashMap;

fn score(s: &str, joker: bool) -> u64 {
    let mut card_lut = HashMap::new();
    for c in s.chars() {
        let cnt = card_lut.entry(c).or_insert(0_usize);
        *cnt += 1;
    }

    if joker {
        if let Some(&j) = card_lut.get(&'J') {
            if j != 5 {
                card_lut.remove(&'J');
                let m = card_lut.values_mut().max().unwrap();
                *m += j
            }
        }
    }

    match (card_lut.len(), card_lut.values().max().unwrap()) {
        (5, _) => 0,
        (4, _) => 1,
        (3, 2) => 2,
        (3, 3) => 3,
        (2, 3) => 4,
        (2, 4) => 5,
        (1, _) => 6,
        _ => unreachable!(),
    }
}

fn compare_cards(a: &str, b: &str, joker: bool) -> Ordering {
    let score_a = score(a, joker);
    let score_b = score(b, joker);

    let cards = if joker {
        [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ]
    } else {
        [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ]
    };

    if score_a != score_b {
        score_a.cmp(&score_b)
    } else {
        let compare = a.chars().zip(b.chars()).find(|(ca, cb)| ca != cb);
        if let Some((ca, cb)) = compare {
            let idx_a = cards.iter().position(|&c| ca == c).unwrap();
            let idx_b = cards.iter().position(|&c| cb == c).unwrap();
            idx_b.cmp(&idx_a)
        } else {
            Ordering::Equal
        }
    }
}

fn p1(instr: &str) -> u64 {
    let mut list: Vec<_> = instr
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(cards, bidstr)| (cards, bidstr.parse::<u64>().unwrap()))
                .unwrap()
        })
        .collect();

    list.sort_by(|a, b| compare_cards(a.0, b.0, false));
    list.iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u64 + 1) * bid)
        .sum()
}

fn p2(instr: &str) -> u64 {
    let mut list: Vec<_> = instr
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(cards, bidstr)| (cards, bidstr.parse::<u64>().unwrap()))
                .unwrap()
        })
        .collect();

    list.sort_by(|a, b| compare_cards(a.0, b.0, true));
    list.iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u64 + 1) * bid)
        .sum()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
