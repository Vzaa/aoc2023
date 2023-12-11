use std::collections::HashSet;

type Pos = (i64, i64);
type TileMap = HashSet<Pos>;

fn m_dist(a: Pos, b: Pos) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn sln(instr: &str, offset: i64) -> i64 {
    let mut tilemap: TileMap = HashSet::new();

    let cols: HashSet<_> = instr
        .lines()
        .take(1)
        .flat_map(|l| {
            l.chars().enumerate().map(|(x, _)| x).filter(|x| {
                instr
                    .lines()
                    .map(|l| l.chars().nth(*x).unwrap())
                    .all(|c| c == '.')
            })
        })
        .collect();

    let mut offset_y = 0;
    for (y, line) in instr.lines().enumerate() {
        let mut offset_x = 0;
        let len_before = tilemap.len();
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                tilemap.insert((x as i64 + offset_x, y as i64 + offset_y));
            } else if cols.contains(&x) {
                offset_x += offset - 1;
            }
        }
        if tilemap.len() == len_before {
            offset_y += offset - 1;
        }
    }

    tilemap
        .iter()
        .map(|t| tilemap.iter().map(|&o| m_dist(o, *t)).sum::<i64>())
        .sum::<i64>()
        / 2
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", sln(&instr, 2));
    println!("Part 2: {}", sln(&instr, 1000000));
}
