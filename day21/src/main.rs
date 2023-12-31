use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);
type TileMap = HashMap<Pos, char>;

type State = (i32, Pos);

static NLIST: [Pos; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn parse_map(s: &str) -> (TileMap, Pos) {
    let mut tilemap: TileMap = HashMap::new();
    let mut start = None;

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let y = y as i32;
            let x = x as i32;

            if c == 'S' {
                start = Some((x, y));
                tilemap.insert((x, y), '.');
            } else {
                tilemap.insert((x, y), c);
            }
        }
    }
    (tilemap, start.unwrap())
}

fn solve(
    map: &TileMap,
    limit: i32,
    use_pu: bool,
    odd: &mut HashSet<Pos>,
    even: &mut HashSet<Pos>,
    visited: &mut HashMap<Pos, i32>,
    initial: &mut Vec<State>,
) -> usize {
    let mut frontier = BinaryHeap::new();
    frontier.extend(initial.iter().clone());
    initial.clear();

    let (max_x, max_y) = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    while let Some((d, pos)) = frontier.pop() {
        if d % 2 == 0 {
            even.insert(pos);
        } else {
            odd.insert(pos);
        }
        visited.insert(pos, d);

        if d == limit {
            initial.push((d, pos));
            continue;
        }

        for n in NLIST.iter() {
            let np = add(*n, pos);
            let nd = d + 1;
            let point = if use_pu {
                (np.0.rem_euclid(max_x + 1), np.1.rem_euclid(max_y + 1))
            } else {
                np
            };

            if let Some('.') = map.get(&point) {
                if let Some(&old) = visited.get(&np) {
                    if nd < old {
                        frontier.push((nd, np));
                    }
                } else {
                    frontier.push((nd, np));
                }
            }
        }
    }

    if limit % 2 == 0 {
        even.len()
    } else {
        odd.len()
    }
}

fn p1(instr: &str) -> usize {
    let (map, start) = parse_map(instr);
    let mut odd = HashSet::new();
    let mut even = HashSet::new();
    let mut visited = HashMap::new();
    let mut initial = vec![(0, start)];
    solve(
        &map,
        64,
        false,
        &mut odd,
        &mut even,
        &mut visited,
        &mut initial,
    )
}

fn p2(instr: &str) -> usize {
    let mut past = vec![];
    let mut prev = 0;

    let (map, start) = parse_map(instr);

    let max_x = map.keys().map(|p| p.0).max().unwrap();

    let check = max_x as usize + 1;
    let tgt = 26501365;

    let mut odd = HashSet::new();
    let mut even = HashSet::new();
    let mut visited = HashMap::new();
    let mut initial = vec![(0, start)];

    for i in 1.. {
        let next = solve(
            &map,
            i as i32,
            true,
            &mut odd,
            &mut even,
            &mut visited,
            &mut initial,
        );
        past.push(next - prev);
        prev = next;

        // guess work here, could be optimized
        if i > 270 {
            let mut lut = vec![];
            let mut lut_d = vec![];
            for idx in (0..check).rev() {
                lut_d.push(past[i - 1 - idx] - past[i - 1 - check - idx]);
                lut.push(past[i - 1 - idx]);
            }
            let mut cur = next;
            for x in 0.. {
                let ndiff = lut[x % check] + lut_d[x % check];
                cur += ndiff;
                lut[x % check] = ndiff;
                if i + x + 1 == tgt {
                    return cur;
                }
            }
        }
    }

    unreachable!();
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
