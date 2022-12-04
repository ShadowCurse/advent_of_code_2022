#![feature(iter_array_chunks)]

use std::collections::HashSet;

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let total = input
        .lines()
        .map(|line| {
            let compartment1 = HashSet::<char>::from_iter(line[..line.len() / 2].chars());
            let compartment2 = HashSet::<char>::from_iter(line[line.len() / 2..].chars());
            let intersection = compartment1.intersection(&compartment2).collect::<Vec<_>>();
            *intersection[0] as u8
        })
        .map(|c| {
            if c.is_ascii_uppercase() {
                (c - b'A' + 27) as u32
            } else {
                (c - b'a' + 1) as u32
            }
        })
        .sum::<u32>();
    println!("total: {total}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let total = input
        .lines()
        .array_chunks::<3>()
        .map(|lines| {
            let line1 = HashSet::<char>::from_iter(lines[0].chars());
            let line2 = HashSet::<char>::from_iter(lines[1].chars());
            let line3 = HashSet::<char>::from_iter(lines[2].chars());
            let intersection_1_2 = line1.intersection(&line2).copied().collect::<HashSet<_>>();
            let intersection = intersection_1_2.intersection(&line3).collect::<Vec<_>>();
            *intersection[0] as u8
        })
        .map(|c| {
            if c.is_ascii_uppercase() {
                (c - b'A' + 27) as u32
            } else {
                (c - b'a' + 1) as u32
            }
        })
        .sum::<u32>();
    println!("total: {total}");
    Ok(())
}
