use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Lt(char, usize),
    Gt(char, usize),
}

impl Op {
    fn from_str(s: &str) -> Op {
        if let Some((s, n)) = s.split_once('<') {
            Op::Lt(s.chars().next().unwrap(), n.parse().unwrap())
        } else if let Some((s, n)) = s.split_once('>') {
            Op::Gt(s.chars().next().unwrap(), n.parse().unwrap())
        } else {
            unreachable!()
        }
    }

    fn apply(&self, part: &Part) -> bool {
        match self {
            Op::Lt('x', v) => part.x < *v,
            Op::Lt('m', v) => part.m < *v,
            Op::Lt('a', v) => part.a < *v,
            Op::Lt('s', v) => part.s < *v,
            Op::Gt('x', v) => part.x > *v,
            Op::Gt('m', v) => part.m > *v,
            Op::Gt('a', v) => part.a > *v,
            Op::Gt('s', v) => part.s > *v,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    check: Option<Op>,
    dst: &'a str,
}

impl Rule<'_> {
    fn from_str(s: &str) -> Rule {
        if let Some((op_str, dst)) = s.split_once(':') {
            Rule {
                check: Some(Op::from_str(op_str)),
                dst,
            }
        } else {
            Rule {
                check: None,
                dst: s,
            }
        }
    }

    fn apply(&self, part: &Part) -> bool {
        if let Some(op) = &self.check {
            op.apply(part)
        } else {
            true
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl Workflow<'_> {
    fn from_str(s: &str) -> Workflow {
        let (name, ops_str) = s.split_once('{').unwrap();
        let rules_str = ops_str.trim_matches('}');
        let rules = rules_str.split(',').map(|s| Rule::from_str(s)).collect();
        Workflow { name, rules }
    }

    fn apply(&self, part: &Part) -> &str {
        for r in &self.rules {
            if r.apply(part) {
                return r.dst;
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn from_str(s: &str) -> Part {
        let s = s.trim_matches(|c| c == '}' || c == '{');

        let mut iter = s
            .split(',')
            .map(|cat_str| cat_str.split_once('=').unwrap().1.parse().unwrap());

        let x = iter.next().unwrap();
        let m = iter.next().unwrap();
        let a = iter.next().unwrap();
        let s = iter.next().unwrap();

        Part { x, m, a, s }
    }
}

fn p1(instr: &str) -> usize {
    let (workflows_str, parts_str) = instr.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows_str
        .lines()
        .map(Workflow::from_str)
        .map(|w| (w.name, w))
        .collect();
    let parts: Vec<_> = parts_str.lines().map(Part::from_str).collect();

    let mut accept = vec![];
    let mut reject = vec![];

    for p in &parts {
        let mut wf = &workflows[&"in"];

        loop {
            let dst = wf.apply(p);
            if dst == "A" {
                accept.push(p);
                break;
            } else if dst == "R" {
                reject.push(p);
                break;
            } else {
                wf = &workflows[&dst];
            }
        }
    }

    accept.iter().map(|p| p.x + p.m + p.a + p.s).sum()
}

#[derive(Debug, Clone)]
enum Step<'a> {
    Pos(&'a Op),
    Neg(&'a Op),
}

fn p2(instr: &str) -> usize {
    let (workflows_str, _) = instr.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows_str
        .lines()
        .map(Workflow::from_str)
        .map(|w| (w.name, w))
        .collect();

    let mut frontier = vec![("in", vec![])];

    let mut accept_paths = vec![];

    while let Some((name, path)) = frontier.pop() {
        if name == "A" {
            accept_paths.push(path);
            continue;
        } else if name == "R" {
            continue;
        }

        let wf = &workflows[name];
        for (i, r) in wf.rules.iter().enumerate() {
            let mut npath = path.clone();
            if let Some(op) = &r.check {
                npath.push(Step::Pos(op));
            }
            let negatives = wf
                .rules
                .iter()
                .take(i)
                .map(|r| Step::Neg(r.check.as_ref().unwrap()));
            npath.extend(negatives);
            frontier.push((r.dst, npath.clone()));
        }
    }

    let mut sum = 0;
    for p in &accept_paths {
        let mut x: Vec<_> = (1..=4000).collect();
        let mut m: Vec<_> = (1..=4000).collect();
        let mut a: Vec<_> = (1..=4000).collect();
        let mut s: Vec<_> = (1..=4000).collect();

        for op in p {
            match op {
                Step::Pos(Op::Lt('x', v)) => x.retain(|vv| vv < v),
                Step::Pos(Op::Lt('m', v)) => m.retain(|vv| vv < v),
                Step::Pos(Op::Lt('a', v)) => a.retain(|vv| vv < v),
                Step::Pos(Op::Lt('s', v)) => s.retain(|vv| vv < v),
                Step::Pos(Op::Gt('x', v)) => x.retain(|vv| vv > v),
                Step::Pos(Op::Gt('m', v)) => m.retain(|vv| vv > v),
                Step::Pos(Op::Gt('a', v)) => a.retain(|vv| vv > v),
                Step::Pos(Op::Gt('s', v)) => s.retain(|vv| vv > v),

                Step::Neg(Op::Lt('x', v)) => x.retain(|vv| vv >= v),
                Step::Neg(Op::Lt('m', v)) => m.retain(|vv| vv >= v),
                Step::Neg(Op::Lt('a', v)) => a.retain(|vv| vv >= v),
                Step::Neg(Op::Lt('s', v)) => s.retain(|vv| vv >= v),
                Step::Neg(Op::Gt('x', v)) => x.retain(|vv| vv <= v),
                Step::Neg(Op::Gt('m', v)) => m.retain(|vv| vv <= v),
                Step::Neg(Op::Gt('a', v)) => a.retain(|vv| vv <= v),
                Step::Neg(Op::Gt('s', v)) => s.retain(|vv| vv <= v),
                _ => unreachable!(),
            };
        }
        sum += x.len() * m.len() * a.len() * s.len();
    }

    sum
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
