use std::cmp::Ordering;

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

enum GameOutcome {
    Win,
    Lose,
    Draw,
}

impl GameOutcome {
    fn from_str(letter: &str) -> Self {
        match letter.chars().next().unwrap() {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            Some(match (self, other) {
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => Ordering::Greater,
                _ => Ordering::Less,
            })
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn from_str(letter: &str) -> Self {
        match letter.chars().next().unwrap() {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn pivot_hand(&self, outcome: GameOutcome) -> Self {
        match self {
            Self::Rock => match outcome {
                GameOutcome::Win => Self::Paper,
                GameOutcome::Draw => Self::Rock,
                GameOutcome::Lose => Self::Scissors,
            },
            Self::Paper => match outcome {
                GameOutcome::Win => Self::Scissors,
                GameOutcome::Draw => Self::Paper,
                GameOutcome::Lose => Self::Rock,
            },
            Self::Scissors => match outcome {
                GameOutcome::Win => Self::Rock,
                GameOutcome::Draw => Self::Scissors,
                GameOutcome::Lose => Self::Paper,
            },
        }
    }

    fn play(&self, opponent: Self) -> u32 {
        self.score()
            + match self.cmp(&opponent) {
                Ordering::Greater => 6,
                Ordering::Equal => 3,
                Ordering::Less => 0,
            }
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let total_score = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let opponent = Hand::from_str(iter.next().unwrap());
            let you = Hand::from_str(iter.next().unwrap());
            you.play(opponent)
        })
        .sum::<u32>();
    println!("total score: {total_score}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let total_score = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let opponent = Hand::from_str(iter.next().unwrap());
            let outcome = GameOutcome::from_str(iter.next().unwrap());
            let you = opponent.pivot_hand(outcome);
            you.play(opponent)
        })
        .sum::<u32>();
    println!("total score: {total_score}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hands() {
        let rock = Hand::Rock;
        let paper = Hand::Paper;
        let scissors = Hand::Scissors;

        assert_eq!(rock, Hand::from_str("A"));
        assert_eq!(rock, Hand::from_str("X"));
        assert_eq!(paper, Hand::from_str("B"));
        assert_eq!(paper, Hand::from_str("Y"));
        assert_eq!(scissors, Hand::from_str("C"));
        assert_eq!(scissors, Hand::from_str("Z"));

        assert_eq!(rock.play(rock), 1 + 3);
        assert_eq!(rock.play(paper), 1 + 0);
        assert_eq!(rock.play(scissors), 1 + 6);

        assert_eq!(paper.play(rock), 2 + 6);
        assert_eq!(paper.play(paper), 2 + 3);
        assert_eq!(paper.play(scissors), 2 + 0);

        assert_eq!(scissors.play(rock), 3 + 0);
        assert_eq!(scissors.play(paper), 3 + 6);
        assert_eq!(scissors.play(scissors), 3 + 3);
    }
}
