use std::str::FromStr;

type Range = (u64, u64);

#[derive(Debug)]
struct MapRule {
    dst: u64,
    src: u64,
    range: u64,
}

impl FromStr for MapRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let dst = iter.next().unwrap().parse().unwrap();
        let src = iter.next().unwrap().parse().unwrap();
        let range = iter.next().unwrap().parse().unwrap();

        Ok(MapRule { dst, src, range })
    }
}

impl MapRule {
    fn get(&self, n: u64) -> Option<u64> {
        if n >= self.src && n <= self.src + self.range {
            Some(self.dst + (n - self.src))
        } else {
            None
        }
    }

    fn intersect(&self, n: Range) -> (Option<Range>, Vec<Range>) {
        // ugly af, also should have been a separate function with ranges instead of a method here
        // for consistency
        if n.0 >= self.src && n.0 < self.src + self.range {
            if (n.0 + n.1 - 1) >= self.src && (n.0 + n.1 - 1) < self.src + self.range {
                (Some(n), vec![])
            } else {
                (
                    Some((n.0, self.range - (n.0 - self.src))),
                    vec![(
                        self.src + self.range,
                        (n.0 + n.1 - 1) - (self.src + self.range - 1),
                    )],
                )
            }
        } else if (n.0 + n.1 - 1) >= self.src && (n.0 + n.1 - 1) < self.src + self.range {
            (
                Some((self.src, (n.0 + n.1 - 1) - self.src + 1)),
                vec![(n.0, self.src - n.0)],
            )
        } else if n.0 < self.src && (n.0 + n.1 - 1) > self.src + self.range - 1 {
            (
                Some((self.src, self.range)),
                vec![
                    (n.0, self.src - n.0),
                    (
                        self.src + self.range,
                        (n.0 + n.1 - 1) - (self.src + self.range - 1),
                    ),
                ],
            )
        } else {
            (None, vec![n])
        }
    }

    fn conv(&self, n: Range) -> Range {
        (self.dst + (n.0 - self.src), n.1)
    }
}

fn parse_seeds(s: &str) -> Vec<u64> {
    let nums_str = s.split_once(':').unwrap().1.trim();
    nums_str
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_seeds2(s: &str) -> Vec<(u64, u64)> {
    let nums_str = s.split_once(':').unwrap().1.trim();
    let mut iter = nums_str.split_whitespace();

    let mut seeds = vec![];
    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();
        seeds.push((a, b));
    }
    seeds
}

fn p1(instr: &str) -> u64 {
    let mut iter = instr.split("\n\n");

    let seeds_str = iter.next().unwrap();
    let seeds = parse_seeds(seeds_str);

    let mut maps = vec![];
    while let Some(to_map) = iter.next() {
        let ranges: Vec<MapRule> = to_map.lines().skip(1).map(|l| l.parse().unwrap()).collect();
        maps.push(ranges);
    }

    seeds
        .iter()
        .map(|s| {
            let mut mapped = *s;
            for m in &maps {
                for r in m {
                    if let Some(v) = r.get(mapped) {
                        mapped = v;
                        break;
                    }
                }
            }
            mapped
        })
        .min()
        .unwrap()
}

fn p2(instr: &str) -> u64 {
    let mut iter = instr.split("\n\n");

    let seeds_str = iter.next().unwrap();
    let seeds = parse_seeds2(seeds_str);

    let mut maps = vec![];
    while let Some(to_map) = iter.next() {
        let ranges: Vec<MapRule> = to_map.lines().skip(1).map(|l| l.parse().unwrap()).collect();
        maps.push(ranges);
    }

    let mut cur = seeds.clone();
    for m in &maps {
        let mut next = vec![];
        for r in m {
            let mut remain = vec![];
            while let Some(s) = cur.pop() {
                let (intersection, extra) = r.intersect(s);
                if let Some(i) = intersection {
                    next.push(r.conv(i));
                }
                remain.extend(extra);
            }
            cur.extend(remain);
        }
        cur.extend(next);
    }

    cur.iter().map(|c| c.0).min().unwrap()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
