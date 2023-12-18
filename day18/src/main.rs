type Pos = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

fn dir_char(c: char) -> Dir {
    match c {
        'U' => Dir::N,
        'D' => Dir::S,
        'L' => Dir::W,
        'R' => Dir::E,
        _ => unreachable!(),
    }
}

fn dir_char2(c: char) -> Dir {
    match c {
        '3' => Dir::N,
        '1' => Dir::S,
        '2' => Dir::W,
        '0' => Dir::E,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Dig {
    d: Dir,
    l: i64,
}

impl Dig {
    fn from_str(s: &str) -> Dig {
        let mut iter = s.split_whitespace();
        let d = dir_char(iter.next().unwrap().chars().next().unwrap());
        let l = iter.next().unwrap().parse().unwrap();
        Dig { d, l }
    }

    fn from_str2(s: &str) -> Dig {
        let hex_str = s.split_whitespace().nth(2).unwrap();
        let l = i64::from_str_radix(&hex_str[2..7], 16).unwrap();
        let d = dir_char2(hex_str.chars().nth(7).unwrap());
        Dig { d, l }
    }
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn solve(instr: &str, parse: fn(&str) -> Dig) -> i64 {
    let plan: Vec<_> = instr.lines().map(parse).collect();

    let mut pos = (0, 0);
    let mut points = vec![pos];
    let mut len = 0;

    for dig in &plan {
        pos = match dig.d {
            Dir::N => add(pos, (0, -dig.l)),
            Dir::S => add(pos, (0, dig.l)),
            Dir::W => add(pos, (-dig.l, 0)),
            Dir::E => add(pos, (dig.l, 0)),
        };
        points.push(pos);
        len += dig.l;
    }

    let mut area = 0;
    for p in points.windows(2) {
        area += (p[1].1 + p[0].1) * (p[0].0 - p[1].0);
    }
    let inner = (area.abs() / 2) - (len / 2) + 1;

    inner + len
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", solve(&instr, Dig::from_str));
    println!("Part 2: {}", solve(&instr, Dig::from_str2));
}
