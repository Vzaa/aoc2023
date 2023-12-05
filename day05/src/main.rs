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

fn intersect_range(tgt: Range, src: Range) -> (Option<Range>, Vec<Range>) {
    let tgt_begin = tgt.0;
    let tgt_range = tgt.1;
    let tgt_end = tgt.0 + tgt.1 - 1;
    let src_begin = src.0;
    let src_end = src.0 + src.1 - 1;

    if src_begin >= tgt_begin && src_begin <= tgt_end {
        // src starts within tgt
        if src_end >= tgt_begin && src_end <= tgt_end {
            // src completly in tgt
            (Some(src), vec![])
        } else {
            // src ends outside tgt
            (
                Some((src_begin, tgt_range - (src_begin - tgt_begin))),
                vec![(tgt_begin + tgt_range, (src_end) - (tgt_end))],
            )
        }
    } else if src_end >= tgt_begin && src_end <= tgt_end {
        // src begins before tgt but ends in tgt
        (
            Some((tgt_begin, src_end - tgt_begin + 1)),
            vec![(src_begin, tgt_begin - src_begin)],
        )
    } else if src_begin < tgt_begin && src_end > tgt_end {
        // src contains tgt
        (
            Some(tgt),
            vec![
                (src_begin, tgt_begin - src_begin),
                (tgt_begin + tgt_range, src_end - tgt_end),
            ],
        )
    } else {
        (None, vec![src])
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
        intersect_range((self.src, self.range), n)
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
    for to_map in iter {
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
    for to_map in iter {
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
