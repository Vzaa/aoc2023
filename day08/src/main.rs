use std::collections::HashMap;

fn p1(instr: &str) -> u64 {
    let (instructions, map_str) = instr.split_once("\n\n").unwrap();

    let map: HashMap<_, _> = map_str
        .lines()
        .map(|l| {
            let (pos_str, dsts_str) = l.split_once("=").unwrap();
            let pos_str = pos_str.trim();

            let (l, r) = dsts_str
                .trim_matches(|c| "() ".contains(c))
                .split_once(',')
                .unwrap();

            (pos_str, (l.trim(), r.trim()))
        })
        .collect();

    let mut pos = "AAA";
    for (steps, c) in instructions.chars().cycle().enumerate() {
        let (l, r) = map[pos];
        match c {
            'L' => pos = l,
            'R' => pos = r,
            _ => unreachable!(),
        }
        if pos == "ZZZ" {
            return steps as u64 + 1;
        }
    }
    unreachable!()
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn p2(instr: &str) -> u64 {
    let (instructions, map_str) = instr.split_once("\n\n").unwrap();

    let map: HashMap<_, _> = map_str
        .lines()
        .map(|l| {
            let (pos_str, dsts_str) = l.split_once("=").unwrap();
            let pos_str = pos_str.trim();

            let (l, r) = dsts_str
                .trim_matches(|c| "() ".contains(c))
                .split_once(',')
                .unwrap();

            (pos_str, (l.trim(), r.trim()))
        })
        .collect();

    let mut poses: Vec<_> = map
        .keys()
        .filter(|p| p.ends_with('A'))
        .map(|p| *p)
        .collect();
    let mut cycles = vec![];

    for p in &mut poses {
        for (steps, c) in instructions.chars().cycle().enumerate() {
            let (l, r) = map[p];
            match c {
                'L' => *p = l,
                'R' => *p = r,
                _ => unreachable!(),
            }

            if p.ends_with('Z') {
                cycles.push(steps as u64 + 1);
                break;
            }
        }
    }

    cycles.iter().fold(1, |acc, c| lcm(*c, acc))
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
