use std::str::FromStr;

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Crate(char);

impl From<Crate> for char {
    fn from(c: Crate) -> Self {
        c.0
    }
}

impl FromStr for Crate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(1).unwrap() {
            '-' => Err(()),
            c => Ok(Self(c)),
        }
    }
}

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = s.split_whitespace();
        items.next();
        let num = items.next().unwrap().parse::<usize>().unwrap();
        items.next();
        let from = items.next().unwrap().parse::<usize>().unwrap() - 1;
        items.next();
        let to = items.next().unwrap().parse::<usize>().unwrap() - 1;
        Ok(Self { num, from, to })
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<Crate>>,
}

impl Stacks {
    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.num {
            let c = self.stacks[m.from].pop().unwrap();
            self.stacks[m.to].push(c)
        }
    }

    fn apply_move_2(&mut self, m: &Move) {
        if m.from < m.to {
            let (from, to) = self.stacks.split_at_mut(m.to);
            let from = &mut from[m.from];
            let to = &mut to[0];
            to.extend_from_slice(&from[(from.len() - m.num)..]);
            from.resize(from.len() - m.num, Crate('-'));
        } else {
            let (to, from) = self.stacks.split_at_mut(m.from);
            let to = &mut to[m.to];
            let from = &mut from[0];
            to.extend_from_slice(&from[(from.len() - m.num)..]);
            from.resize(from.len() - m.num, Crate('-'));
        }
    }

    fn top_row(&self) -> Vec<Crate> {
        self.stacks.iter().map(|s| *s.last().unwrap()).collect()
    }
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let cols = lines.next().unwrap();
        let cols_num: usize = cols
            .split_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        let mut stacks = Self {
            stacks: vec![vec![]; cols_num],
        };
        for line in lines {
            for (i, c) in line.split_whitespace().map(Crate::from_str).enumerate() {
                if let Ok(c) = c {
                    stacks.stacks[i].push(c);
                }
            }
        }
        Ok(stacks)
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let mut part_iter = input.split("\n\n");
    let mut stacks: Stacks = part_iter.next().unwrap().parse().unwrap();
    let moves = part_iter.next().unwrap();
    for m in moves.lines() {
        let m: Move = m.parse().unwrap();
        stacks.apply_move(&m);
    }
    let top_row = stacks
        .top_row()
        .into_iter()
        .map(char::from)
        .collect::<String>();
    println!("top row: {top_row:?}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let mut part_iter = input.split("\n\n");
    let mut stacks: Stacks = part_iter.next().unwrap().parse().unwrap();
    let moves = part_iter.next().unwrap();
    for m in moves.lines() {
        let m: Move = m.parse().unwrap();
        stacks.apply_move_2(&m);
    }
    let top_row = stacks
        .top_row()
        .into_iter()
        .map(char::from)
        .collect::<String>();
    println!("top row: {top_row:?}");
    Ok(())
}
