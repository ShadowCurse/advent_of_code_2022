use std::collections::HashSet;

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

impl Motion {
    fn from_str(s: &str) -> Vec<Self> {
        let (dir, repeat) = s.split_once(' ').unwrap();
        let repeat = repeat.parse::<usize>().unwrap();
        match dir {
            "U" => vec![Motion::Up; repeat],
            "D" => vec![Motion::Down; repeat],
            "L" => vec![Motion::Left; repeat],
            "R" => vec![Motion::Right; repeat],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn dist(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() | (self.y - other.y).abs()
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: Pos,
    tail: Pos,
}

impl Rope {
    fn tail_pos(&self) -> Pos {
        self.tail
    }

    fn apply_motion(&mut self, motion: Motion) {
        match motion {
            Motion::Up => {
                self.move_head_up();
            }
            Motion::Down => {
                self.move_head_down();
            }
            Motion::Left => {
                self.move_head_left();
            }
            Motion::Right => {
                self.move_head_right();
            }
        }
    }

    fn move_head_up(&mut self) {
        self.head.y += 1;
        if 1 < self.head.dist(&self.tail) {
            self.tail.x = self.head.x;
            self.tail.y = self.head.y - 1;
        }
    }

    fn move_head_down(&mut self) {
        self.head.y -= 1;
        if 1 < self.head.dist(&self.tail) {
            self.tail.x = self.head.x;
            self.tail.y = self.head.y + 1;
        }
    }

    fn move_head_left(&mut self) {
        self.head.x -= 1;
        if 1 < self.head.dist(&self.tail) {
            self.tail.x = self.head.x + 1;
            self.tail.y = self.head.y;
        }
    }

    fn move_head_right(&mut self) {
        self.head.x += 1;
        if 1 < self.head.dist(&self.tail) {
            self.tail.x = self.head.x - 1;
            self.tail.y = self.head.y;
        }
    }
}

struct LongRope {
    knots: Vec<Pos>,
}

impl LongRope {
    fn new(size: usize) -> Self {
        Self {
            knots: vec![Default::default(); size],
        }
    }

    fn head_pos(&self) -> Pos {
        self.knots[0]
    }

    fn tail_pos(&self) -> Pos {
        self.knots[self.knots.len() - 1]
    }

    fn apply_motion(&mut self, motion: Motion) {
        match motion {
            Motion::Up => {
                self.move_head_up();
            }
            Motion::Down => {
                self.move_head_down();
            }
            Motion::Left => {
                self.move_head_left();
            }
            Motion::Right => {
                self.move_head_right();
            }
        }
    }

    fn move_knots(&mut self) {
        let range = 0..(self.knots.len() - 1);
        self.knots.copy_within(range, 1);
    }

    fn move_head_up(&mut self) {
        let mut new_head = self.head_pos();
        new_head.y += 1;
        if 1 < new_head.dist(&self.knots[1]) {
            self.move_knots()
        }
    }

    fn move_head_down(&mut self) {
        let mut new_head = self.head_pos();
        new_head.y -= 1;
        if 1 < new_head.dist(&self.knots[1]) {
            self.move_knots()
        }
    }

    fn move_head_left(&mut self) {
        let mut new_head = self.head_pos();
        new_head.x -= 1;
        if 1 < new_head.dist(&self.knots[1]) {
            self.move_knots()
        }
    }

    fn move_head_right(&mut self) {
        let mut new_head = self.head_pos();
        new_head.x += 1;
        if 1 < new_head.dist(&self.knots[1]) {
            self.move_knots()
        }
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let motions = input.lines().flat_map(Motion::from_str).collect::<Vec<_>>();
    let mut rope = Rope::default();
    let mut set = HashSet::new();
    set.insert(Pos::default());
    for motion in motions {
        rope.apply_motion(motion);
        set.insert(rope.tail_pos());
    }

    println!("total: {}", set.len());
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let motions = input.lines().flat_map(Motion::from_str).collect::<Vec<_>>();
    let mut long_rope = LongRope::new(10);
    let mut set = HashSet::new();
    set.insert(Pos::default());
    for motion in motions {
        long_rope.apply_motion(motion);
        set.insert(long_rope.tail_pos());
    }

    println!("total: {}", set.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rope() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let motions = input.lines().flat_map(Motion::from_str).collect::<Vec<_>>();
        let mut rope = Rope::default();
        let mut set = HashSet::new();
        set.insert(Pos::default());
        println!("{rope:#?}");
        for motion in motions {
            rope.apply_motion(motion);
            println!("{rope:#?}");
            set.insert(rope.tail_pos());
        }
        assert_eq!(set.len(), 13);
    }
}
