fn p1(instr: &str) -> u64 {
    let mut iter = instr.lines();
    let time_str = iter.next().unwrap();
    let dist_str = iter.next().unwrap();

    let times: Vec<u64> = time_str
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let dists: Vec<u64> = dist_str
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut mult = 1;
    for (&t, &d) in times.iter().zip(dists.iter()) {
        let mut cnt = 0;
        for i in 0..t {
            if (t - i) * i > d {
                cnt += 1;
            }
        }
        mult *= cnt;
    }
    mult
}

fn p2(instr: &str) -> u64 {
    let mut iter = instr.lines();
    let time_str = iter.next().unwrap();
    let dist_str = iter.next().unwrap();

    let t: u64 = time_str
        .split_whitespace()
        .skip(1)
        .fold("".to_owned(), |acc, s| acc + s)
        .parse()
        .unwrap();
    let d: u64 = dist_str
        .split_whitespace()
        .skip(1)
        .fold("".to_owned(), |acc, s| acc + s)
        .parse()
        .unwrap();

    let mut cnt = 0;
    for i in 0..t {
        if (t - i) * i > d {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
