fn p1(instr: &str) -> u32 {
    instr
        .lines()
        .map(|l| {
            let first: u32 = l.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last: u32 = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

fn p2(instr: &str) -> usize {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    instr
        .lines()
        .map(|l| {
            let first = (0..l.len())
                .map(|i| &l[i..])
                .find_map(|s| digits.iter().position(|&d| s.starts_with(d)))
                .map(|idx| (idx % 9) + 1)
                .unwrap();

            let last = (0..l.len())
                .rev()
                .map(|i| &l[i..])
                .find_map(|s| digits.iter().position(|&d| s.starts_with(d)))
                .map(|idx| (idx % 9) + 1)
                .unwrap();

            first * 10 + last
        })
        .sum()
}
fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
