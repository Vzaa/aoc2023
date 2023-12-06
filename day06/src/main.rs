fn p1(instr: &str) -> u64 {
    let mut iter = instr.lines().map(|l| {
        l.split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>()
    });
    let (times, dists) = (iter.next().unwrap(), iter.next().unwrap());

    times
        .iter()
        .zip(dists.iter())
        .map(|(&t, &d)| (0..t).filter(|i| (t - i) * i > d).count() as u64)
        .product()
}

fn p2(instr: &str) -> u64 {
    let mut iter = instr.lines().map(|l| {
        l.split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });
    let (t, d) = (iter.next().unwrap(), iter.next().unwrap());
    (0..t).filter(|i| (t - i) * i > d).count() as u64
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
