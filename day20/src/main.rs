use std::collections::{HashMap, VecDeque};

type Modules<'a> = HashMap<&'a str, Module<'a>>;

#[derive(Debug, Clone, Copy)]
enum Pulse {
    L,
    H,
}

#[derive(Debug)]
enum ModType<'a> {
    F(bool),
    C(HashMap<&'a str, Pulse>),
    B,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    m_type: ModType<'a>,
    dsts: Vec<&'a str>,
}

impl Module<'_> {
    fn from_str(s: &str) -> Module {
        let (name_str, dsts_str) = s.split_once(" -> ").unwrap();

        let (m, name) = if name_str.starts_with('%') {
            (ModType::F(false), &name_str[1..])
        } else if name_str.starts_with('&') {
            (ModType::C(HashMap::new()), &name_str[1..])
        } else {
            (ModType::B, name_str)
        };

        let dsts = dsts_str.split(',').map(|s| s.trim()).collect();

        Module {
            name,
            m_type: m,
            dsts,
        }
    }
}

fn add_conjunction_inputs(modules: &mut Modules) {
    let mut tmp = vec![];
    for m in modules.values() {
        for &d in &m.dsts {
            if let Some(ModType::C(_)) = modules.get(&d).map(|m| &m.m_type) {
                tmp.push((d, m.name));
            }
        }
    }

    for (c, input) in tmp {
        let m = modules.get_mut(c).unwrap();
        if let ModType::C(ref mut inputs) = m.m_type {
            inputs.insert(input, Pulse::L);
        }
    }
}

fn p1(instr: &str) -> usize {
    let mut modules: HashMap<_, _> = instr
        .lines()
        .map(Module::from_str)
        .map(|m| (m.name, m))
        .collect();
    add_conjunction_inputs(&mut modules);

    let mut h = 0;
    let mut l = 0;
    for _ in 0..1000 {
        let signals = press(&mut modules);
        let ll = signals
            .iter()
            .filter(|(_, _, p)| matches!(p, Pulse::L))
            .count();
        let hh = signals
            .iter()
            .filter(|(_, _, p)| matches!(p, Pulse::H))
            .count();
        h += hh;
        l += ll;
    }

    h * l
}

fn press<'a>(modules: &mut Modules<'a>) -> Vec<(&'a str, &'a str, Pulse)> {
    let mut queue = VecDeque::from([("button", "broadcaster", Pulse::L)]);

    let mut signals = vec![];

    while let Some((src, dst, p)) = queue.pop_back() {
        signals.push((src, dst, p));

        let m = modules.get_mut(&dst);
        if m.is_none() {
            continue;
        }
        let m = m.unwrap();
        match m.m_type {
            ModType::F(ref mut f) => {
                if matches!(p, Pulse::L) {
                    for d in &m.dsts {
                        if *f {
                            queue.push_front((m.name, d, Pulse::L));
                        } else {
                            queue.push_front((m.name, d, Pulse::H));
                        }
                    }
                    *f = !*f;
                }
            }
            ModType::C(ref mut mem) => {
                mem.insert(src, p);
                if mem.values().all(|past| matches!(past, Pulse::H)) {
                    for d in &m.dsts {
                        queue.push_front((m.name, d, Pulse::L));
                    }
                } else {
                    for d in &m.dsts {
                        queue.push_front((m.name, d, Pulse::H));
                    }
                }
            }
            ModType::B => {
                for d in &m.dsts {
                    queue.push_front((m.name, d, p));
                }
            }
        }
    }
    signals
}

fn p2(instr: &str) -> usize {
    let mut modules: HashMap<_, _> = instr
        .lines()
        .map(Module::from_str)
        .map(|m| (m.name, m))
        .collect();
    add_conjunction_inputs(&mut modules);

    let mut found = HashMap::new();

    let prev = modules.values().find(|m| m.dsts.contains(&"rx")).unwrap();
    let conjunctions: Vec<_> = modules
        .values()
        .filter(|m| m.dsts.contains(&prev.name))
        .map(|m| m.name)
        .collect();

    for i in 1.. {
        let signals = press(&mut modules);

        let lows = signals
            .iter()
            .filter(|(_, dst, p)| conjunctions.contains(dst) && matches!(p, Pulse::L));

        for low in lows {
            if !found.contains_key(low.0) {
                found.insert(low.0, i);
            }
        }

        if found.len() == conjunctions.len() {
            break;
        }
    }

    found.values().fold(1, |acc, c| lcm(*c, acc))
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
