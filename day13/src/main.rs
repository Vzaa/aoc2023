use std::collections::HashMap;

type Pos = (usize, usize);
type TileMap = HashMap<Pos, char>;

fn parse_map(s: &str) -> TileMap {
    let mut tilemap: TileMap = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x, y), c);
        }
    }
    tilemap
}

fn vertical(map: &TileMap, filter: Option<usize>) -> usize {
    let (w, h) = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    'outer: for m in 0..w {
        let mirror = (m + 1).min(w - m);
        for offset in 0..mirror {
            for y in 0..=h {
                if map[&(m - offset, y)] != map[&(m + offset + 1, y)] {
                    continue 'outer;
                }
            }
        }
        match filter {
            Some(f) if f == m + 1 => continue,
            _ => return m + 1,
        }
    }

    0
}

fn horizontal(map: &TileMap, filter: Option<usize>) -> usize {
    let (w, h) = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    'outer: for m in 0..h {
        let mirror = (m + 1).min(h - m);
        for offset in 0..mirror {
            for x in 0..=w {
                if map[&(x, m - offset)] != map[&(x, m + offset + 1)] {
                    continue 'outer;
                }
            }
        }
        match filter {
            Some(f) if f == m + 1 => continue,
            _ => return m + 1,
        }
    }
    0
}

fn p1(instr: &str) -> usize {
    let maps: Vec<_> = instr.split("\n\n").map(|m| parse_map(m)).collect();

    let mut sum = 0;
    for map in maps.iter() {
        sum += vertical(map, None);
        sum += 100 * horizontal(map, None);
    }

    sum
}

fn p2(instr: &str) -> usize {
    let maps: Vec<_> = instr.split("\n\n").map(|m| parse_map(m)).collect();

    let mut sum = 0;
    'outer: for map in maps.iter() {
        let v = vertical(map, None);
        let h = horizontal(map, None);
        for pos in map.keys() {
            let mut clone = map.clone();
            let t = clone.get_mut(pos).unwrap();
            if *t == '.' {
                *t = '#';
            } else {
                *t = '.';
            }
            let vv = vertical(&clone, Some(v));
            let hh = horizontal(&clone, Some(h));

            if vv != 0 || hh != 0 {
                sum += vv;
                sum += 100 * hh;
                continue 'outer;
            }
        }
    }
    sum
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
