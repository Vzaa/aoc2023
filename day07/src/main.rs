use std::{collections::HashMap, cmp::{Ordering, self}};

fn score(s: &str) -> u64 {
    let mut card_lut = HashMap::new();
    for c in s.chars() {
        let cnt = card_lut.entry(c).or_insert(0_usize);
        *cnt += 1;
    }

    if card_lut.len() == 5 {
        0
    } else if card_lut.len() == 4 {
        1
    } else if card_lut.len() == 3 {
        let mut cnts: Vec<_> = card_lut.values().collect();
        cnts.sort();
        cnts.reverse();
        if *cnts[0] == 2 {
            2
        } else if *cnts[0] == 3 {
            3
        } else {
            unreachable!()
        }
    } else if card_lut.len() == 2 {
        let mut cnts: Vec<_> = card_lut.values().collect();
        cnts.sort();
        cnts.reverse();
        if *cnts[0] == 3 {
            4
        } else if *cnts[0] == 4 {
            5
        } else {
            unreachable!()
        }
    } else if card_lut.len() == 1 {
        6
    } else {
        unreachable!()
    }
}

fn compare_cards(a: &str, b: &str) -> cmp::Ordering {
    let score_a = score(a);
    let score_b = score(b);

    let cards = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

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

fn score2(s: &str) -> u64 {
    let mut card_lut = HashMap::new();
    for c in s.chars() {
        let cnt = card_lut.entry(c).or_insert(0_usize);
        *cnt += 1;
    }

    if let Some(&j) = card_lut.get(&'J') {
        if j != 5 {
            card_lut.remove(&'J');
            let m = card_lut.values_mut().max().unwrap();
            *m += j
        }
    }

    if card_lut.len() == 5 {
        0
    } else if card_lut.len() == 4 {
        1
    } else if card_lut.len() == 3 {
        let mut cnts: Vec<_> = card_lut.values().collect();
        cnts.sort();
        cnts.reverse();
        if *cnts[0] == 2 {
            2
        } else if *cnts[0] == 3 {
            3
        } else {
            unreachable!()
        }
    } else if card_lut.len() == 2 {
        let mut cnts: Vec<_> = card_lut.values().collect();
        cnts.sort();
        cnts.reverse();
        if *cnts[0] == 3 {
            4
        } else if *cnts[0] == 4 {
            5
        } else {
            unreachable!()
        }
    } else if card_lut.len() == 1 {
        6
    } else {
        unreachable!()
    }
}

fn compare_cards2(a: &str, b: &str) -> cmp::Ordering {
    let score_a = score2(a);
    let score_b = score2(b);

    let cards = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
    ];

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

    list.sort_by(|a, b| compare_cards(a.0, b.0));
    list.iter().enumerate().map(|(idx, (_, bid))| (idx as u64 + 1) * bid).sum()
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

    list.sort_by(|a, b| compare_cards2(a.0, b.0));
    list.iter().enumerate().map(|(idx, (_, bid))| (idx as u64 + 1) * bid).sum()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
