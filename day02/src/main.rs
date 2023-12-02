use std::str::FromStr;

#[derive(Debug)]
enum Color {
    R(u32),
    G(u32),
    B(u32),
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cnt_str, color_str) = s.trim().split_once(' ').unwrap();
        let cnt: u32 = cnt_str.parse().unwrap();
        match color_str {
            "red" => Ok(Color::R(cnt)),
            "green" => Ok(Color::G(cnt)),
            "blue" => Ok(Color::B(cnt)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Color>>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_str, rounds_str) = s.split_once(':').unwrap();
        let id: u32 = id_str.split_once(' ').unwrap().1.parse().unwrap();

        let rounds = rounds_str
            .split(';')
            .map(|r| r.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();

        Ok(Game { id, rounds })
    }
}

fn p1(instr: &str) -> u32 {
    let games: Vec<Game> = instr.lines().map(|l| l.parse().unwrap()).collect();
    let rule_r = 12;
    let rule_g = 13;
    let rule_b = 14;
    let sum = games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.iter().all(|color| match color {
                    Color::R(cnt) if *cnt <= rule_r => true,
                    Color::G(cnt) if *cnt <= rule_g => true,
                    Color::B(cnt) if *cnt <= rule_b => true,
                    _ => false,
                })
            })
        })
        .map(|game| game.id)
        .sum();
    sum
}

fn p2(instr: &str) -> u32 {
    let games: Vec<Game> = instr.lines().map(|l| l.parse().unwrap()).collect();
    let mut sum = 0;

    for game in &games {
        let (mut max_r, mut max_b, mut max_g) = (0, 0, 0);
        for round in &game.rounds {
            for c in round {
                match c {
                    Color::R(cnt) => max_r = max_r.max(*cnt),
                    Color::G(cnt) => max_g = max_g.max(*cnt),
                    Color::B(cnt) => max_b = max_b.max(*cnt),
                }
            }
        }
        sum += max_r * max_g * max_b;
    }
    sum
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", p1(&instr));
    println!("Part 2: {}", p2(&instr));
}
