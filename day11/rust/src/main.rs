use std::collections::VecDeque;

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

struct Monkey {
    inspected: u64,
    starting_items: VecDeque<u64>,
    operation: fn(old: u64) -> u64,
    test: fn(val: u64) -> usize,
}

impl Monkey {
    fn add_item(&mut self, item: u64) {
        self.starting_items.push_back(item);
    }

    fn inspect(&mut self) -> Option<(u64, usize)> {
        if let Some(item) = self.starting_items.pop_front() {
            self.inspected += 1;
            let new_level = (self.operation)(item);
            let new_level = new_level / 3;
            let next_monkey = (self.test)(new_level);
            Some((new_level, next_monkey))
        } else {
            None
        }
    }

    fn inspect_2(&mut self) -> Option<(u64, usize)> {
        if let Some(item) = self.starting_items.pop_front() {
            self.inspected += 1;
            let new_level = (self.operation)(item);
            let next_monkey = (self.test)(new_level);
            Some((new_level, next_monkey))
        } else {
            None
        }
    }
}

fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            inspected: 0,
            starting_items: vec![92, 73, 86, 83, 65, 51, 55, 93].into(),
            operation: |old| old * 5,
            test: |val| {
                if val % 11 == 0 {
                    3
                } else {
                    4
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![99, 67, 62, 61, 59, 98].into(),
            operation: |old| old * old,
            test: |val| {
                if val % 2 == 0 {
                    6
                } else {
                    7
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![81, 89, 56, 61, 99].into(),
            operation: |old| old * 7,
            test: |val| {
                if val % 5 == 0 {
                    1
                } else {
                    5
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![97, 74, 68].into(),
            operation: |old| old + 1,
            test: |val| {
                if val % 17 == 0 {
                    2
                } else {
                    5
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![78, 73].into(),
            operation: |old| old + 3,
            test: |val| {
                if val % 19 == 0 {
                    2
                } else {
                    3
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![50].into(),
            operation: |old| old + 5,
            test: |val| {
                if val % 7 == 0 {
                    1
                } else {
                    6
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![95, 88, 53, 75].into(),
            operation: |old| old + 8,
            test: |val| {
                if val % 3 == 0 {
                    0
                } else {
                    7
                }
            },
        },
        Monkey {
            inspected: 0,
            starting_items: vec![50, 77, 98, 85, 94, 56, 89].into(),
            operation: |old| old + 2,
            test: |val| {
                if val % 13 == 0 {
                    4
                } else {
                    0
                }
            },
        },
    ]
}

fn part_1() -> Result<(), std::io::Error> {
    let mut monkeys = get_monkeys();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((new_level, next_monkey)) = monkeys[i].inspect() {
                monkeys[next_monkey].add_item(new_level);
            }
        }
    }

    let mut vals = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    vals.sort_unstable();
    let val = vals[vals.len() - 2] * vals[vals.len() - 1];
    println!("val: {val}");

    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let mut monkeys = get_monkeys();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some((new_level, next_monkey)) = monkeys[i].inspect_2() {
                monkeys[next_monkey].add_item(new_level);
            }
        }
    }

    let mut vals = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    vals.sort_unstable();
    let val = vals[vals.len() - 2] * vals[vals.len() - 1];
    println!("val: {val}");

    Ok(())
}
