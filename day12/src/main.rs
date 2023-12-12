use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Cond {
    Op,
    Broken,
    Unknown,
}

#[derive(Debug)]
struct Spring {
    springs: Vec<Cond>,
    rules: Vec<usize>,
}

fn char_cond(c: char) -> Cond {
    match c {
        '#' => Cond::Broken,
        '.' => Cond::Op,
        '?' => Cond::Unknown,
        _ => unreachable!(),
    }
}

impl FromStr for Spring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs_str, rules_str) = s.split_once(' ').unwrap();
        let springs = springs_str.chars().map(char_cond).collect();
        let rules = rules_str.split(',').map(|n| n.parse().unwrap()).collect();

        Ok(Spring { springs, rules })
    }
}

fn parse5(s: &str) -> Result<Spring, ()> {
    let (s, r) = s.split_once(' ').unwrap();
    let tmp = [s, s, s, s, s];
    let springs_str = tmp.join("?");
    let tmp = [r, r, r, r, r];
    let rules_str = tmp.join(",");
    let springs = springs_str.chars().map(char_cond).collect();
    let rules = rules_str.split(',').map(|n| n.parse().unwrap()).collect();
    Ok(Spring { springs, rules })
}

fn get_cmp(springs: &[Cond], expected: &[usize]) -> bool {
    let mut cnt = 0;
    let mut idx = 0;
    for s in springs {
        match s {
            Cond::Op => {
                if cnt != 0 {
                    if idx >= expected.len() {
                        return false;
                    }
                    if expected[idx] != cnt {
                        return false;
                    }
                    idx += 1;
                    cnt = 0;
                }
            }
            Cond::Broken => cnt += 1,
            Cond::Unknown => unreachable!(),
        }
    }

    if cnt != 0 {
        if idx + 1 != expected.len() {
            return false;
        }
        if expected[idx] != cnt {
            return false;
        }
    } else {
        return idx == expected.len();
    }
    true
}

fn can_put(slots: &[Cond], idx: usize, len: usize) -> bool {
    if slots.len() < idx + len {
        return false;
    }
    for slot in &slots[idx..(idx + len)] {
        match slot {
            Cond::Op => return false,
            _ => (),
        };
    }

    match slots.get(idx + len) {
        Some(Cond::Unknown) => return true,
        Some(Cond::Op) => return true,
        Some(Cond::Broken) => return false,
        None => return true,
    }
}

impl Spring {
    fn solve2(&self) -> u64 {
        let unknowns: Vec<_> = self
            .springs
            .iter()
            .enumerate()
            .filter(|(_, s)| matches!(s, Cond::Unknown))
            .map(|(i, _)| i)
            .collect();

        let mut cnt = 0;
        let mut copy = self.springs.clone();
        let tgt: usize = self.rules.iter().sum();
        let broken = self
            .springs
            .iter()
            .filter(|x| matches!(x, Cond::Broken))
            .count();
        for perm in 0..2_u128.pow(unknowns.len() as u32) {
            if (unknowns.len() - perm.count_ones() as usize) != (tgt - broken) {
                continue;
            }
            for (i, &pos) in unknowns.iter().enumerate() {
                if (1 << i) & perm != 0 {
                    copy[pos] = Cond::Op;
                } else {
                    copy[pos] = Cond::Broken;
                }
            }
            if get_cmp(&copy, &self.rules) {
                cnt += 1;
            }
        }

        cnt
    }
}

fn solve3(
    conds: &[Cond],
    rules: &[usize],
    depth: usize,
    memo: &mut HashMap<(Vec<Cond>, Vec<usize>), u64>,
) -> u64 {
    let mut perms = 0;
    let length = rules[0];

    if let Some(res) = memo.get(&(conds.to_owned(), rules.to_owned())) {
        return *res;
    }

    let mut state = true;
    for i in 0..conds.len() {
        if state && can_put(&conds, i, length) {
            if rules.len() == 1 {
                if i + length + 1 <= conds.len()
                    && conds[i + length + 1..]
                        .iter()
                        .any(|x| matches!(x, Cond::Broken))
                {
                } else {
                    perms += 1;
                }
                match conds[i] {
                    Cond::Op => state = true,
                    Cond::Broken => break,
                    Cond::Unknown => state = true,
                };
                continue;
            }
            if i + length == conds.len() {
                match conds[i] {
                    Cond::Op => state = true,
                    Cond::Broken => break,
                    Cond::Unknown => state = true,
                };
                continue;
            }
            let mut dis = 1;
            dis = dis * solve3(&conds[i + length + 1..], &rules[1..], depth + 1, memo);
            perms += dis;
        }
        match conds[i] {
            Cond::Op => state = true,
            Cond::Broken => break,
            Cond::Unknown => state = true,
        };
    }

    memo.insert((conds.to_owned(), rules.to_owned()), perms);
    perms
}

fn p1(instr: &str) -> u64 {
    let springs: Vec<Spring> = instr.lines().map(|l| l.parse().unwrap()).collect();

    let mut sum = 0;
    for s in springs.iter() {
        sum += s.solve2();
    }

    sum
}

fn p2(instr: &str) -> u64 {
    let springs5: Vec<Spring> = instr.lines().map(|l| parse5(l).unwrap()).collect();

    springs5
        .iter()
        .map(|d| {
            let mut memo = HashMap::new();
            return solve3(&d.springs, &d.rules, 0, &mut memo);
        })
        .sum()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
