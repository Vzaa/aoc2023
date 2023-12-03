use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);
type TileMap = HashMap<Pos, Tile>;

#[derive(Debug)]
enum Tile {
    Num(i32),
    Symbol(char),
}

static NLIST: [Pos; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbors(tilemap: &TileMap, p: Pos) -> impl Iterator<Item = &Tile> + '_ {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        tilemap.get(&np)
    })
}

fn neighborsp(area: &TileMap, p: Pos) -> impl Iterator<Item = (Pos, &Tile)> + '_ {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        area.get(&np).map(|n| (np, n))
    })
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => None,
            x if x.is_ascii_digit() => Some(Tile::Num(x.to_digit(10).unwrap() as i32)),
            x => Some(Tile::Symbol(x)),
        }
    }
}

fn p1(instr: &str) -> i32 {
    let mut tilemap: TileMap = HashMap::new();

    let w = instr.lines().next().unwrap().chars().count() as i32;
    let h = instr.lines().count() as i32;

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(t) = Tile::from_char(c) {
                tilemap.insert((x as i32, y as i32), t);
            }
        }
    }

    let mut sum = 0;
    for y in 0..h {
        let mut sym_found = false;
        let mut num = 0;
        for x in 0..(w + 1) {
            if let Some(Tile::Num(n)) = tilemap.get(&(x, y)) {
                num = num * 10 + n;
                if !sym_found {
                    sym_found = neighbors(&tilemap, (x, y)).any(|t| matches!(t, Tile::Symbol(_)));
                }
            } else {
                if sym_found {
                    sum += num;
                }
                num = 0;
                sym_found = false;
            }
        }
    }

    sum
}

fn getnum(tilemap: &TileMap, p: Pos) -> (Pos, i32) {
    let mut num = 0;
    let mut start = p;
    while let Some(Tile::Num(_)) = tilemap.get(&start) {
        start.0 -= 1;
    }
    start.0 += 1;

    let mut c = start;
    while let Some(Tile::Num(n)) = tilemap.get(&c) {
        num = num * 10 + n;
        c.0 += 1;
    }
    (start, num)
}

fn p2(instr: &str) -> i32 {
    let mut tilemap: TileMap = HashMap::new();

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(t) = Tile::from_char(c) {
                tilemap.insert((x as i32, y as i32), t);
            }
        }
    }

    let mut sum = 0;
    for (&(x, y), _) in tilemap
        .iter()
        .filter(|(_, t)| matches!(t, Tile::Symbol('*')))
    {
        let nums: HashSet<_> = neighborsp(&tilemap, (x, y))
            .filter(|(_, t)| matches!(t, Tile::Num(_)))
            .map(|(p, _)| getnum(&tilemap, p))
            .collect();

        if nums.len() == 2 {
            sum += nums.iter().fold(1, |m, n| m * n.1);
        }
    }
    sum
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
