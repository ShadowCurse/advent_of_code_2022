const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
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

    fn calc_scenic_score(&self) -> u32 {
        let mut max = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                max = max.max(self.scenic_score_of_tree(x, y));
            }
        }
        max
    }

    fn scenic_score_of_tree(&self, x: usize, y: usize) -> u32 {
        let mut max_vis_up = 0;
        for up_y in (0..y).rev() {
            max_vis_up += 1;
            if self.get(x, y) <= self.get(x, up_y) {
                break;
            }
        }
        let mut max_vis_down = 0;
        for down_y in (y + 1)..self.height {
            max_vis_down += 1;
            if self.get(x, y) <= self.get(x, down_y) {
                break;
            }
        }
        let mut max_vis_left = 0;
        for left_x in (0..x).rev() {
            max_vis_left += 1;
            if self.get(x, y) <= self.get(left_x, y) {
                break;
            }
        }
        let mut max_vis_right = 0;
        for right_x in (x + 1)..self.width {
            max_vis_right += 1;
            if self.get(x, y) <= self.get(right_x, y) {
                break;
            }
        }

        max_vis_up * max_vis_down * max_vis_left * max_vis_right
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let forest = Forest::from_string(input);
    let visible = forest.calc_visible();
    println!("visible: {visible}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let forest = Forest::from_string(input);
    let max_scenic_score = forest.calc_scenic_score();
    println!("max_scenic_score: {max_scenic_score}");
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
