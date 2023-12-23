use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

type Pos = (i16, i16);
type TileMap = HashMap<Pos, char>;

static NLIST: [Pos; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn parse_map(s: &str) -> TileMap {
    let mut map: TileMap = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let y = y as i16;
            let x = x as i16;

            map.insert((x, y), c);
        }
    }
    map
}

fn p1(instr: &str) -> usize {
    let map = parse_map(instr);

    let max_y = map.keys().map(|p| p.1).max().unwrap();

    let start = *map.iter().find(|(p, &c)| p.1 == 0 && c == '.').unwrap().0;
    let finish = *map
        .iter()
        .find(|(p, &c)| p.1 == max_y && c == '.')
        .unwrap()
        .0;

    let mut max = 0;
    let mut frontier = BinaryHeap::from([(0, start, start)]);
    while let Some((d, prev, pos)) = frontier.pop() {
        if pos == finish {
            max = max.max(d);
        }

        for n in NLIST.iter() {
            let np = add(*n, pos);
            let nd = d + 1;
            if np == prev {
                continue;
            }

            match map.get(&np) {
                Some('.') => {
                    frontier.push((nd, pos, np));
                }
                Some('v') if *n == (0, 1) => {
                    frontier.push((nd, pos, np));
                }
                Some('<') if *n == (-1, 0) => {
                    frontier.push((nd, pos, np));
                }
                Some('>') if *n == (1, 0) => {
                    frontier.push((nd, pos, np));
                }
                Some('^') if *n == (0, -1) => {
                    frontier.push((nd, pos, np));
                }
                _ => (),
            }
        }
    }
    max
}

fn reachable(map: &TileMap, start: Pos, tgt: Pos, past: &HashSet<Pos>) -> bool {
    let mut frontier = BinaryHeap::from([(0, start)]);
    let mut visited = HashSet::new();
    while let Some((d, pos)) = frontier.pop() {
        if pos == tgt {
            return true;
        }
        visited.insert(pos);
        for n in NLIST.iter() {
            let np = add(*n, pos);
            let nd = d + 1;
            if past.contains(&np) {
                continue;
            }

            match map.get(&np) {
                None => (),
                Some('#') => (),
                _ => {
                    if visited.insert(np) {
                        frontier.push((nd, np));
                    }
                }
            }
        }
    }

    false
}

fn follow(map: &TileMap, p: Pos, past: &HashSet<Pos>) -> (Pos, usize) {
    let mut cur = p;
    let mut prev;

    let mut tgt = cur;
    let mut d = 0;

    let mut cnt = 1;
    while cnt == 1 {
        prev = cur;
        cur = tgt;
        cnt = 0;
        d += 1;
        for n in NLIST.iter() {
            let np = add(*n, cur);
            if prev == np {
                continue;
            }
            if past.contains(&np) {
                continue;
            }

            match map.get(&np) {
                None => (),
                Some('#') => (),
                _ => {
                    tgt = np;
                    cnt += 1;
                }
            }
        }
    }

    (cur, d - 1)
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    d: usize,
    past: HashSet<Pos>,
    pos: Pos,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(other.d.cmp(&self.d))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.d.cmp(&self.d)
    }
}

fn p2(instr: &str) -> usize {
    let map = parse_map(instr);

    let max_y = map.keys().map(|p| p.1).max().unwrap();

    let start = *map.iter().find(|(p, &c)| p.1 == 0 && c == '.').unwrap().0;
    let finish = *map
        .iter()
        .find(|(p, &c)| p.1 == max_y && c == '.')
        .unwrap()
        .0;

    let mut max = 0;
    let mut frontier = BinaryHeap::from([(State {
        d: 0,
        past: HashSet::from([start]),
        pos: start,
    })]);

    while let Some(mut state) = frontier.pop() {
        if state.pos == finish {
            max = max.max(state.d);
        }

        let (next, d) = follow(&map, state.pos, &state.past);
        state.past.insert(next);
        state.d += d;
        state.pos = next;

        if state.pos == finish {
            max = max.max(state.d);
        }

        for n in NLIST.iter() {
            let np = add(*n, state.pos);
            let nd = state.d + 1;
            if state.past.contains(&np) {
                continue;
            }

            match map.get(&np) {
                None => (),
                Some('#') => (),
                _ => {
                    if reachable(&map, np, finish, &state.past) {
                        let mut pn = state.past.clone();
                        pn.insert(np);
                        frontier.push(State {
                            d: nd,
                            past: pn,
                            pos: np,
                        });
                    }
                }
            }
        }
    }
    max
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
