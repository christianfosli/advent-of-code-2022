use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Formatter},
    fs,
    sync::Arc,
};

use regex::Regex;

type WorryLevel = usize;

#[derive(Clone)]
struct Monkey {
    items: Vec<WorryLevel>,
    op: Arc<dyn Fn(WorryLevel) -> WorryLevel>,
    test: Arc<dyn Fn(WorryLevel) -> usize>, // returns index of "throw to monkey"
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey, items: {:?}", self.items)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let monkeys = parse_monkeys(&input)?;
    println!("Part 1: {}", part_1_level_of_monkey_business(&monkeys));
    println!("Part 2: {}", part_2_level_of_monkey_business(&monkeys));
    Ok(())
}

fn parse_monkeys(s: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let op_mul_re = Regex::new(r"new = old \* (.+)")?;
    let op_add_re = Regex::new(r"new = old \+ (.+)")?;
    let tst_div_re = Regex::new(r"divisible by (\d+)")?;
    let throw_monkey_re = Regex::new(r"throw to monkey (\d+)")?;

    Ok(s.split("\n\n")
        .map(|x| {
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
                            Arc::new(move |x| x * x) as Arc<dyn Fn(WorryLevel) -> WorryLevel>
                        } else {
                            let arg = arg.parse::<usize>().unwrap();
                            Arc::new(move |x| x * arg)
                        }
                    } else if op_add_re.is_match(op) {
                        let arg = op_add_re.captures(op).unwrap().get(1).unwrap().as_str();
                        if arg == "old" {
                            Arc::new(move |x| x + x) as Arc<dyn Fn(WorryLevel) -> WorryLevel>
                        } else {
                            let arg = arg.parse::<usize>().unwrap();
                            Arc::new(move |x| x + arg)
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
            let test = Arc::new(move |x: WorryLevel| {
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

fn part_1_level_of_monkey_business(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut inspections = HashMap::new();
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let current_monkey = &mut monkeys[i];
                let item = current_monkey.items.pop().unwrap();
                let item = current_monkey.op.clone()(item) / 3;
                let next_monkey = current_monkey.test.clone()(item);
                monkeys[next_monkey].items.push(item);

                *inspections.entry(i).or_insert(0) += 1;
            }
        }
    }
    let most_active = inspections.iter().max_by_key(|&(_key, val)| val).unwrap();
    let second_most_active = inspections
        .iter()
        .filter(|&(key, _val)| key != most_active.0)
        .max_by_key(|&(_key, val)| val)
        .unwrap();
    most_active.1 * second_most_active.1
}

fn part_2_level_of_monkey_business(monkeys: &[Monkey]) -> usize {
    // TODO: Overflows during multiplication
    let mut monkeys = monkeys.to_vec();
    let mut inspections = HashMap::new();
    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let current_monkey = &mut monkeys[i];
                let item = current_monkey.items.pop().unwrap();
                let item = current_monkey.op.clone()(item);
                let next_monkey = current_monkey.test.clone()(item);
                monkeys[next_monkey].items.push(item);

                *inspections.entry(i).or_insert(0) += 1;
            }
        }
    }
    let most_active = inspections.iter().max_by_key(|&(_key, val)| val).unwrap();
    let second_most_active = inspections
        .iter()
        .filter(|&(key, _val)| key != most_active.0)
        .max_by_key(|&(_key, val)| val)
        .unwrap();
    most_active.1 * second_most_active.1
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
