fn p1(instr: &str) -> i64 {
    let lines: Vec<_> = instr
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut sum = 0;
    for l in &lines {
        let mut stack = vec![l.clone()];
        loop {
            let next: Vec<_> = stack
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            if next.iter().all(|v| *v == 0) {
                sum += stack.iter().map(|l| l.last().unwrap()).sum::<i64>();
                break;
            }
            stack.push(next);
        }
    }
    sum
}

fn p2(instr: &str) -> i64 {
    let lines: Vec<_> = instr
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut sum = 0;
    for l in &lines {
        let mut stack = vec![l.clone()];
        loop {
            let next: Vec<_> = stack
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            if next.iter().all(|v| *v == 0) {
                sum += stack
                    .iter()
                    .rev()
                    .map(|l| l.first().unwrap())
                    .fold(0, |acc, f| f - acc);
                break;
            }
            stack.push(next);
        }
    }
    sum
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
