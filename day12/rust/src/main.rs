use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct HightMap {
    cells: Vec<u8>,
    width: usize,
    hight: usize,
    start: Pos,
    end: Pos,
    lows: Vec<Pos>,
}

impl FromStr for HightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self::default();
        for (y, line) in s.lines().enumerate() {
            map.width = line.len();
            map.hight += 1;
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    map.start = Pos { x, y };
                    map.cells.push(b'a');
                    map.lows.push(map.start);
                } else if c == 'E' {
                    map.end = Pos { x, y };
                    map.cells.push(b'z');
                } else {
                    if c == 'a' {
                        map.lows.push(Pos { x, y });
                    }
                    map.cells.push(c as u8);
                }
            }
        }
        Ok(map)
    }
}

impl HightMap {
    fn shortest_path_from_lows(&self) -> u32 {
        self.lows
            .iter()
            .filter_map(|low| self.shortest_path(*low))
            .min()
            .unwrap()
    }

    fn shortest_path_from_start(&self) -> u32 {
        self.shortest_path(self.start).unwrap()
    }

    fn shortest_path(&self, start: Pos) -> Option<u32> {
        let mut visited = HashMap::new();
        let mut queue = BinaryHeap::new();

        visited.insert(start, 0);
        queue.push(Reverse((0, start)));

        while let Some(Reverse((path, pos))) = queue.pop() {
            for new_pos in self.paths(pos) {
                visited
                    .entry(new_pos)
                    .and_modify(|val| *val = (*val).min(path + 1))
                    .or_insert_with(|| {
                        queue.push(Reverse((path + 1, new_pos)));
                        path + 1
                    });
            }
        }
        visited.get(&self.end).copied()
    }

    fn hight(&self, pos: Pos) -> u8 {
        self.cells[pos.x + pos.y * self.width]
    }

    fn paths(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        let check_add_width = |x| {
            if x + 1 < self.width {
                Some(x + 1)
            } else {
                None
            }
        };
        let check_add_hight = |y| {
            if y + 1 < self.hight {
                Some(y + 1)
            } else {
                None
            }
        };
        [
            (check_add_width(pos.x), Some(pos.y)),
            (Some(pos.x), check_add_hight(pos.y)),
            (pos.x.checked_sub(1), Some(pos.y)),
            (Some(pos.x), pos.y.checked_sub(1)),
        ]
        .into_iter()
        .filter_map(move |new_pos| match new_pos {
            (Some(x), Some(y)) => {
                let new_pos = Pos { x, y };
                if self.hight(pos) < self.hight(new_pos)
                    && 1 < self.hight(new_pos) - self.hight(pos)
                {
                    None
                } else {
                    Some(new_pos)
                }
            }
            _ => None,
        })
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let map = input.parse::<HightMap>().unwrap();
    let min_path = map.shortest_path_from_start();
    println!("min_path: {min_path}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let map = input.parse::<HightMap>().unwrap();
    let min_path = map.shortest_path_from_lows();
    println!("min_path: {min_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        let map = input.parse::<HightMap>().unwrap();
        assert_eq!(map.width, 8);
        assert_eq!(map.hight, 5);
        let min_path = map.shortest_path_from_start();
        assert_eq!(min_path, 31);
    }
}
