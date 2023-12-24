type Pos = (i64, i64, i64);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

#[derive(Debug, Clone)]
struct Hail {
    p: Pos,
    v: Pos,
}

impl Hail {
    fn from_str(s: &str) -> Hail {
        let (p_str, v_str) = s.split_once(" @ ").unwrap();

        let mut iter = p_str.split(',').map(|n| n.trim().parse::<i64>().unwrap());
        let p = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        let mut iter = v_str.split(',').map(|n| n.trim().parse::<i64>().unwrap());
        let v = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        Hail { p, v }
    }

    fn intersect2(&self, other: &Hail) -> Option<(f64, f64)> {
        let (x1, y1, _) = self.p;
        let (x2, y2, _) = add(self.p, self.v);

        let (x3, y3, _) = other.p;
        let (x4, y4, _) = add(other.p, other.v);
        let x1 = x1 as i128;
        let x2 = x2 as i128;
        let x3 = x3 as i128;
        let x4 = x4 as i128;
        let y1 = y1 as i128;
        let y2 = y2 as i128;
        let y3 = y3 as i128;
        let y4 = y4 as i128;

        let denom = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

        if denom == 0 {
            return None;
        }

        let num_x = ((x1 * y2 - y1 * x2) * (x3 - x4)) - ((x1 - x2) * (x3 * y4 - y3 * x4));

        let num_y = ((x1 * y2 - y1 * x2) * (y3 - y4)) - ((y1 - y2) * (x3 * y4 - y3 * x4));

        Some((num_x as f64 / denom as f64, num_y as f64 / denom as f64))
    }

    fn future(&self, other: &Hail, lim_a: f64, lim_b: f64) -> bool {
        if let Some((x, y)) = self.intersect2(other) {
            if x >= lim_a
                && x <= lim_b
                && y >= lim_a
                && y <= lim_b
                && (x > self.p.0 as f64 && self.v.0 > 0 || x < self.p.0 as f64 && self.v.0 < 0)
                && (x > other.p.0 as f64 && other.v.0 > 0 || x < other.p.0 as f64 && other.v.0 < 0)
            {
                return true;
            }
        }
        false
    }

    fn future_int(&self, other: &Hail, lim_a: f64, lim_b: f64) -> Option<(i64, i64)> {
        if let Some((x, y)) = self.intersect2(other) {
            if x >= lim_a
                && x <= lim_b
                && y >= lim_a
                && y <= lim_b
                && (x > self.p.0 as f64 && self.v.0 > 0 || x < self.p.0 as f64 && self.v.0 < 0)
                && (x > other.p.0 as f64 && other.v.0 > 0 || x < other.p.0 as f64 && other.v.0 < 0)
                && (x - x.round()).abs() < 0.1
                && (y - y.round()).abs() < 0.1
            {
                return Some((x.round() as i64, y.round() as i64));
            }
        } else {
            // parallel
            return Some((0, 0));
        }
        None
    }
}

fn p1(instr: &str) -> usize {
    let hails: Vec<Hail> = instr.lines().map(Hail::from_str).collect();

    let lim_a = 200000000000000.0;
    let lim_b = 400000000000000.0;

    let mut cnt = 0;
    for (i, a) in hails.iter().enumerate() {
        for b in &hails[i + 1..] {
            if a.future(b, lim_a, lim_b) {
                cnt += 1;
            }
        }
    }

    cnt
}

fn brute_force(hails: &[Hail], swap: bool) -> (i64, i64) {
    let lim_a = 0.0;
    let lim_b = 400000000000000.0;
    for vx in -1000..1000 {
        'outer: for vy in -1000..1000 {
            let mut prev = None;
            for (i, a) in hails.iter().enumerate() {
                for b in &hails[i + 1..] {
                    let mut aa = a.clone();
                    let mut bb = b.clone();

                    if swap {
                        aa.p = (aa.p.0, aa.p.2, aa.p.2);
                        bb.p = (bb.p.0, bb.p.2, bb.p.2);
                        aa.v = (aa.v.0, aa.v.2, aa.v.2);
                        bb.v = (bb.v.0, bb.v.2, bb.v.2);
                    }

                    aa.v.0 -= vx;
                    bb.v.0 -= vx;
                    aa.v.1 -= vy;
                    bb.v.1 -= vy;

                    if let Some(p) = aa.future_int(&bb, lim_a, lim_b) {
                        if p == (0, 0) {
                            continue;
                        }
                        if let Some(pr) = prev {
                            if pr != p {
                                continue 'outer;
                            }
                        }
                        prev = Some(p);
                    } else {
                        continue 'outer;
                    }
                }
            }
            return prev.unwrap();
        }
    }
    unreachable!();
}

fn p2(instr: &str) -> i64 {
    let hails: Vec<Hail> = instr.lines().map(Hail::from_str).collect();
    let (_, y) = brute_force(&hails, false);
    let (x, z) = brute_force(&hails, true);
    println!("{x} {y} {z}");

    x + y + z
}
fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
