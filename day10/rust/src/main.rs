use std::collections::VecDeque;

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Vec<Instruction> {
        match s.split_once(' ') {
            Some((_, num)) => {
                let n = num.parse().unwrap();
                vec![Instruction::Noop, Instruction::Addx(n)]
            }
            _ => {
                vec![Instruction::Noop]
            }
        }
    }
}

struct Cpu {
    reg_x: i32,
    in_cycle_reg_x: i32,
    cycles: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg_x: 1,
            in_cycle_reg_x: 1,
            cycles: 0,
        }
    }

    fn execute(&mut self, inst: Instruction) {
        self.cycles += 1;
        match inst {
            Instruction::Noop => self.in_cycle_reg_x = self.reg_x,
            Instruction::Addx(n) => {
                self.in_cycle_reg_x = self.reg_x;
                self.reg_x += n
            }
        }
    }

    fn in_cycle_reg_x(&self) -> i32 {
        self.in_cycle_reg_x
    }

    fn cycles(&self) -> i32 {
        self.cycles
    }
}

struct Crt {
    screen: Vec<char>,
    width: usize,
    height: usize,
}

impl Crt {
    fn new(height: usize, width: usize) -> Self {
        Self {
            screen: vec!['.'; height * width],
            width,
            height,
        }
    }

    fn update(&mut self, cycle: i32, x: i32) {
        let pos = (cycle - 1) % self.width as i32;
        if (x - 1..=x + 1).contains(&pos) {
            self.screen[(cycle - 1) as usize] = '#';
        }
    }

    fn print(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                print!("{}", self.screen[w + h * self.width]);
            }
            println!();
        }
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let mut instructions = input
        .lines()
        .flat_map(Instruction::from_str)
        .collect::<VecDeque<_>>();
    let mut cpu = Cpu::new();
    for _ in 0..20 {
        cpu.execute(instructions.pop_front().unwrap());
    }
    let mut total = cpu.in_cycle_reg_x() * 20;
    for _ in 0..5 {
        for _ in 0..40 {
            cpu.execute(instructions.pop_front().unwrap());
        }
        total += cpu.in_cycle_reg_x() * cpu.cycles();
    }

    println!("total: {total}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let instructions = input
        .lines()
        .flat_map(Instruction::from_str)
        .collect::<Vec<_>>();
    let mut cpu = Cpu::new();
    let mut crt = Crt::new(6, 40);
    for inst in instructions {
        cpu.execute(inst);
        crt.update(cpu.cycles(), cpu.in_cycle_reg_x());
    }
    crt.print();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let mut instructions = input
            .lines()
            .flat_map(Instruction::from_str)
            .collect::<VecDeque<_>>();
        let mut cpu = Cpu::new();
        for _ in 0..20 {
            cpu.execute(instructions.pop_front().unwrap());
        }
        let mut total = cpu.in_cycle_reg_x() * 20;
        for _ in 0..5 {
            for _ in 0..40 {
                cpu.execute(instructions.pop_front().unwrap());
            }
            total += cpu.in_cycle_reg_x() * cpu.cycles();
        }
        assert_eq!(total, 13140);
    }

    #[test]
    fn crt() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let instructions = input
            .lines()
            .flat_map(Instruction::from_str)
            .collect::<Vec<_>>();
        let mut cpu = Cpu::new();
        let mut crt = Crt::new(6, 40);
        for inst in instructions {
            cpu.execute(inst);
            crt.update(cpu.cycles(), cpu.in_cycle_reg_x());
        }
        crt.print();
    }
}
