use std::collections::HashMap;

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

#[derive(Debug)]
struct UniqueSet {
    map: HashMap<u8, u8>,
    repeats: u32,
}

impl UniqueSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            repeats: 0,
        }
    }

    fn add(&mut self, item: u8) {
        if self.map.contains_key(&item) {
            *self.map.get_mut(&item).unwrap() += 1;
            self.repeats += 1;
        } else {
            self.map.insert(item, 1);
        }
    }

    fn remove(&mut self, item: u8) {
        *self.map.get_mut(&item).unwrap() -= 1;
        if self.map[&item] == 0 {
            self.map.remove(&item);
        } else {
            self.repeats -= 1;
        }
    }

    fn unique(&self) -> bool {
        self.repeats == 0
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let bytes = input.into_bytes();
    let mut unique_set = UniqueSet::new();
    for i in 0..4 {
        unique_set.add(bytes[i]);
    }
    for i in 4..bytes.len() {
        if unique_set.unique() {
            println!("index: {}", i);
            break;
        }
        unique_set.add(bytes[i]);
        unique_set.remove(bytes[i - 4]);
    }
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let bytes = input.into_bytes();
    let mut unique_set = UniqueSet::new();
    for i in 0..14 {
        unique_set.add(bytes[i]);
    }
    for i in 14..bytes.len() {
        if unique_set.unique() {
            println!("index: {}", i);
            break;
        }
        unique_set.add(bytes[i]);
        unique_set.remove(bytes[i - 14]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_set() {
        let mut us = UniqueSet::new();

        us.add(0);
        us.add(1);
        us.add(2);

        assert!(us.unique());

        us.add(0);
        assert!(!us.unique());
        us.add(1);
        assert!(!us.unique());
        us.add(2);
        assert!(!us.unique());

        us.remove(0);
        assert!(!us.unique());
        us.remove(1);
        assert!(!us.unique());
        us.remove(2);
        assert!(us.unique());

        us.add(1);
        us.remove(1);
        assert!(us.unique());

        us.add(3);
        us.remove(3);
        assert!(us.unique());
    }

    #[test]
    fn unique_set_testcases() {
        fn test_ans(bytes: &[u8], ans: usize) {
            let mut us = UniqueSet::new();
            for i in 0..4 {
                us.add(bytes[i]);
            }
            let mut res = 0;
            for i in 4..bytes.len() {
                if us.unique() {
                    res = i;
                    break;
                }
                us.add(bytes[i]);
                us.remove(bytes[i - 4]);
            }
            assert_eq!(res, ans)
        }
        let bytes = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();
        test_ans(bytes, 7);
        let bytes = "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes();
        test_ans(bytes, 5);
        let bytes = "nppdvjthqldpwncqszvftbrmjlhg".as_bytes();
        test_ans(bytes, 6);
        let bytes = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes();
        test_ans(bytes, 10);
        let bytes = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes();
        test_ans(bytes, 11);
    }
}
