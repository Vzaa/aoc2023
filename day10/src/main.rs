use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);
type TileMap = HashMap<Pos, char>;

static NLIST: [Pos; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn neighbors_kv(tilemap: &TileMap, p: Pos) -> impl Iterator<Item = (Pos, char)> + '_ {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        tilemap.get(&np).map(|c| (*n, *c))
    })
}

fn reachable(n: Pos, c: char) -> bool {
    match (n, c) {
        ((-1, 0), '-') => true,
        ((-1, 0), 'F') => true,
        ((-1, 0), 'L') => true,
        ((1, 0), '-') => true,
        ((1, 0), '7') => true,
        ((1, 0), 'J') => true,
        ((0, -1), '|') => true,
        ((0, -1), 'F') => true,
        ((0, -1), '7') => true,
        ((0, 1), '|') => true,
        ((0, 1), 'J') => true,
        ((0, 1), 'L') => true,
        _ => false,
    }
}

fn reachables(tilemap: &TileMap, p: Pos) -> Vec<Pos> {
    neighbors_kv(tilemap, p)
        .filter(|(k, v)| reachable(*k, *v))
        .map(|(k, _)| k)
        .collect()
}

fn get_start_type(map: &TileMap, p: Pos) -> char {
    let mut n = reachables(map, p);
    n.sort();
    match n[..] {
        [(-1, 0), (1, 0)] => '-',
        [(0, -1), (0, 1)] => '|',
        [(0, 1), (1, 0)] => 'F',
        [(0, -1), (1, 0)] => 'L',
        [(-1, 0), (0, -1)] => 'J',
        [(-1, 0), (0, 1)] => '7',
        _ => unreachable!(),
    }
}

fn p1(instr: &str) -> i64 {
    let mut start = None;
    let mut tilemap: TileMap = HashMap::new();
    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((x as i64, y as i64));
            }
            if c != '.' {
                tilemap.insert((x as i64, y as i64), c);
            }
        }
    }
    let start = start.unwrap();
    let t = get_start_type(&tilemap, start);
    tilemap.insert(start, t);

    let dir = match t {
        '|' => Dir::N,
        '-' => Dir::E,
        'L' => Dir::N,
        'J' => Dir::N,
        '7' => Dir::S,
        'F' => Dir::S,
        _ => unreachable!(),
    };

    let mut pipe: HashSet<Pos> = HashSet::new();
    pipe.insert(start);
    let (mut cur, mut dir) = dir.mv(&tilemap, start);
    while cur != start {
        pipe.insert(cur);
        (cur, dir) = dir.mv(&tilemap, cur);
    }
    pipe.len() as i64 / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn mv(self, map: &TileMap, p: Pos) -> (Pos, Dir) {
        let next = match self {
            Dir::N => (p.0, p.1 - 1),
            Dir::S => (p.0, p.1 + 1),
            Dir::W => (p.0 - 1, p.1),
            Dir::E => (p.0 + 1, p.1),
        };

        let dir = match (self, map[&next]) {
            (Dir::N, '|') => Dir::N,
            (Dir::N, 'F') => Dir::E,
            (Dir::N, '7') => Dir::W,

            (Dir::S, '|') => Dir::S,
            (Dir::S, 'J') => Dir::W,
            (Dir::S, 'L') => Dir::E,

            (Dir::E, '-') => Dir::E,
            (Dir::E, 'J') => Dir::N,
            (Dir::E, '7') => Dir::S,

            (Dir::W, '-') => Dir::W,
            (Dir::W, 'F') => Dir::S,
            (Dir::W, 'L') => Dir::N,
            _ => unreachable!(),
        };
        (next, dir)
    }
}

fn get_right_spaces(c: char, dir: Dir) -> Vec<Pos> {
    match c {
        '|' => match dir {
            Dir::N => vec![(1, 0)],
            Dir::S => vec![(-1, 0)],
            _ => unreachable!(),
        },
        '-' => match dir {
            Dir::E => vec![(0, 1)],
            Dir::W => vec![(0, -1)],
            _ => unreachable!(),
        },
        'L' => match dir {
            Dir::N => vec![(1, -1)],
            Dir::E => vec![(-1, 0), (-1, 1), (0, 1)],
            _ => unreachable!(),
        },
        'J' => match dir {
            Dir::W => vec![(-1, -1)],
            Dir::N => vec![(0, 1), (1, 1), (1, 0)],
            _ => unreachable!(),
        },
        '7' => match dir {
            Dir::S => vec![(-1, 1)],
            Dir::W => vec![(0, -1), (1, -1), (1, 0)],
            _ => unreachable!(),
        },
        'F' => match dir {
            Dir::E => vec![(1, 1)],
            Dir::S => vec![(-1, 0), (-1, -1), (0, -1)],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn expand(pipe: &HashSet<Pos>, enclosed: &mut HashSet<Pos>, p: Pos) {
    if enclosed.contains(&p) {
        return;
    }
    enclosed.insert(p);

    for n in NLIST.iter() {
        let np = add(*n, p);
        if !pipe.contains(&np) {
            expand(pipe, enclosed, np);
        }
    }
}

fn p2(instr: &str) -> i64 {
    let mut start = None;
    let mut tilemap: TileMap = HashMap::new();
    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((x as i64, y as i64));
            }
            tilemap.insert((x as i64, y as i64), c);
        }
    }
    let start = start.unwrap();
    let t = get_start_type(&tilemap, start);
    tilemap.insert(start, t);

    // 50% chance
    let dir = match t {
        '|' => Dir::N,
        '-' => Dir::E,
        'L' => Dir::N,
        'J' => Dir::N,
        '7' => Dir::S,
        'F' => Dir::S,
        _ => unreachable!(),
    };

    let mut pipe: HashSet<Pos> = HashSet::new();
    pipe.insert(start);
    let (mut cur, mut dir) = dir.mv(&tilemap, start);
    while cur != start {
        pipe.insert(cur);
        (cur, dir) = dir.mv(&tilemap, cur);
    }

    let mut enclosed: HashSet<Pos> = HashSet::new();
    let (mut cur, mut dir) = dir.mv(&tilemap, start);
    while cur != start {
        let right = get_right_spaces(tilemap[&cur], dir);
        for r in right
            .iter()
            .map(|&n| add(n, cur))
            .filter(|n| !pipe.contains(n))
        {
            expand(&pipe, &mut enclosed, r);
        }
        (cur, dir) = dir.mv(&tilemap, cur);
    }
    enclosed.len() as i64
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
