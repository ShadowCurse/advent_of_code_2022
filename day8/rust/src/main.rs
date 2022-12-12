const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    // part_2()?;
    Ok(())
}

struct Forest {
    trees: Vec<u8>,
    height: usize,
    width: usize,
}

impl Forest {
    fn from_string(s: String) -> Self {
        let width = s.find('\n').unwrap() + 1;
        let height = s.len() / width;
        let width = width - 1;
        Self {
            trees: s.into_bytes(),
            height,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.trees[x + y * self.width + y] - b'0'
    }

    fn calc_visible(&self) -> u32 {
        let mut visible = vec![0; self.width * self.height];
        for x in 0..self.width {
            visible[x] = 1;
            visible[x + self.width * (self.height - 1)] = 1;
        }
        for y in 0..self.height {
            visible[self.width * y] = 1;
            visible[(self.height - 1) + self.width * y] = 1;
        }
        // top -> down
        let mut max_horizontal_hights = vec![0; self.width];
        for y in 0..self.height {
            for x in 0..self.width {
                if max_horizontal_hights[x] < self.get(x, y) {
                    visible[x + self.width * y] = 1;
                    max_horizontal_hights[x] = self.get(x, y);
                }
            }
        }
        // down -> up
        let mut max_horizontal_hights = vec![0; self.width];
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if max_horizontal_hights[x] < self.get(x, y) {
                    visible[x + self.width * y] = 1;
                    max_horizontal_hights[x] = self.get(x, y);
                }
            }
        }
        // left -> right
        let mut max_vertical_hights = vec![0; self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                if max_vertical_hights[y] < self.get(x, y) {
                    visible[x + self.width * y] = 1;
                    max_vertical_hights[y] = self.get(x, y);
                }
            }
        }
        // right -> left
        let mut max_vertical_hights = vec![0; self.height];
        for x in (0..self.width).rev() {
            for y in 0..self.height {
                if max_vertical_hights[y] < self.get(x, y) {
                    visible[x + self.width * y] = 1;
                    max_vertical_hights[y] = self.get(x, y);
                }
            }
        }

        visible.iter().sum()
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let forest = Forest::from_string(input);
    let visible = forest.calc_visible();
    println!("visible: {visible}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forest() {
        let input = "30373
25512
65332
33549
35390
";
        let forest = Forest::from_string(input.to_string());
        let visible = forest.calc_visible();
        assert_eq!(visible, 21);
    }
}
