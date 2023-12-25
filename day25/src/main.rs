use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Edge<'a> = (&'a str, &'a str);

fn p1(instr: &str) -> usize {
    let pairs = instr.lines().flat_map(|l| {
        let (name, cons) = l.split_once(": ").unwrap();
        cons.split_whitespace().map(move |con| (name, con))
    });

    let mut lut = HashMap::new();
    let mut cons = vec![];

    for (a, b) in pairs {
        lut.entry(a)
            .and_modify(|v: &mut Vec<&str>| v.push(b))
            .or_insert(vec![b]);

        lut.entry(b)
            .and_modify(|v: &mut Vec<&str>| v.push(a))
            .or_insert(vec![a]);

        cons.push((a, b));
    }

    let distance = |src, dst: &str, skip: &[Edge]| {
        let mut frontier = BinaryHeap::from([Reverse((0, src))]);
        let mut visited = HashSet::new();
        while let Some(Reverse((d, pos))) = frontier.pop() {
            if pos == dst {
                return d;
            }
            visited.insert(pos);
            for &dst in &lut[&pos] {
                if skip.contains(&(pos, dst)) || skip.contains(&(dst, pos)) {
                    continue;
                }

                if !visited.contains(dst) {
                    frontier.push(Reverse((d + 1, dst)));
                }
            }
        }
        unreachable!();
    };

    let reachables = |init, skip: &[Edge]| {
        let mut frontier = vec![init];
        let mut visited = HashSet::new();
        while let Some(pos) = frontier.pop() {
            visited.insert(pos);
            for &dst in &lut[&pos] {
                if skip.contains(&(pos, dst)) || skip.contains(&(dst, pos)) {
                    continue;
                }

                if !visited.contains(dst) {
                    frontier.push(dst);
                }
            }
        }
        visited
    };

    let count_islands = |skip: &[(&str, &str)]| {
        let mut past: Vec<HashSet<&str>> = vec![];
        'outer: for n in lut.keys() {
            for p in &past {
                if p.contains(n) {
                    continue 'outer;
                }
            }
            let visitable = reachables(n, skip);
            past.push(visitable);
        }
        past
    };

    let mut distances = vec![];
    for (a, b) in &cons {
        let d = distance(a, b, &[(a, b)]);
        distances.push((d, (*a, *b)));
    }
    distances.sort();
    distances.reverse();

    let ans = count_islands(&[distances[0].1, distances[1].1, distances[2].1]);
    assert_eq!(ans.len(), 2);
    return ans[0].len() * ans[1].len();

    // graphviz baby
    // dot -T svg -o sln.svg sln.dot
    let header = "graph G {\nlayout=sfdp";
    println!("{header}");
    for k in lut.keys() {
        println!("{k}");
    }
    for c in &cons {
        println!("{} -- {}", c.0, c.1);
    }
    println!("}}");

    let graphviz_ans = [
        ("pnz", "tmt"),
        ("pnz", "gbc"),
        ("pnz", "mvv"),
        ("xkz", "tmt"),
        ("xkz", "gbc"),
        ("xkz", "mvv"),
        ("hxr", "tmt"),
        ("hxr", "gbc"),
        ("hxr", "mvv"),
    ];
    let ans = count_islands(&graphviz_ans);
    assert_eq!(ans.len(), 2);
    ans[0].len() * ans[1].len()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
}
