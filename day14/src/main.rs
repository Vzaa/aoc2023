use std::collections::HashMap;

type Pos = (i64, i64);
type TileMap = HashMap<Pos, Rock>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Rock {
    Cube,
    Round,
}

fn char_rock(c: char) -> Rock {
    match c {
        '#' => Rock::Cube,
        'O' => Rock::Round,
        _ => unreachable!(),
    }
}

fn parse_map(s: &str) -> TileMap {
    let mut tilemap: TileMap = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                tilemap.insert((x as i64, y as i64), char_rock(c));
            }
        }
    }
    tilemap
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn tilt(mut map: TileMap, dir: Pos) -> TileMap {
    // rip repeated calculations
    let (max_x, max_y) = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    let (min_x, min_y) = (
        map.keys().map(|p| p.0).min().unwrap(),
        map.keys().map(|p| p.1).min().unwrap(),
    );

    let in_range = |(x, y)| x >= min_x && x <= max_x && y >= min_y && y <= max_y;

    loop {
        let mut clone = map.clone();
        for (pos, _) in map.iter().filter(|(_, r)| matches!(r, Rock::Round)) {
            let mut cur = *pos;
            loop {
                let next = add(cur, dir);
                if map.contains_key(&next) || !in_range(next) {
                    let tmp = clone.remove(pos).unwrap();
                    clone.insert(cur, tmp);
                    break;
                }
                cur = next;
            }
        }
        if clone == map {
            break;
        }
        map = clone;
    }
    map
}

fn tilt_north(map: TileMap) -> TileMap {
    tilt(map, (0, -1))
}

fn tilt_west(map: TileMap) -> TileMap {
    tilt(map, (-1, 0))
}

fn tilt_south(map: TileMap) -> TileMap {
    tilt(map, (0, 1))
}

fn tilt_east(map: TileMap) -> TileMap {
    tilt(map, (1, 0))
}

fn calc_load(map: &TileMap) -> i64 {
    let max_y = map.keys().map(|p| p.1).max().unwrap();

    map.iter()
        .filter(|(_, r)| matches!(r, Rock::Round))
        .map(|(pos, _)| max_y - pos.1 + 1)
        .sum()
}

fn p1(instr: &str) -> i64 {
    let mut map = parse_map(instr);
    map = tilt_north(map);
    calc_load(&map)
}

fn p2(instr: &str) -> i64 {
    let mut map = parse_map(instr);
    let mut past = HashMap::new();
    let mut offset = None;
    let limit = 1000000000;

    for i in 1..=limit {
        map = tilt_north(map);
        map = tilt_west(map);
        map = tilt_south(map);
        map = tilt_east(map);

        let mut state: Vec<_> = map
            .iter()
            .map(|((x, y), r)| ((*x, *y), r.clone()))
            .collect();
        state.sort();

        if let Some(old) = past.insert(state, i) {
            let repeat = i - old;
            if offset.is_none() {
                offset = Some(old - 1);
            }
            let offset = offset.unwrap();

            if (old - offset) % repeat == (limit - offset) % repeat {
                return calc_load(&map);
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn draw_map(tilemap: &TileMap) {
    let (max_x, max_y) = (
        tilemap.keys().map(|p| p.0).max().unwrap(),
        tilemap.keys().map(|p| p.1).max().unwrap(),
    );

    let (min_x, min_y) = (
        tilemap.keys().map(|p| p.0).min().unwrap(),
        tilemap.keys().map(|p| p.1).min().unwrap(),
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let t = tilemap.get(&(x, y));
            let c = match t {
                Some(Rock::Round) => 'O',
                Some(Rock::Cube) => '#',
                None => '.',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
