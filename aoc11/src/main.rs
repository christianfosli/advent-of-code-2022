use std::{
    error::Error,
    fmt::{self, Debug, Formatter},
    fs,
};

use regex::Regex;

type WorryLevel = usize;

struct Monkey {
    items: Vec<WorryLevel>,
    op: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test: Box<dyn Fn(WorryLevel) -> usize>, // returns index of "throw to monkey"
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey, items: {:?}", self.items)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut monkeys = parse_monkeys(&input)?;
    println!("Part 1: {}", part_1_level_of_monkey_business(&mut monkeys));
    Ok(())
}

fn parse_monkeys(s: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let op_mul_re = Regex::new(r"new = old \* (.+)")?;
    let op_add_re = Regex::new(r"new = old \+ (.+)")?;
    let tst_div_re = Regex::new(r"divisible by (\d+)")?;
    let throw_monkey_re = Regex::new(r"throw to monkey (\d+)")?;

    Ok(s.split("\n\n")
        .map(|x| {
            dbg!(&x);
            let mut monkey_data = x
                .lines()
                .skip(1)
                .filter_map(|line| line.split(": ").skip(1).next());
            let items = monkey_data
                .next()
                .unwrap()
                .split(',')
                .map(|itm| itm.trim().parse::<WorryLevel>().unwrap())
                .collect();
            let op = monkey_data
                .next()
                .map(|op| {
                    if op_mul_re.is_match(op) {
                        let arg = op_mul_re.captures(op).unwrap().get(1).unwrap().as_str();
                        if arg == "old" {
                            Box::new(move |x| x * x) as Box<dyn Fn(WorryLevel) -> WorryLevel>
                        } else {
                            let arg = arg.parse::<usize>().unwrap();
                            Box::new(move |x| x * arg)
                        }
                    } else if op_add_re.is_match(op) {
                        let arg = op_add_re.captures(op).unwrap().get(1).unwrap().as_str();
                        if arg == "old" {
                            Box::new(move |x| x + x) as Box<dyn Fn(WorryLevel) -> WorryLevel>
                        } else {
                            let arg = arg.parse::<usize>().unwrap();
                            Box::new(move |x| x + arg)
                        }
                    } else {
                        unreachable!();
                    }
                })
                .unwrap();
            let divisible_by = monkey_data
                .next()
                .map(|txt| {
                    tst_div_re
                        .captures(txt)
                        .expect("assumed all tests are 'divisible by n'")
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap();
            let if_true = monkey_data
                .next()
                .map(|txt| {
                    throw_monkey_re
                        .captures(txt)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap();
            let if_false = monkey_data
                .next()
                .map(|txt| {
                    throw_monkey_re
                        .captures(txt)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap();
            let test = Box::new(move |x: WorryLevel| {
                if x % divisible_by == 0 {
                    if_true
                } else {
                    if_false
                }
            });
            Monkey { items, op, test }
        })
        .collect())
}

fn part_1_level_of_monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    for _round in 0..20 {
        for monkey in monkeys {
            for _ in monkey.items {
                monkey.items.pop();
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MONKEYS: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn it_parses_monkeys() {
        let monkeys = parse_monkeys(TEST_MONKEYS).unwrap();
        dbg!(&monkeys);
        assert_eq!(4, monkeys.len());
    }
}
