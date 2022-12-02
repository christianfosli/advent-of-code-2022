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
    fn to_score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn fight(self, other: Self) -> usize {
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

fn part_1(strategy_guide: &str) -> Result<usize, Box<dyn Error>> {
    Ok(strategy_guide
        .lines()
        .map(|round| {
            if let [their, mine] = round.split(' ').collect::<Vec<_>>()[..] {
                let their = match their {
                    "A" => Ok(RPS::Rock),
                    "B" => Ok(RPS::Paper),
                    "C" => Ok(RPS::Scissors),
                    _ => Err("Invalid first char in strategy round"),
                }?;
                let mine = match mine {
                    "X" => Ok(RPS::Rock),
                    "Y" => Ok(RPS::Paper),
                    "Z" => Ok(RPS::Scissors),
                    _ => Err("Invalid last char in strategy round"),
                }?;
                return Ok(mine.fight(their));
            }
            Err("Invalid format for round")
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
}

fn part_2(strategy_guide: &str) -> Result<usize, Box<dyn Error>> {
    Ok(strategy_guide
        .lines()
        .map(|round| {
            if let [their, result] = round.split(' ').collect::<Vec<_>>()[..] {
                let their = match their {
                    "A" => Ok(RPS::Rock),
                    "B" => Ok(RPS::Paper),
                    "C" => Ok(RPS::Scissors),
                    _ => Err("Invalid first char in strategy round"),
                }?;
                let mine = match result {
                    "X" => match their {
                        // need to lose
                        RPS::Rock => Ok(RPS::Scissors),
                        RPS::Paper => Ok(RPS::Rock),
                        RPS::Scissors => Ok(RPS::Paper),
                    },
                    "Y" => Ok(their), // tied
                    "Z" => match their {
                        // need to win
                        RPS::Rock => Ok(RPS::Paper),
                        RPS::Paper => Ok(RPS::Scissors),
                        RPS::Scissors => Ok(RPS::Rock),
                    },
                    _ => Err("Invalid last char in strategy round"),
                }?;
                return Ok(mine.fight(their));
            }
            Err("Invalid format for round")
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
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
        assert_eq!(15, part_1(TEST_STRATEGY_GUIDE).unwrap());
    }

    #[test]
    fn it_works_with_example_2() {
        assert_eq!(12, part_2(TEST_STRATEGY_GUIDE).unwrap());
    }
}
