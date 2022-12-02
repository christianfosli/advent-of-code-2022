use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn to_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn fight(&self, other: &Self) -> usize {
        self.to_score()
            + match (self, other) {
                // Win
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => 6,
                // Tie
                _ if self == other => 3,
                // Loose
                _ => 0,
            }
    }
}

fn part_1(strategy_guide: &str) -> usize {
    strategy_guide
        .lines()
        .map(|round| {
            if let [their, mine] = round.split(' ').collect::<Vec<_>>()[..] {
                let their = match their {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissors,
                    _ => unreachable!(),
                };
                let mine = match mine {
                    "X" => RPS::Rock,
                    "Y" => RPS::Paper,
                    "Z" => RPS::Scissors,
                    _ => unreachable!(),
                };
                return mine.fight(&their);
            }
            unreachable!();
        })
        .sum()
}

fn part_2(strategy_guide: &str) -> usize {
    strategy_guide
        .lines()
        .map(|round| {
            if let [their, result] = round.split(' ').collect::<Vec<_>>()[..] {
                let their = match their {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissors,
                    _ => unreachable!(),
                };
                let mine = match result {
                    "X" => match their {
                        // need to lose
                        RPS::Rock => RPS::Scissors,
                        RPS::Paper => RPS::Rock,
                        RPS::Scissors => RPS::Paper,
                    },
                    "Y" => their, // tied
                    "Z" => match their {
                        // need to win
                        RPS::Rock => RPS::Paper,
                        RPS::Paper => RPS::Scissors,
                        RPS::Scissors => RPS::Rock,
                    },
                    _ => unreachable!(),
                };
                return mine.fight(&their);
            }
            unreachable!();
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRATEGY_GUIDE: &'static str = "A Y
B X
C Z
";

    #[test]
    fn it_works_with_example_1() {
        assert_eq!(15, part_1(TEST_STRATEGY_GUIDE));
    }

    #[test]
    fn it_works_with_example_2() {
        assert_eq!(12, part_2(TEST_STRATEGY_GUIDE));
    }
}
