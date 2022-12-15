use std::{collections::HashSet, fmt::Debug};

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

struct LongRope {
    knots: Vec<Pos>,
}

impl Debug for LongRope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..10).rev() {
            for x in 0..10 {
                if let Some(knot) = self.knots.iter().position(|knot| knot == &Pos { x, y }) {
                    let _ = write!(f, " {knot} ");
                } else {
                    let _ = write!(f, " . ");
                }
            }
            let _ = writeln!(f);
        }
        Ok(())
    }
}

impl LongRope {
    fn new(size: usize) -> Self {
        Self {
            knots: vec![Default::default(); size],
        }
    }

    fn head_mut(&mut self) -> &mut Pos {
        &mut self.knots[0]
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
        for head in 0..(self.knots.len() - 1) {
            let x_diff = self.knots[head].x - self.knots[head + 1].x;
            let y_diff = self.knots[head].y - self.knots[head + 1].y;
            // _|_|_
            // _|_|_
            // #|_|
            if x_diff >= 2 && y_diff >= 2 {
                self.knots[head + 1].y += 1;
                self.knots[head + 1].x += 1;
            }
            // _|_|_
            // #|_|_
            //  |_|
            else if x_diff >= 2 && y_diff < 2 && y_diff > -2 {
                self.knots[head + 1] = self.knots[head];
                self.knots[head + 1].x -= 1;
            }
            // #|_|_
            // _|_|_
            //  |_|
            else if x_diff >= 2 && y_diff <= -2 {
                self.knots[head + 1].y -= 1;
                self.knots[head + 1].x += 1;
            }
            // _|_|_
            // _|_|_
            //  |_|#
            else if x_diff <= -2 && y_diff >= 2 {
                self.knots[head + 1].y += 1;
                self.knots[head + 1].x -= 1;
            }
            // _|_|_
            // _|_|#
            //  |_|
            else if x_diff <= -2 && y_diff < 2 && y_diff > -2 {
                self.knots[head + 1] = self.knots[head];
                self.knots[head + 1].x += 1;
            }
            // _|_|#
            // _|_|_
            //  |_|
            else if x_diff <= -2 && y_diff <= -2 {
                self.knots[head + 1].y -= 1;
                self.knots[head + 1].x -= 1;
            }
            // _|_|
            // _|_|_
            //  |#|
            else if y_diff >= 2 && x_diff < 2 && x_diff > -2 {
                self.knots[head + 1] = self.knots[head];
                self.knots[head + 1].y -= 1;
            }
            // _|#|_
            // _|_|_
            //  |_|
            else if y_diff <= -2 && x_diff < 2 && x_diff > -2 {
                self.knots[head + 1] = self.knots[head];
                self.knots[head + 1].y += 1;
            }
        }
    }

    fn move_head_up(&mut self) {
        self.head_mut().y += 1;
        self.move_knots()
    }

    fn move_head_down(&mut self) {
        self.head_mut().y -= 1;
        self.move_knots()
    }

    fn move_head_left(&mut self) {
        self.head_mut().x -= 1;
        self.move_knots()
    }

    fn move_head_right(&mut self) {
        self.head_mut().x += 1;
        self.move_knots()
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let motions = input.lines().flat_map(Motion::from_str).collect::<Vec<_>>();
    let mut long_rope = LongRope::new(2);
    let mut set = HashSet::new();
    set.insert(Pos::default());
    for motion in motions {
        long_rope.apply_motion(motion);
        set.insert(long_rope.tail_pos());
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
        let mut long_rope = LongRope::new(2);
        let mut set = HashSet::new();
        set.insert(Pos::default());
        for motion in motions {
            println!("{long_rope:?}");
            long_rope.apply_motion(motion);
            set.insert(long_rope.tail_pos());
        }
        assert_eq!(set.len(), 13);
    }

    #[test]
    fn long_rope() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let motions = input.lines().flat_map(Motion::from_str).collect::<Vec<_>>();
        let mut long_rope = LongRope::new(10);
        let mut set = HashSet::new();
        set.insert(Pos::default());
        for motion in motions {
            println!("{long_rope:?}");
            long_rope.apply_motion(motion);
            set.insert(long_rope.tail_pos());
        }
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn long_rope_2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let motions = input.lines().flat_map(Motion::from_str).collect::<Vec<_>>();
        let mut long_rope = LongRope::new(10);
        let mut set = HashSet::new();
        set.insert(Pos::default());
        for motion in motions {
            println!("{long_rope:?}");
            long_rope.apply_motion(motion);
            set.insert(long_rope.tail_pos());
        }
        assert_eq!(set.len(), 36);
    }
}
