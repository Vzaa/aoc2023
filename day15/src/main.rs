use std::array::from_fn;

fn hash(s: &str) -> u32 {
    s.trim()
        .chars()
        .map(|c| c as u32)
        .fold(0, |acc, c| ((c + acc) * 17) % 256)
}

fn p1(instr: &str) -> u32 {
    instr.split(',').map(hash).sum()
}

fn p2(instr: &str) -> u32 {
    let inputs: Vec<_> = instr.trim().split(',').collect();
    let mut boxes: [Vec<_>; 256] = from_fn(|_| vec![]);

    for input in &inputs {
        if let Some((h, _)) = input.split_once('-') {
            let hashed = hash(h) as usize;
            if let Some(idx) = boxes[hashed].iter().position(|(s, _)| *s == h) {
                boxes[hashed].remove(idx);
            }
        } else if let Some((h, n)) = input.split_once('=') {
            let hashed = hash(h) as usize;
            let num: u32 = n.parse().unwrap();
            if let Some(idx) = boxes[hashed].iter().position(|(s, _)| *s == h) {
                boxes[hashed][idx] = (h, num);
            } else {
                boxes[hashed].push((h, num));
            }
        } else {
            unreachable!()
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_id, b)| {
            b.iter()
                .enumerate()
                .map(|(slot_id, (_, f))| (box_id as u32 + 1) * (slot_id as u32 + 1) * (*f))
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
