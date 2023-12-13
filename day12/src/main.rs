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
    let tmp = [s; 5];
    let springs_str = tmp.join("?");
    let tmp = [r; 5];
    let rules_str = tmp.join(",");
    (springs_str + " " + &rules_str).parse()
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

fn solve<'a>(
    conds: &'a [Cond],
    rules: &'a [usize],
    depth: usize,
    memo: &mut HashMap<(&'a [Cond], &'a [usize]), u64>,
) -> u64 {
    let mut perms = 0;
    let length = rules[0];

    if let Some(res) = memo.get(&(conds, rules)) {
        return *res;
    }

    for i in 0..conds.len() {
        if can_put(&conds, i, length) {
            if rules.len() == 1 {
                if !(i + length + 1 <= conds.len()
                    && conds[i + length + 1..]
                        .iter()
                        .any(|x| matches!(x, Cond::Broken)))
                {
                    perms += 1;
                }
                if matches!(conds[i], Cond::Broken) {
                    break;
                }
                continue;
            }
            if i + length == conds.len() {
                continue;
            }
            perms += solve(&conds[i + length + 1..], &rules[1..], depth + 1, memo);
        }
        if matches!(conds[i], Cond::Broken) {
            break;
        }
    }

    // let mut memo2 = HashMap::new();
    // memo2.insert((conds, rules), perms);

    memo.insert((conds, rules), perms);
    perms
}

fn p1(instr: &str) -> u64 {
    let springs: Vec<Spring> = instr.lines().map(|l| l.parse().unwrap()).collect();

    springs
        .iter()
        .map(|s| {
            let mut memo = HashMap::new();
            return solve(&s.springs, &s.rules, 0, &mut memo);
        })
        .sum()
}

fn p2(instr: &str) -> u64 {
    let springs5: Vec<Spring> = instr.lines().map(|l| parse5(l).unwrap()).collect();

    springs5
        .iter()
        .map(|s| {
            let mut memo = HashMap::new();
            return solve(&s.springs, &s.rules, 0, &mut memo);
        })
        .sum()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
