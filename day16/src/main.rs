use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);
type TileMap = HashMap<Pos, char>;

#[derive(Debug, Clone)]
struct Cave {
    map: TileMap,
    corner_a: Pos,
    corner_b: Pos,
}

impl Cave {
    fn from_str(s: &str) -> Cave {
        let mut map: TileMap = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.insert((x as i64, y as i64), c);
            }
        }

        let corner_a = (
            map.keys().map(|p| p.0).min().unwrap(),
            map.keys().map(|p| p.1).min().unwrap(),
        );

        let corner_b = (
            map.keys().map(|p| p.0).max().unwrap(),
            map.keys().map(|p| p.1).max().unwrap(),
        );

        Cave {
            map,
            corner_a,
            corner_b,
        }
    }

    fn in_range(&self, p: Pos) -> bool {
        p.0 >= self.corner_a.0
            && p.0 <= self.corner_b.0
            && p.1 >= self.corner_a.1
            && p.1 <= self.corner_b.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn mv(self, map: &TileMap, p: Pos) -> (Pos, Dir, Option<Dir>) {
        let next = match self {
            Dir::N => (p.0, p.1 - 1),
            Dir::S => (p.0, p.1 + 1),
            Dir::W => (p.0 - 1, p.1),
            Dir::E => (p.0 + 1, p.1),
        };

        let mut split = None;

        let dir = match (self, map.get(&next).unwrap_or(&'.')) {
            (Dir::N, '/') => Dir::E,
            (Dir::N, '\\') => Dir::W,
            (Dir::N, '|') => Dir::N,
            (Dir::N, '-') => {
                split = Some(Dir::W);
                Dir::E
            }

            (Dir::S, '/') => Dir::W,
            (Dir::S, '\\') => Dir::E,
            (Dir::S, '|') => Dir::S,
            (Dir::S, '-') => {
                split = Some(Dir::W);
                Dir::E
            }

            (Dir::E, '/') => Dir::N,
            (Dir::E, '\\') => Dir::S,
            (Dir::E, '-') => Dir::E,
            (Dir::E, '|') => {
                split = Some(Dir::N);
                Dir::S
            }

            (Dir::W, '/') => Dir::S,
            (Dir::W, '\\') => Dir::N,
            (Dir::W, '-') => Dir::W,
            (Dir::W, '|') => {
                split = Some(Dir::N);
                Dir::S
            }

            _ => self,
        };

        (next, dir, split)
    }
}

#[derive(Debug, Clone)]
struct Ray {
    p: Pos,
    d: Dir,
    past: HashSet<(Pos, Dir)>,
}

impl Ray {
    fn new(p: Pos, d: Dir) -> Ray {
        let past = HashSet::new();
        Ray { p, d, past }
    }

    fn mv(&mut self, cave: &Cave) -> (bool, Option<Ray>) {
        let split;
        (self.p, self.d, split) = self.d.mv(&cave.map, self.p);

        if !cave.in_range(self.p) {
            return (true, None);
        }

        if !self.past.insert((self.p, self.d)) {
            return (true, None);
        }

        let newray = if let Some(d) = split {
            let mut tmp = self.clone();
            tmp.d = d;
            Some(tmp)
        } else {
            None
        };

        (false, newray)
    }
}

fn solve(cave: &Cave, p: Pos, d: Dir) -> usize {
    let mut rays = vec![Ray::new(p, d)];

    let mut splits = HashSet::new();

    loop {
        let mut finish = true;
        let mut next = vec![];
        for r in &mut rays {
            let (done, newray) = r.mv(cave);

            if let Some(ray) = newray {
                if splits.insert((ray.p, ray.d)) {
                    next.push(ray);
                }
            }

            if !done {
                finish = false;
            }
        }
        rays.extend(next);
        if finish {
            break;
        }
    }

    let energized: HashSet<_> = rays
        .iter()
        .flat_map(|r| r.past.iter().map(|p| p.0))
        .collect();

    energized.len()
}

fn p1(instr: &str) -> usize {
    let cave = Cave::from_str(instr);
    solve(&cave, (-1, 0), Dir::E)
}

fn p2(instr: &str) -> usize {
    let cave = Cave::from_str(instr);
    let a = (0..=cave.corner_b.0)
        .map(|n| solve(&cave, (n, -1), Dir::S))
        .max()
        .unwrap();
    let b = (0..=cave.corner_b.0)
        .map(|n| solve(&cave, (n, cave.corner_b.1 + 1), Dir::N))
        .max()
        .unwrap();
    let c = (0..=cave.corner_b.1)
        .map(|n| solve(&cave, (-1, n), Dir::E))
        .max()
        .unwrap();
    let d = (0..=cave.corner_b.1)
        .map(|n| solve(&cave, (cave.corner_b.0 + 1, n), Dir::W))
        .max()
        .unwrap();
    [a, b, c, d].into_iter().max().unwrap()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
