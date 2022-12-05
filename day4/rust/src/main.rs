use std::{str::FromStr, cmp::Ordering};

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

struct Range(std::ops::RangeInclusive<u32>);

impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn overlap(&self, other: &Range) -> bool {
        match self.0.start().cmp(other.0.start()) {
            Ordering::Equal => true,
            Ordering::Less => other.0.start() <= self.0.end(),
            Ordering::Greater => self.0.start() <= other.0.end() ,
        } 
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut val_iter = s.split('-');
        let left = val_iter.next().unwrap().parse::<u32>().unwrap();
        let right = val_iter.next().unwrap().parse::<u32>().unwrap();
        Ok(Self(left..=right))
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let total = input
        .lines()
        .map(|line| {
            let mut range_iter = line.split(',');
            let range1: Range = range_iter.next().unwrap().parse().unwrap();
            let range2: Range = range_iter.next().unwrap().parse().unwrap();
            u32::from(range1.fully_contains(&range2) || range2.fully_contains(&range1))
        })
        .sum::<u32>();
    println!("total: {total}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let total = input
        .lines()
        .map(|line| {
            let mut range_iter = line.split(',');
            let range1: Range = range_iter.next().unwrap().parse().unwrap();
            let range2: Range = range_iter.next().unwrap().parse().unwrap();
            u32::from(range1.overlap(&range2))
        })
        .sum::<u32>();
    println!("total: {total}");
    Ok(())
}
