use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Pos = (i64, i64);
type TileMap = HashMap<Pos, u32>;

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn parse_map(s: &str) -> TileMap {
    let mut map = HashMap::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i64, y as i64), c.to_digit(10).unwrap());
        }
    }
    map
}

fn p1(instr: &str) -> u32 {
    let map = parse_map(instr);
    let corner = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    let left = (-1, 0);
    let right = (1, 0);
    let up = (0, -1);
    let down = (0, 1);

    let state_init = (0, (0, 0), (0, 0));

    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse(state_init));
    let mut best = HashMap::new();

    while let Some(Reverse((cost, cpos, ctraveled))) = frontier.pop() {
        if let Some(&b) = best.get(&(cpos, ctraveled)) {
            if b < cost {
                continue;
            }
        }
        best.insert((cpos, ctraveled), cost);

        if cpos == corner {
            return cost;
        }

        let mut push_better = |p, t, c| {
            if let Some(&b) = best.get(&(p, t)) {
                if c < b {
                    best.insert((p, t), c);
                    frontier.push(Reverse((c, p, t)))
                }
            } else {
                best.insert((p, t), c);
                frontier.push(Reverse((c, p, t)))
            }
        };

        let l_pos = add(cpos, left);
        let r_pos = add(cpos, right);
        let u_pos = add(cpos, up);
        let d_pos = add(cpos, down);

        if let Some(heat) = map.get(&l_pos) {
            if ctraveled.0 < 0 && ctraveled.0 > -3 {
                push_better(l_pos, add(ctraveled, left), heat + cost);
            } else if ctraveled.0 == 0 {
                push_better(l_pos, left, heat + cost);
            }
        }

        if let Some(heat) = map.get(&r_pos) {
            if ctraveled.0 > 0 && ctraveled.0 < 3 {
                push_better(r_pos, add(ctraveled, right), heat + cost);
            } else if ctraveled.0 == 0 {
                push_better(r_pos, right, heat + cost);
            }
        }

        if let Some(heat) = map.get(&u_pos) {
            if ctraveled.1 < 0 && ctraveled.1 > -3 {
                push_better(u_pos, add(ctraveled, up), heat + cost);
            } else if ctraveled.1 == 0 {
                push_better(u_pos, up, heat + cost);
            }
        }

        if let Some(heat) = map.get(&d_pos) {
            if ctraveled.1 > 0 && ctraveled.1 < 3 {
                push_better(d_pos, add(ctraveled, down), heat + cost);
            } else if ctraveled.1 == 0 {
                push_better(d_pos, down, heat + cost);
            }
        }
    }
    unreachable!()
}

fn p2(instr: &str) -> u32 {
    let map = parse_map(instr);
    let corner = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    let left = (-1, 0);
    let right = (1, 0);
    let up = (0, -1);
    let down = (0, 1);

    let state_init_a = (0, (0, 0), (10, 0));
    let state_init_b = (0, (0, 0), (0, 10));

    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse(state_init_a));
    frontier.push(Reverse(state_init_b));
    let mut best = HashMap::new();

    while let Some(Reverse((cost, cpos, ctraveled))) = frontier.pop() {
        if let Some(&b) = best.get(&(cpos, ctraveled)) {
            if b < cost {
                continue;
            }
        }
        best.insert((cpos, ctraveled), cost);

        if cpos == corner && (ctraveled.1 >= 4 || ctraveled.0 >= 4) {
            return cost;
        }

        let mut push_better = |p, t, c| {
            if let Some(&b) = best.get(&(p, t)) {
                if c < b {
                    best.insert((p, t), c);
                    frontier.push(Reverse((c, p, t)))
                }
            } else {
                best.insert((p, t), c);
                frontier.push(Reverse((c, p, t)))
            }
        };

        let l_pos = add(cpos, left);
        let r_pos = add(cpos, right);
        let u_pos = add(cpos, up);
        let d_pos = add(cpos, down);

        if let Some(heat) = map.get(&l_pos) {
            if ctraveled.0 < 0 && ctraveled.0 > -10 {
                push_better(l_pos, add(ctraveled, left), heat + cost);
            } else if ctraveled.1.abs() >= 4 {
                push_better(l_pos, left, heat + cost);
            }
        }

        if let Some(heat) = map.get(&r_pos) {
            if ctraveled.0 > 0 && ctraveled.0 < 10 {
                push_better(r_pos, add(ctraveled, right), heat + cost);
            } else if ctraveled.1.abs() >= 4 {
                push_better(r_pos, right, heat + cost);
            }
        }

        if let Some(heat) = map.get(&u_pos) {
            if ctraveled.1 < 0 && ctraveled.1 > -10 {
                push_better(u_pos, add(ctraveled, up), heat + cost);
            } else if ctraveled.0.abs() >= 4 {
                push_better(u_pos, up, heat + cost);
            }
        }

        if let Some(heat) = map.get(&d_pos) {
            if ctraveled.1 > 0 && ctraveled.1 < 10 {
                push_better(d_pos, add(ctraveled, down), heat + cost);
            } else if ctraveled.0.abs() >= 4 {
                push_better(d_pos, down, heat + cost);
            }
        }
    }
    unreachable!()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
