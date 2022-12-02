use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", part_1(&input)?);
    println!("{:?}", part_2(&input)?);
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
        match (self, other) {
            // Win
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => 6 + self.to_score(),
            // Tie
            _ if self == other => 3 + self.to_score(),
            // Loose
            _ => 0 + self.to_score(),
        }
    }
}

fn part_1(strategy_guide: &str) -> Result<usize, Box<dyn Error>> {
    Ok(strategy_guide
        .trim_end()
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
        .sum())
}

fn part_2(strategy_guide: &str) -> Result<usize, Box<dyn Error>> {
    Ok(strategy_guide
        .trim_end()
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
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_example_1() {
        let strategy_guide = "A Y
B X
C Z
";
        assert_eq!(15, part_1(strategy_guide).unwrap());
    }

    #[test]
    fn it_works_with_example_2() {
        let strategy_guide = "A Y
B X
C Z
";
        assert_eq!(12, part_2(strategy_guide).unwrap());
    }
}
