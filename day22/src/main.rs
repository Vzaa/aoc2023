use std::collections::HashSet;

type Pos = (i32, i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

#[derive(Debug, Clone)]
struct Brick {
    cubes: Vec<Pos>,
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let (a_str, b_str) = s.split_once('~').unwrap();

        let mut iter = a_str.split(',').map(|n| n.parse::<i32>().unwrap());
        let a = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        let mut iter = b_str.split(',').map(|n| n.parse::<i32>().unwrap());
        let b = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let cubes = if a.0 != b.0 {
            let min = a.0.min(b.0);
            let max = a.0.max(b.0);
            (min..=max).map(|v| (v, a.1, a.2)).collect()
        } else if a.1 != b.1 {
            let min = a.1.min(b.1);
            let max = a.1.max(b.1);
            (min..=max).map(|v| (a.0, v, a.2)).collect()
        } else if a.2 != b.2 {
            let min = a.2.min(b.2);
            let max = a.2.max(b.2);
            (min..=max).map(|v| (a.0, a.1, v)).collect()
        } else {
            vec![a]
        };
        Brick { cubes }
    }
}

fn fall(bricks: &mut [Brick]) -> Vec<usize> {
    let mut moved = vec![];
    let down = (0, 0, -1);
    let map: HashSet<Pos> = bricks
        .iter()
        .flat_map(|b| b.cubes.iter().cloned())
        .collect();

    for (idx, b) in bricks.iter_mut().enumerate() {
        let mut map = map.clone();
        for c in &b.cubes {
            map.remove(c);
        }
        if b.cubes
            .iter()
            .all(|&c| c.2 != 1 && !map.contains(&add(down, c)))
        {
            moved.push(idx);
            b.cubes.iter_mut().for_each(|c| *c = add(down, *c));
        }
    }

    moved
}

fn p1(instr: &str) -> usize {
    let mut bricks: Vec<_> = instr.lines().map(Brick::from_str).collect();

    while !fall(&mut bricks).is_empty() {}

    let mut cnt = 0;

    for i in 0..bricks.len() {
        let mut test = bricks.clone();
        test.swap_remove(i);

        if fall(&mut test).is_empty() {
            cnt += 1;
        }
    }

    cnt
}

fn p2(instr: &str) -> usize {
    let mut bricks: Vec<_> = instr.lines().map(Brick::from_str).collect();

    while !fall(&mut bricks).is_empty() {}

    let mut cnt = 0;

    for i in 0..bricks.len() {
        let mut test = bricks.clone();
        test.swap_remove(i);

        let mut list = HashSet::new();

        loop {
            let fell = fall(&mut test);
            if fell.is_empty() {
                break;
            }
            list.extend(fell);
        }

        cnt += list.len();
    }

    cnt
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
